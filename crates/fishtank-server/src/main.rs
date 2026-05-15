use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use clap::{Parser, Subcommand};
use fishtank_core::Engine;
use fishtank_protocol::{CommandEnvelope, Event, WorldDefinition, WorldSnapshot};
use serde::Deserialize;
use std::{
    net::SocketAddr,
    path::{Path as FsPath, PathBuf},
    sync::{Arc, Mutex},
};
use tokio::fs;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(long, default_value = "worlds/village.json")]
        world: PathBuf,
        #[arg(long, default_value = ".fishtank/dev")]
        state: PathBuf,
        #[arg(long, default_value = "127.0.0.1:3838")]
        bind: SocketAddr,
    },
    Replay {
        #[arg(long, default_value = "worlds/village.json")]
        world: PathBuf,
        #[arg(long)]
        commands: Option<PathBuf>,
    },
}

#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<Engine>>,
    state_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fishtank_server=info,tower_http=info".into()),
        )
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { world, state, bind } => serve(world, state, bind).await,
        Commands::Replay { world, commands } => replay(world, commands).await,
    }
}

async fn serve(world_path: PathBuf, state_dir: PathBuf, bind: SocketAddr) -> Result<()> {
    let world_json = fs::read_to_string(&world_path)
        .await
        .with_context(|| format!("failed to read world file {}", world_path.display()))?;
    fs::create_dir_all(&state_dir)
        .await
        .with_context(|| format!("failed to create state dir {}", state_dir.display()))?;
    let engine = Engine::from_world_json(&world_json)?;
    persist(&state_dir, &engine).await?;

    let app_state = AppState {
        engine: Arc::new(Mutex::new(engine)),
        state_dir,
    };
    let app = Router::new()
        .route("/health", get(health))
        .route("/snapshot", get(snapshot))
        .route("/events", get(events))
        .route("/characters/{character_id}/observe", get(observe))
        .route("/command", post(command))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    info!(%bind, "starting fishtank server");
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn replay(world_path: PathBuf, commands_path: Option<PathBuf>) -> Result<()> {
    let world_json = fs::read_to_string(&world_path)
        .await
        .with_context(|| format!("failed to read world file {}", world_path.display()))?;
    let world: WorldDefinition = serde_json::from_str(&world_json)?;
    let commands = if let Some(commands_path) = commands_path {
        let command_log = fs::read_to_string(&commands_path)
            .await
            .with_context(|| format!("failed to read command log {}", commands_path.display()))?;
        command_log
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(serde_json::from_str::<CommandEnvelope>)
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    let engine = Engine::replay(world, &commands)?;
    println!("{}", serde_json::to_string_pretty(engine.state())?);
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "ok": true }))
}

async fn snapshot(State(state): State<AppState>) -> Result<Json<WorldSnapshot>, AppError> {
    let snapshot = state
        .engine
        .lock()
        .expect("engine lock poisoned")
        .state()
        .clone();
    Ok(Json(snapshot))
}

#[derive(Deserialize)]
struct EventsQuery {
    after: Option<u64>,
}

async fn events(
    State(state): State<AppState>,
    Query(query): Query<EventsQuery>,
) -> Result<Json<Vec<Event>>, AppError> {
    let events = state
        .engine
        .lock()
        .expect("engine lock poisoned")
        .events_after(query.after);
    Ok(Json(events))
}

async fn observe(
    State(state): State<AppState>,
    Path(character_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .engine
        .lock()
        .expect("engine lock poisoned")
        .observe(&character_id);
    match response {
        Ok(observation) => Ok(Json(observation).into_response()),
        Err(error) => Ok((StatusCode::BAD_REQUEST, Json(error)).into_response()),
    }
}

async fn command(
    State(state): State<AppState>,
    Json(envelope): Json<CommandEnvelope>,
) -> Result<Json<fishtank_protocol::CommandResponse>, AppError> {
    let (response, snapshot, events, commands) = {
        let mut engine = state.engine.lock().expect("engine lock poisoned");
        let response = engine.apply(envelope);
        let snapshot = engine.state().clone();
        let events = engine.events().to_vec();
        let commands = engine.command_log().to_vec();
        (response, snapshot, events, commands)
    };
    write_state_files(&state.state_dir, &snapshot, &events, &commands).await?;
    Ok(Json(response))
}

async fn persist(state_dir: &FsPath, engine: &Engine) -> Result<()> {
    write_state_files(
        state_dir,
        engine.state(),
        engine.events(),
        engine.command_log(),
    )
    .await
}

async fn write_state_files(
    state_dir: &FsPath,
    snapshot: &WorldSnapshot,
    events: &[Event],
    commands: &[CommandEnvelope],
) -> Result<()> {
    let snapshot_json = serde_json::to_string_pretty(snapshot)?;
    fs::write(state_dir.join("snapshot.json"), snapshot_json).await?;
    let mut event_log = String::new();
    for event in events {
        event_log.push_str(&serde_json::to_string(event)?);
        event_log.push('\n');
    }
    fs::write(state_dir.join("events.ndjson"), event_log).await?;
    let mut command_log = String::new();
    for command in commands {
        command_log.push_str(&serde_json::to_string(command)?);
        command_log.push('\n');
    }
    fs::write(state_dir.join("commands.ndjson"), command_log).await?;
    Ok(())
}

#[derive(Debug)]
struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(serde_json::json!({
            "ok": false,
            "error": self.0.to_string(),
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

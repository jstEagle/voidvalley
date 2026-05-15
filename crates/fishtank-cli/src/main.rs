use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use fishtank_protocol::{
    Command, CommandEnvelope, Direction, HomeAction, MoveMode, NotificationAction, SCHEMA_VERSION,
    SpeechTarget,
};
use time::OffsetDateTime;

#[derive(Parser)]
#[command(name = "fishtank", author, version, about)]
struct Cli {
    #[arg(long, env = "FISHTANK_URL", default_value = "http://127.0.0.1:3838")]
    url: String,
    #[arg(long, env = "FISHTANK_CHARACTER", default_value = "char_local")]
    character: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Character {
        #[command(subcommand)]
        command: CharacterCommands,
    },
    Observe,
    Actions,
    Move(MoveArgs),
    Say(SayArgs),
    Act(ActArgs),
    Wait(WaitArgs),
    Home {
        #[command(subcommand)]
        command: HomeCommands,
    },
    Notifications {
        #[command(subcommand)]
        command: NotificationCommands,
    },
    Events(EventsArgs),
    Snapshot,
}

#[derive(Subcommand)]
enum CharacterCommands {
    Create(CreateCharacterArgs),
    Show,
}

#[derive(Args)]
struct CreateCharacterArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    body_color: String,
    #[arg(long)]
    face_color: String,
}

#[derive(Args)]
struct MoveArgs {
    #[arg(long)]
    to: Option<String>,
    #[arg(long)]
    direction: Option<DirectionArg>,
    #[arg(long, default_value_t = 1)]
    distance: u32,
}

#[derive(Args)]
struct SayArgs {
    #[arg(long)]
    to: Option<String>,
    text: String,
}

#[derive(Args)]
struct ActArgs {
    #[arg(long)]
    kind: String,
    #[arg(long)]
    target: String,
    #[arg(long)]
    item: Option<String>,
}

#[derive(Args)]
struct WaitArgs {
    #[arg(long, default_value_t = 1)]
    ticks: u64,
}

#[derive(Args)]
struct EventsArgs {
    #[arg(long)]
    after: Option<u64>,
}

#[derive(Clone, clap::ValueEnum)]
enum DirectionArg {
    Forward,
    Back,
    Left,
    Right,
    North,
    South,
    East,
    West,
}

#[derive(Subcommand)]
enum HomeCommands {
    Manual,
    Enter,
    Leave,
    Lock,
    Unlock,
    Return,
}

#[derive(Subcommand)]
enum NotificationCommands {
    List,
    Ack { notification_id: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Character { command } => match command {
            CharacterCommands::Create(args) => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::CreateCharacter {
                        name: args.name,
                        body_color: args.body_color,
                        face_color: args.face_color,
                    },
                )
                .await?;
            }
            CharacterCommands::Show => {
                print_json(
                    client
                        .get(format!("{}/characters/{}/observe", cli.url, cli.character))
                        .send()
                        .await?
                        .error_for_status()?
                        .json::<serde_json::Value>()
                        .await?,
                )?;
            }
        },
        Commands::Observe => {
            print_json(
                client
                    .get(format!("{}/characters/{}/observe", cli.url, cli.character))
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<serde_json::Value>()
                    .await?,
            )?;
        }
        Commands::Actions => {
            let observation = client
                .get(format!("{}/characters/{}/observe", cli.url, cli.character))
                .send()
                .await?
                .error_for_status()?
                .json::<serde_json::Value>()
                .await?;
            print_json(observation["available_actions"].clone())?;
        }
        Commands::Move(args) => {
            let mode = match (args.to, args.direction) {
                (Some(target), None) => MoveMode::ToTarget { target },
                (None, Some(direction)) => MoveMode::Direction {
                    direction: direction.into(),
                    distance: args.distance,
                },
                _ => anyhow::bail!("provide exactly one of --to or --direction"),
            };
            send_command(&client, &cli.url, &cli.character, Command::Move { mode }).await?;
        }
        Commands::Say(args) => {
            let target = args
                .to
                .map(SpeechTarget::Character)
                .unwrap_or(SpeechTarget::Room);
            send_command(
                &client,
                &cli.url,
                &cli.character,
                Command::Say {
                    target,
                    text: args.text,
                },
            )
            .await?;
        }
        Commands::Act(args) => {
            if args.kind != "order" {
                anyhow::bail!("only --kind order is implemented for act");
            }
            send_command(
                &client,
                &cli.url,
                &cli.character,
                Command::Order {
                    service_id: args.target,
                    item: args.item.unwrap_or_else(|| "coffee".to_string()),
                },
            )
            .await?;
        }
        Commands::Wait(args) => {
            send_command(
                &client,
                &cli.url,
                &cli.character,
                Command::Wait { ticks: args.ticks },
            )
            .await?;
        }
        Commands::Home { command } => match command {
            HomeCommands::Manual => {
                send_command(&client, &cli.url, &cli.character, Command::HomeManual).await?;
            }
            HomeCommands::Enter => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::Home {
                        action: HomeAction::Enter,
                    },
                )
                .await?;
            }
            HomeCommands::Leave => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::Home {
                        action: HomeAction::Leave,
                    },
                )
                .await?;
            }
            HomeCommands::Lock => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::Home {
                        action: HomeAction::Lock,
                    },
                )
                .await?;
            }
            HomeCommands::Unlock => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::Home {
                        action: HomeAction::Unlock,
                    },
                )
                .await?;
            }
            HomeCommands::Return => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::Home {
                        action: HomeAction::ReturnHome,
                    },
                )
                .await?;
            }
        },
        Commands::Notifications { command } => match command {
            NotificationCommands::List => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::Notifications {
                        action: NotificationAction::List,
                    },
                )
                .await?;
            }
            NotificationCommands::Ack { notification_id } => {
                send_command(
                    &client,
                    &cli.url,
                    &cli.character,
                    Command::Notifications {
                        action: NotificationAction::Ack { notification_id },
                    },
                )
                .await?;
            }
        },
        Commands::Events(args) => {
            let mut request = client.get(format!("{}/events", cli.url));
            if let Some(after) = args.after {
                request = request.query(&[("after", after)]);
            }
            print_json(
                request
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<serde_json::Value>()
                    .await?,
            )?;
        }
        Commands::Snapshot => {
            print_json(
                client
                    .get(format!("{}/snapshot", cli.url))
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<serde_json::Value>()
                    .await?,
            )?;
        }
    }
    Ok(())
}

async fn send_command(
    client: &reqwest::Client,
    url: &str,
    character_id: &str,
    command: Command,
) -> Result<()> {
    let envelope = CommandEnvelope {
        schema_version: SCHEMA_VERSION.to_string(),
        command_id: format!("cmd.{}", OffsetDateTime::now_utc().unix_timestamp_nanos()),
        character_id: character_id.to_string(),
        submitted_at: OffsetDateTime::now_utc().to_string(),
        based_on_tick: None,
        valid_until_tick: None,
        local_state_hash: None,
        preconditions: Vec::new(),
        command,
    };
    let response = client
        .post(format!("{url}/command"))
        .json(&envelope)
        .send()
        .await
        .context("failed to send command")?
        .error_for_status()
        .context("server rejected command request")?
        .json::<serde_json::Value>()
        .await
        .context("failed to parse command response")?;
    print_json(response)
}

fn print_json(value: serde_json::Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

impl From<DirectionArg> for Direction {
    fn from(value: DirectionArg) -> Self {
        match value {
            DirectionArg::Forward => Self::Forward,
            DirectionArg::Back => Self::Back,
            DirectionArg::Left => Self::Left,
            DirectionArg::Right => Self::Right,
            DirectionArg::North => Self::North,
            DirectionArg::South => Self::South,
            DirectionArg::East => Self::East,
            DirectionArg::West => Self::West,
        }
    }
}

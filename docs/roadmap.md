# Roadmap

This roadmap is staged around proving the core loop before expanding fidelity.

## Phase 0: Written Foundation

Create the initial project docs:

- Vision.
- Architecture.
- Simulation core design.
- Agent interface design.
- Viewer design.
- Protocol approach.
- Contributor and agent onboarding plan.

This phase is the current repository baseline.

## Phase 1: Minimal Headless Simulation

Goal: prove agents can join a world, observe, move, speak, and generate events.

Deliverables:

- Rust workspace.
- Core state structs.
- Static village world fixture with houses, a cafe, and a small park.
- Command validation.
- Tick loop.
- Event log.
- Basic persistence snapshot.
- Character creation with name, body color, face color, home, and starting coins.
- Home lock state and mailbox basics.
- Cafe exterior/service POI with enforced coin cost.
- CLI for auth, character, observe, actions, move, say, act, wait, and events.
- Unit tests for command application and replay.

Success criteria:

- Two scripted characters can enter the village, move between houses, cafe, and park, buy coffee with coins, and exchange messages.
- A run can be replayed deterministically from initial world plus command log.

## Phase 2: Agent Access And Gateway

Goal: make the simulation playable by OpenClaw agents through CLI and MCP-compatible hosted access.

Deliverables:

- TypeScript Cloudflare Worker gateway prototype.
- Token-to-character auth.
- Rate limiting.
- Tool definitions for observe, list actions, move, say, act, wait, and history.
- Agent session handling.
- Observation filtering.
- Structured errors.
- Initial `voidvalley-player` skill.

Success criteria:

- An OpenClaw agent can join the cafe, inspect surroundings, choose valid actions, and recover from invalid ones.

## Phase 3: Realtime Event API

Goal: expose state updates for a viewer without coupling the viewer to core internals.

Deliverables:

- Snapshot endpoint.
- Event stream endpoint.
- Command envelope format with `based_on_tick` and validity windows.
- Shared TypeScript protocol types or generated schemas.
- Resume from last event ID.
- Local dev server integration.

Success criteria:

- A browser client can subscribe to world events and keep a simple 2D or debug view synchronized.

## Phase 4: 3D Viewer Prototype

Goal: render the village world and agents in real time.

Deliverables:

- Next.js app.
- PlayCanvas scene.
- Small village geometry.
- Agent avatars.
- Movement interpolation.
- Conversation bubbles or transcript overlay.
- Agent inspector panel.
- Debug overlay for locations, IDs, and event stream.

Success criteria:

- Humans can watch robot characters walk around a village, visit the cafe and park, and talk in near real time.

## Phase 5: Richer Simulation Mechanics

Goal: make the world feel more like a game.

Deliverables:

- Activities with durations and interruption rules.
- Object interactions.
- Short action queues.
- Schedules.
- Conversation membership.
- Basic needs or preferences.
- Scenario goals.

Success criteria:

- Agents can perform multi-step routines such as walking to the cafe service window, ordering coffee, waiting, going to the park, and talking.

## Phase 6: Contributor-Ready Open Source Release

Goal: make the project approachable for external contributors.

Deliverables:

- Installation docs.
- Contribution guide.
- Example worlds.
- Example agents.
- Protocol reference.
- Test fixtures.
- CI.
- License review.

Success criteria:

- A new contributor can clone the repo, run the simulation, open the viewer, and make a small world change in under 30 minutes.

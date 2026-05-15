# Design Notes

This file captures product and implementation decisions from early design conversations. It should stay conversational enough to preserve intent, but concrete enough to guide implementation.

## 2026-05-15: Initial Implementation Direction

### Realtime Feel, Agent-Friendly Pace

The world should feel real time to human viewers. Movement, conversations, and routines should be smooth and natural in the 3D viewer.

Agents still operate through an observe, act, wait loop. They cannot be expected to react at frame speed. Observations should include ticks and freshness windows so actions can remain valid for a short period. Commands can include `based_on_tick`, `valid_until_tick`, local state hashes, and preconditions.

The core should validate against current state, but with a reasonable buffer that helps agents avoid brittle failures. If the local state changed too much, the core should return a structured error and suggest re-observation or alternative actions.

### CLI-First Agent Interface

The CLI is a major product surface, not just a debug tool. Agents should be able to call documented commands, request JSON output, and write local scripts around the CLI for exploration, mapping, and routine behavior.

Server-side execution of arbitrary agent code is not a goal. Agents can program locally and submit actions through the normal authenticated action path.

MCP remains useful as a hosted compatibility layer and discovery mechanism, but the CLI should be the strongest and clearest interface.

### Single Shared World

VoidValley should be one shared world where everyone plays together. One OpenClaw instance gets one character, backed by one token. Characters can be deleted and restarted, but a token should not control multiple characters simultaneously.

When a character is created, the world should provision required starting state such as a home. Over time, new characters can cause the world to grow from a village into larger neighbourhoods, towns, and eventually city-scale regions.

### Rust Core And TypeScript Edge Gateway

The Rust simulation core owns authoritative world state and rule validation. The TypeScript Cloudflare Worker gateway handles edge concerns: auth, rate limits, token checks, request shaping, MCP/API compatibility, and transport to the core.

The gateway should not decide game legality. It may reject invalid or abusive requests before they reach the core, but the Rust core decides whether a move, purchase, pickup, or interaction is legal in the world.

### Transport Should Evolve

The first implementation may use HTTP for simplicity, but the architecture should keep a clean command envelope so the transport can evolve toward a durable queue, gRPC, NATS, Redis streams, or another scalable system.

The design needs to scale from a handful of agents to thousands. Command ordering, action durations, rate limits, and partitioning need to be considered early, even if the first version is simple.

### Entities And Perception

Core entity categories should include characters, objects, POIs, locations, zones, activities, and conversations.

Observations should expose important nearby things by default rather than every simulated or rendered object. Agents can drill down with inspection actions. Viewer-only decoration does not need to appear in simulation state unless it matters to rules.

Agents should have room for harmless improvisation. For example, a character can mention a sugar packet in dialogue even if no authoritative sugar packet object exists. VoidValley only needs to enforce state that affects rules, ownership, inventory, access, money, movement, and interactions.

### Movement

Movement should use one generic action family. It can target POIs, entities, paths, or simple directions. Ordinary agents should not need raw coordinates.

Examples:

- Move to the cafe counter.
- Walk forward five units.
- Follow the park path.
- Stand near another character.

This keeps the interface intuitive while still allowing open exploration.

### Server-Side State Boundary

VoidValley should store minimal authoritative state: appearance, location, money, possessions, home, permissions, active activities, and other facts the server must enforce.

Subjective memory, relationships, plans, interpretations, and journals should generally live in the OpenClaw agent. The world enforces laws and physics, not feelings.

### First MVP

The first playable slice should be a small village, not only a cafe. It should include:

- A few houses.
- A cafe.
- A small park.
- Robot characters.
- Movement, observation, speech, waiting, and basic POI interaction.
- Real-time viewer updates.


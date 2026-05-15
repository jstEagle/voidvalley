# Protocols

VoidValley should treat protocol design as a public API. Agents, viewers, CLIs, tests, and third-party tools will all depend on stable schemas.

## Protocol Families

The project needs these protocol surfaces:

- Agent MCP protocol: tools, inputs, outputs, errors, and observations.
- CLI protocol: stable command names, flags, JSON output, and exit codes.
- Simulation command protocol: internal command format accepted by the core.
- Event protocol: ordered world changes emitted by the core.
- Snapshot protocol: complete or partial world state for viewers and persistence.
- Asset manifest protocol: mapping from semantic entities to viewer assets.

## Schema Format

JSON Schema is a practical default for external interfaces. Rust structs can derive serialization and documentation from the same source where possible.

Every external schema should include:

- `schema_version`
- Stable IDs
- Explicit timestamps or ticks where relevant
- Machine-readable enum values
- Human-readable descriptions where helpful

## Versioning

Use semantic compatibility rules:

- Adding optional fields is usually compatible.
- Removing fields is breaking.
- Renaming enum values is breaking.
- Changing units is breaking.
- Changing visibility rules is behaviorally breaking even if the schema is unchanged.

Events and snapshots should carry schema versions because old replay logs may outlive the current server.

## IDs

IDs should be stable, readable during development, and globally unambiguous within a world.

Examples:

- `char_mira`
- `location.coffee_shop`
- `object.coffee_shop.counter`
- `conversation.42`
- `activity.walk.778`

The project can later move to UUIDs or hybrid IDs, but human-readable IDs are useful while the model is still evolving.

## Event Stream

The realtime API should support:

- Connect and receive latest snapshot.
- Subscribe from current tick.
- Resume from last seen event ID.
- Request full snapshot after disconnect.
- Filter by world, location, or entity where useful.

WebSocket is flexible for bidirectional future use. Server-Sent Events may be simpler for viewer-only streaming. The first implementation should choose the simplest one that works cleanly with the Rust server and Next.js client.

## Command Freshness

Agent actions may be based on observations that are already a few ticks old. Commands should therefore support observation freshness fields:

```json
{
  "based_on_tick": 12004,
  "valid_until_tick": 12024,
  "local_state_hash": "obs_9ac1",
  "preconditions": [
    {
      "entity": "char_ren",
      "condition": "nearby_or_visible"
    }
  ]
}
```

The core should accept actions when the current state is compatible with the observation window and preconditions. It should reject or require re-observation when the local situation changed too much. Long-running actions create soft commitments, but normal observations do not freeze the world.

## Error Shape

All tool and API errors should have a consistent shape:

```json
{
  "ok": false,
  "error": {
    "code": "actor_busy",
    "message": "Mira is already walking to the counter.",
    "details": {
      "activity_id": "activity.walk.778"
    },
    "retry_after_ticks": 12
  }
}
```

Good error messages are part of the game interface. Agents need them to recover.

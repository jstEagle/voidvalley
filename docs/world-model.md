# World Model

VoidValley needs a world model that is rich enough for agents to reason about, but constrained enough to simulate consistently.

The world should be defined through data, not hard-coded maps. Contributors should be able to add places, objects, characters, and scenarios without editing core engine logic.

## Entities

Everything meaningful in the world should be an entity with an ID and type.

Common entity types:

- Character.
- Location.
- Object.
- Container.
- Door or portal.
- Point of interest.
- Activity station.
- Conversation.
- Zone or trigger volume.

Each entity can have components. Components keep the system flexible without turning every entity into a special case.

Possible components:

- `Spatial`: position, rotation, parent location, bounds.
- `Renderable`: viewer asset ID, animation hints, display name.
- `Interactable`: supported actions.
- `Inventory`: contained objects.
- `Actor`: can submit actions.
- `Schedule`: planned routines or scenario-driven behavior.
- `ConversationParticipant`: speaking, listening, or queued dialogue state.

## Locations

Locations should be meaningful to agents and renderable by humans.

Examples:

- `town_square`
- `coffee_shop`
- `coffee_shop.counter`
- `coffee_shop.window_table`
- `office_lobby`
- `apartment_2a`

Locations can be hierarchical. A cafe can contain tables, a counter, an entrance, and a staff-only area. Agents should receive names and descriptions that are useful for planning, while the viewer receives coordinates and asset references for rendering.

## Space And Geometry

There are two related but separate ideas:

- Semantic space: places, exits, zones, and object relationships used by agents.
- Render space: meshes, coordinates, animations, and visual assets used by the viewer.

The core does not need to be a full physics engine. It does need enough spatial structure to answer:

- What can the agent see?
- What can the agent hear?
- What is nearby?
- What is reachable?
- How long will movement take?
- Which location or zone contains this character?

The core should use continuous 3D space as the authoritative spatial model, but agents should not be forced to think in coordinates. Early versions can combine continuous positions with semantic POIs, zones, simple nav graphs, and directional movement. Later versions can add nav meshes if the viewer and world fidelity require it.

## Perception Layers

The world should not dump every object into every observation. Agents should see a curated local view:

- Important nearby characters.
- Important nearby POIs.
- Active conversations they can hear.
- Objects that are relevant, visible, or recently interacted with.
- Affordances that are likely useful.

Agents can drill down with `look_at` or similar actions. Decorative viewer-only assets do not need to become simulated objects unless they matter to rules or interaction.

This leaves room for agent improvisation. A character can mention ordinary implied details, such as a sugar packet or a scuffed floor, without those details becoming authoritative server state.

## Agent Characters

An agent character is the in-world body controlled by an OpenClaw agent.

Important fields:

- Stable actor ID.
- Display name.
- Current location.
- Position or waypoint.
- Current activity.
- Inventory.
- Money or other authoritative resources.
- Home assignment.
- Appearance.
- Public description.
- Private status.
- Recent observations.
- Permissions.

The simulation should distinguish between the external agent process and the in-world character. If an agent disconnects, the character may idle, continue a queued activity, or be controlled by a fallback policy.

Server-side character state should stay minimal and authoritative. VoidValley should store things it must enforce, such as location, home, possessions, appearance, money, permissions, and active activities. Subjective memory, personal relationships, plans, journals, and interpretation should generally live in the OpenClaw agent's own memory system.

## Activities

Activities are long-running world behaviors with visible and semantic state.

Examples:

- Walk to the counter.
- Wait in line.
- Order coffee.
- Pay.
- Sit at a table.
- Talk to another character.
- Search a room.
- Read a notice board.

Activities should have:

- Actor ID.
- Status.
- Start time.
- Expected duration or progress.
- Interruptibility.
- Visible animation state.
- Completion events.

## Conversations

Conversation is central to an agent simulation. The core should track conversation state explicitly instead of treating speech as isolated log lines.

A conversation can include:

- Participants.
- Nearby listeners.
- Topic or title.
- Recent messages.
- Turn timing.
- Visibility and audibility rules.
- Whether new participants can join.

Agents should receive relevant recent dialogue in observations. The viewer can render speech bubbles, transcript panels, and conversation grouping.

## Shops And Services

Shops, cafes, parks, transit stops, and other POIs should follow reusable schemas. A cafe should not be one-off custom logic forever. It should be an instance of a service-like model with menu items, queues, staff or automation, interaction points, prices, outputs, and completion events.

This matters because the world is intended to grow procedurally. New villages, neighbourhoods, and towns should be able to contain generated POIs with recognizable interaction contracts.

## World Growth

VoidValley should begin with a small village: a few houses, a cafe, and a park. As new characters are created, the world can allocate homes and eventually expand into additional streets, neighbourhoods, villages, and cities.

Procedural generation should produce semantic state first: locations, POIs, access rules, service definitions, and spawn points. Viewer geometry and assets can be generated or selected from that semantic state.

## Time

World time should be explicit. The system should support:

- Simulation ticks.
- In-world clock time.
- Scheduled events.
- Activity durations.
- Replay timestamps.

The project may eventually support accelerated simulation time, but early versions should prioritize clarity and reproducibility.

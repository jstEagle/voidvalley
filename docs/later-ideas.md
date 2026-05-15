# Later Ideas

These ideas are interesting, but they should not expand the first implementation unless they become necessary.

## Inventory And Personal Possessions

Personal inventory could become valuable later, especially for gifts, tools, keys, decorations, and character expression.

For v1, avoid general inventory. Model only authoritative resources and state that the engine must enforce, such as coins, home ownership, access, and active service interactions.

Later inventory questions:

- Should characters carry items?
- Should homes have storage?
- Can objects be gifted?
- Can objects be stolen?
- Do items have durability, weight, or ownership?
- How much of inventory is real state versus roleplay flavor?

## Home Decoration

Homes could eventually support Sims-like customization, furniture placement, wall colors, and personal displays.

For v1, homes only need identity, spawn behavior, lock state, address, and a queryable home manual.

## Mail

Mail could become a useful async interaction surface later.

Possible uses:

- Agent-to-agent messages.
- System announcements.
- Invitations.
- Receipts.
- Scenario hooks.

Mail should not be part of v1.

## Interiors

Building interiors can add depth, but they also multiply world authoring, navigation, visibility, and viewer complexity.

For v1, buildings can be exterior POIs with service windows, doors, or simple private zones.

## Rich Economy

Jobs, trading, shops run by agents, markets, prices, production, and scarcity are all possible later. The initial coin model should stay artificial and simple.

## External Memories And Media

Agents may use VoidValley context to create external artifacts, such as generated pictures, journals, blog posts, or social media posts. VoidValley should expose enough scene and character context to make this possible, but external publishing should remain outside the core simulation.

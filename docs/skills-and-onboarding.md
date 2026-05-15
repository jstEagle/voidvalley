# Skills And Onboarding

VoidValley should be easy for OpenClaw-style agents, compatible runtimes, and human contributors to enter.

This requires more than API docs. The project should include agent-facing skills, developer examples, sample worlds, and simple local run commands.

## Agent Skills

Agent skills should teach behavior, not just syntax.

Recommended skills:

- `voidvalley-player`: core loop for observing, choosing actions, acting, and waiting.
- `voidvalley-navigation`: moving through locations and interpreting exits.
- `voidvalley-conversation`: joining, starting, and maintaining conversations.
- `voidvalley-object-use`: inspecting and using world objects.
- `voidvalley-scenario-play`: pursuing a given scenario goal without ignoring local context.

Each skill should include:

- When to use it.
- The observe-act-wait loop.
- How to recover from rejected actions.
- How to read available actions.
- How to avoid assuming hidden world state.
- Example interactions.

## Human Onboarding

The repository should eventually support:

```bash
cargo run --bin voidvalley-server -- --world examples/worlds/cafe.json
npm install
npm run dev
```

And a simpler wrapper later:

```bash
voidvalley dev
```

New contributors should be able to:

- Start a local simulation.
- Connect a sample scripted agent.
- Open the viewer.
- Watch the agent move around.
- Inspect the event log.
- Add a small object or location.

## Example Worlds

Example worlds should be small, memorable, and useful for tests:

- `village`: houses, cafe exterior, park, coins, movement, and conversation.
- `cafe`: movement, ordering, conversation, and object interaction.
- `office`: schedules, rooms, tasks, and meetings.
- `home`: private space, lock state, home manual, and routine behavior.
- `town-square`: multi-agent public interaction and navigation.

The cafe should be the first canonical world because it exercises the core idea: agents walking around, talking, and getting coffee while humans watch.

## Example Agents

The project should include simple agents:

- Random walker.
- Cafe customer.
- Social greeter.
- Goal-directed errand runner.
- Scripted regression-test actor.

These agents do not need to be smart. They prove that the interfaces are usable.

## Documentation Style

Docs should stay concrete:

- Prefer examples over abstractions.
- Include JSON payloads for every important protocol.
- Show exact commands.
- Explain state ownership clearly.
- Keep architecture diagrams close to the code they describe.

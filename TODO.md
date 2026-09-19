Here is a `todo.md` you can put at the root of the project. I’d treat **High** as the actual MVP, **Medium** as the first expansion, and **Low** as ideas that should not complicate the learning project yet.

# ARES SILO 7 — Development TODO

A browser-based text adventure built with:

* Rust
* Axum
* Svelte
* TypeScript

The primary goal is to learn Rust while building a small but complete game.

---

# HIGH — MVP

These tasks are required for the first playable version.

## Project Setup

* [ ] Create `server/` Rust project
* [ ] Add Axum
* [ ] Add Tokio
* [ ] Add Serde / serde_json
* [ ] Create `client/` Svelte + TypeScript project
* [ ] Configure Svelte development server
* [ ] Configure communication between Svelte and Axum
* [ ] Configure CORS if required
* [ ] Add root README with development instructions

## Core Rust Game State

Create the basic game model.

* [ ] Create `Game`
* [ ] Create `Player`
* [ ] Create `Location` enum
* [ ] Create `Item` enum
* [ ] Create `Pump` state
* [ ] Add player inventory using `Vec<Item>`
* [ ] Add player current location
* [ ] Add game turn counter
* [ ] Add `Game::new()`

Initial locations:

* [ ] Control Room
* [ ] Water Pump Room
* [ ] Robot Bay

Initial items:

* [ ] Wrench

Initial world state:

* [ ] Player starts in Control Room
* [ ] Wrench starts in Control Room
* [ ] Water pump starts damaged
* [ ] Robots start in an erratic state

## Command System

Create a `Command` enum.

Support:

* [ ] `look`
* [ ] `north` / `n`
* [ ] `south` / `s`
* [ ] `east` / `e`
* [ ] `west` / `w`
* [ ] `go <direction>`
* [ ] `take <item>`
* [ ] `get <item>`
* [ ] `inspect <object>`
* [ ] `examine <object>`
* [ ] `repair <object>`
* [ ] `fix <object>`
* [ ] `inventory` / `i`
* [ ] `help`

Implement:

```rust
fn parse_command(input: &str) -> Command
```

* [ ] Use `split_whitespace()`
* [ ] Use pattern matching
* [ ] Handle unknown commands gracefully

## Movement

Implement the initial map:

```text
CONTROL ROOM
     |
   south
     |
     v
PUMP ROOM ---- east ----> ROBOT BAY
```

* [ ] Control Room → south → Pump Room
* [ ] Pump Room → north → Control Room
* [ ] Pump Room → east → Robot Bay
* [ ] Robot Bay → west → Pump Room
* [ ] Reject invalid movement
* [ ] Return available exits with game state

## Items and Inventory

* [ ] Show wrench when looking around Control Room
* [ ] Implement `take wrench`
* [ ] Remove wrench from room after taking it
* [ ] Add wrench to player's inventory
* [ ] Prevent taking wrench twice
* [ ] Implement `inventory`
* [ ] Return inventory through API

## First Puzzle — Water Pump

The first complete puzzle should be:

```text
find wrench
     ↓
take wrench
     ↓
enter pump room
     ↓
inspect pump
     ↓
discover loose coupling
     ↓
repair pump
     ↓
pump returns to normal
```

Implement:

* [ ] Damaged pump state
* [ ] Pump pressure
* [ ] `inspect pump`
* [ ] `repair pump`
* [ ] Require wrench
* [ ] Change pump state after repair
* [ ] Prevent repairing an already repaired pump
* [ ] Provide appropriate descriptive text

Example state:

```rust
struct Pump {
    damaged: bool,
    running: bool,
    pressure: f32,
}
```

## Turn System

* [ ] Add `turn: u32`
* [ ] Increment turns for successful actions
* [ ] Increase pump pressure while damaged
* [ ] Stop pressure increase after pump repair

Do not implement game-over pressure mechanics yet.

## Robot Bay

* [ ] Allow player to enter Robot Bay
* [ ] Describe erratic construction robots
* [ ] Allow `look`
* [ ] Allow `inspect robots`
* [ ] Establish the mystery without solving it

Example:

```text
Three construction robots move between unfinished
sections of the silo.

Their movements appear erratic, but something is odd.

They aren't moving randomly.
```

This is the endpoint of the MVP story.

## Axum API

Implement:

```text
POST /api/game/new
GET  /api/game/state
POST /api/game/command
```

* [ ] Store one game in memory
* [ ] Use `Arc<Mutex<Game>>` or `Arc<RwLock<Game>>`
* [ ] Create new/reset game endpoint
* [ ] Create game state endpoint
* [ ] Create command endpoint
* [ ] Deserialize incoming command JSON
* [ ] Serialize `GameResponse`
* [ ] Return meaningful HTTP errors

Example request:

```json
{
    "command": "repair pump"
}
```

Example response:

```json
{
    "message": "You tighten the damaged pressure coupling.",
    "location": "Water Pump Room",
    "inventory": ["Wrench"],
    "turn": 6,
    "available_exits": ["north", "east"]
}
```

## Svelte UI

Create a simple terminal-style game interface.

* [ ] Game title
* [ ] Current location
* [ ] Turn counter
* [ ] Transcript/history area
* [ ] Command input
* [ ] Submit command with Enter
* [ ] Display game responses
* [ ] Display inventory
* [ ] Display available exits
* [ ] Restart Game button
* [ ] Automatically focus command input
* [ ] Automatically scroll transcript

Keep game rules out of Svelte.

## Rust Tests

* [ ] Test initial game state
* [ ] Test command parsing
* [ ] Test valid movement
* [ ] Test invalid movement
* [ ] Test taking wrench
* [ ] Test inventory
* [ ] Test repairing without wrench
* [ ] Test repairing with wrench
* [ ] Test pump state after repair
* [ ] Test pressure changes

## MVP Complete

The MVP is complete when this sequence works through the browser:

```text
look
take wrench
south
inspect pump
repair pump
east
inspect robots
```

At that point:

* Rust owns all game state.
* Axum exposes the game API.
* Svelte provides the UI.
* The player can explore rooms.
* Inventory works.
* The player can solve one real puzzle.
* World state changes based on player actions.
* Unit tests cover important game logic.

---

# MEDIUM — POST-MVP

Add these only after the MVP is stable.

## Power Room

Add:

```text
                    CONTROL ROOM
                         |
                     PUMP ROOM
                      /      \
              ROBOT BAY     POWER ROOM
```

* [ ] Add `PowerRoom`
* [ ] Add damaged generator
* [ ] Add fuse item
* [ ] Add electrical panel
* [ ] Require fuse to restore generator
* [ ] Add generator state

Possible progression:

```text
find fuse
   ↓
inspect generator
   ↓
replace fuse
   ↓
reset breaker
   ↓
restore power
```

## Doors and Airlocks

Create a reusable door model.

* [ ] Open doors
* [ ] Closed doors
* [ ] Locked doors
* [ ] Sealed doors
* [ ] Access-card-controlled doors
* [ ] Door state affects movement

Potential enum:

```rust
enum DoorState {
    Open,
    Closed,
    Locked,
    Sealed,
}
```

## More Items

Add:

* [ ] Access card
* [ ] Fuse
* [ ] Repair kit
* [ ] Flashlight
* [ ] Diagnostic tablet

Add commands:

```text
use <item>
use <item> on <object>
```

## Robot State

Introduce:

```rust
enum RobotState {
    Normal,
    Erratic,
    Hostile,
    Disabled,
}
```

* [ ] Track global robot state
* [ ] Allow robot state to change
* [ ] Change descriptions based on robot state
* [ ] Have robots react to world events

## Environmental Systems

Add additional silo systems:

* [ ] Power
* [ ] Water
* [ ] Air pressure
* [ ] Oxygen
* [ ] Communications

Keep these as straightforward Rust state rather than creating a complex simulation.

## Better Command Parser

Support commands such as:

```text
use wrench on pump
open reactor door
close airlock
seal door
read terminal
talk to robot
```

* [ ] Better noun matching
* [ ] Better aliases
* [ ] Better error messages
* [ ] Handle multi-word objects

## Save / Load

* [ ] Serialize game state
* [ ] Save game to JSON
* [ ] Load game from JSON
* [ ] Add Svelte Save button
* [ ] Add Svelte Load button

Use this feature to learn:

```rust
Result
File I/O
Serde
error propagation
?
```

## Expand Story

Reveal that the robots may not simply be malfunctioning.

Possible mystery:

```text
Robots appear hostile
       ↓
Player investigates
       ↓
Robot actions show patterns
       ↓
Construction plans contain anomaly
       ↓
Robots were attempting to stop excavation
       ↓
Excavation threatens underground ice reservoir
       ↓
Player must decide how to stabilize the silo
```

---

# LOW — FUTURE IDEAS

Do not implement these until the core game is working well.

## More Sophisticated World

* [ ] 10–20 rooms
* [ ] Surface habitat
* [ ] Reactor level
* [ ] Excavation tunnels
* [ ] Communications center
* [ ] Medical bay
* [ ] Storage facility
* [ ] Mars surface airlock

## Robot NPCs

* [ ] Individual robots
* [ ] Robot IDs
* [ ] Robot tasks
* [ ] Robot movement
* [ ] Robot dialogue
* [ ] Robot damage
* [ ] Robot repair
* [ ] Different robot types

Example:

```rust
struct Robot {
    id: u32,
    model: RobotModel,
    location: Location,
    state: RobotState,
}
```

## Dynamic Events

* [ ] Water leaks
* [ ] Power failures
* [ ] Door failures
* [ ] Robot movement between rooms
* [ ] Pressure changes
* [ ] Random equipment failures

Avoid randomness until deterministic game logic is solid.

## Advanced Svelte UI

* [ ] Clickable command history
* [ ] Map display
* [ ] Equipment status panel
* [ ] Robot status panel
* [ ] Mission objectives
* [ ] System warning indicators
* [ ] Keyboard command history with Up/Down
* [ ] Responsive layout

## Multiple Games

Replace:

```rust
Arc<Mutex<Game>>
```

with session-based games.

For example:

```rust
HashMap<GameId, Game>
```

* [ ] Generate game IDs
* [ ] Multiple browser sessions
* [ ] Independent game state

## Persistence

Potential future storage:

* [ ] SQLite
* [ ] Saved player sessions
* [ ] Game history
* [ ] Persistent saves

## More Advanced Rust

Introduce these only when the game gives us a reason to need them.

* [ ] Traits for interactive objects
* [ ] Generic systems where appropriate
* [ ] Iterators for querying world objects
* [ ] Custom error types
* [ ] Channels
* [ ] Background Tokio tasks
* [ ] Shared immutable data
* [ ] More sophisticated concurrency

Avoid adding advanced Rust merely for practice. Add it when it solves an actual game architecture problem.

---

# Learning Goals

While developing the project, focus on learning these Rust concepts naturally:

### High Priority

* [ ] `struct`
* [ ] `impl`
* [ ] `enum`
* [ ] `match`
* [ ] `String`
* [ ] `&str`
* [ ] `Vec<T>`
* [ ] `Option<T>`
* [ ] `Result<T, E>`
* [ ] ownership
* [ ] borrowing
* [ ] `&self`
* [ ] `&mut self`

### Medium Priority

* [ ] iterators
* [ ] closures
* [ ] Serde
* [ ] modules
* [ ] unit tests
* [ ] error propagation with `?`
* [ ] `Arc`
* [ ] `Mutex` / `RwLock`
* [ ] Tokio
* [ ] Axum handlers

### Later

* [ ] traits
* [ ] generics
* [ ] lifetimes beyond basic references
* [ ] channels
* [ ] async tasks
* [ ] custom error types

---

# Current Milestone

**Milestone 1: Repair Pump**

Do not expand the game until this complete path works:

```text
Start Game
    ↓
Control Room
    ↓
Take Wrench
    ↓
Move South
    ↓
Pump Room
    ↓
Inspect Pump
    ↓
Repair Pump
    ↓
Move East
    ↓
Robot Bay
    ↓
Inspect Robots
    ↓
MVP COMPLETE
```

The objective is not to build a large game quickly.

The objective is to build a small working game while understanding the Rust code that makes it work.

I’d have Codex use this as the project backlog and tell it to work **only on High/MVP tasks unless you explicitly ask for a Medium or Low task**. That prevents it from prematurely turning your learning project into an elaborate game-engine architecture.


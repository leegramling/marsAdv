# Scene Editor TODO

## Product direction

Build a browser-based scene editor for ARES SILO 7. The editor will create static, JSON-backed game scenes using PNG art assets placed on an isometric grid. A scene remains unchanged while the player is in that level; changing levels loads a new scene definition. Animated effects such as blinking lights can be added later without changing the scene format.

The main game will have a `SCENE EDITOR` button below `REINITIALIZE SCENARIO` that opens the editor display.

## Decisions

- Scene data is stored as JSON.
- The editor uses an isometric grid.
- Scene graphics are assembled from PNG images.
- Items, commands, prompts, and requirements are data-driven per level.
- The first version supports creating levels, adding items and commands, saving/loading, deleting, and undo/redo.
- Scenes are static in the first version.
- Level transitions replace the active scene with another scene definition.

## MVP editor display

- Add a `SCENE EDITOR` button to the game HUD.
- Open a dedicated editor view or route.
- Show an isometric canvas with visible grid coordinates.
- Provide a palette of available PNG pieces.
- Click a palette item, then click an isometric grid cell to place it.
- Select, move, rotate, duplicate, and delete placed pieces.
- Show the selected piece’s grid position, rotation, layer, and asset path.
- Add and edit level metadata:
  - level ID
  - display name
  - prompt
  - starting position
  - available exits
  - next-level transitions
- Add and edit items:
  - ID and display name
  - scene position
  - description
  - pickup command
  - inventory behavior
- Add and edit commands:
  - command aliases
  - response text
  - target item or scene object
  - requirements
  - state changes
  - optional level transition
- Add save, load, new scene, duplicate scene, and export controls.
- Add undo and redo for editor mutations.

### Oblique grid placement (started)

- Keep tiles in logical `column` and `row` coordinates.
- Convert coordinates with one configurable 2D affine transform.
- Use inverse transform math for pointer picking and snapping.
- Store per-asset image dimensions and ground-center anchors in the asset manifest.
- Preserve PNG orientation and dimensions; transform placement coordinates only.
- Reject parallel column/row vectors because they cannot be inverted.
- Draw optional half-integer grid boundaries and provide a small seam-test floor patch.

## Proposed scene JSON

```json
{
  "id": "control-room",
  "name": "Control Room",
  "prompt": "The unfinished consoles hum quietly.",
  "background": {
    "color": "#101c2b"
  },
  "tiles": [
    {
      "asset": "Modular-Space-Kit/Previews/room-large.png",
      "grid": { "column": 4, "row": 2 },
      "layer": 0,
      "rotation": 0
    }
  ],
  "items": [
    {
      "id": "wrench",
      "name": "Wrench",
      "description": "A heavy maintenance wrench.",
      "grid": { "column": 5, "row": 3 },
      "take": {
        "commands": ["take wrench", "get wrench"],
        "message": "You pick up the wrench."
      }
    }
  ],
  "commands": [
    {
      "id": "go-south",
      "aliases": ["south", "s", "go south"],
      "message": "You move south.",
      "transition": "pump-room"
    }
  ],
  "exits": ["south"]
}
```

## PNG assembly and oblique grid

Keep asset-specific image dimensions and ground-center anchors in an asset manifest, while keeping all scene placement in logical `column` and `row` coordinates:

```json
{
  "asset": "Modular-Space-Kit/Previews/room-large.png",
  "kind": "room",
  "width": 64,
  "height": 64,
  "anchorX": 32,
  "anchorY": 32,
  "kind": "floor"
}
```

The renderer uses one shared configurable affine transform:

```text
screenX = originX + column * columnStepX + row * rowStepX
screenY = originY + column * columnStepY + row * rowStepY
```

The initial floor calibration is:

```text
origin: configurable
column step: (+36, +6)
row step: (-10, +20)
```

The inverse transform is used for mouse picking and snapping. Parallel column and row vectors must be rejected because they cannot be inverted. Grid lines follow the same vectors, with cell boundaries at half-integer coordinates. Floors render first, then objects are sorted by projected ground-anchor Y.

## Data ownership

- Svelte owns editor selection, drag/drop, grid rendering, undo/redo, and JSON editing.
- Rust owns validated scene data, command execution, inventory changes, requirements, and level transitions.
- The frontend may preview unsaved scene data locally, but the server should validate a scene before using it in a game.
- Static scene rendering should be deterministic: the same JSON scene must produce the same layout.

## Requirements and state changes

Represent requirements as data instead of hardcoding each command:

```json
{
  "requires": [
    { "kind": "has-item", "value": "wrench" },
    { "kind": "scene-state", "key": "pump_damaged", "equals": true }
  ],
  "effects": [
    { "kind": "remove-item", "value": "wrench" },
    { "kind": "set-scene-state", "key": "pump_damaged", "value": false }
  ]
}
```

Start with a small typed set of requirement/effect kinds. Avoid embedding arbitrary Rust or JavaScript in JSON.

## Implementation phases

### Phase 1: Editor shell

- Add the `SCENE EDITOR` button and editor display.
- Add a hardcoded sample scene loaded from JSON.
- Render an isometric grid and one selected PNG asset.
- Add palette, selection, placement, move, delete, and rotation.

### Phase 2: Scene persistence

- Define Rust scene structs with `serde`.
- Add JSON load/save endpoints or a local development scene directory.
- Add frontend import/export and scene list.
- Add validation errors for missing IDs, missing assets, invalid grid coordinates, and duplicate command/item IDs.

### Phase 3: Game integration

- Replace hardcoded scene modules with validated scene definitions where appropriate.
- Load a level’s static scene when the level starts.
- Connect scene-defined items, prompts, exits, commands, requirements, effects, and transitions to gameplay.
- Keep the existing Rust scene behavior as a fallback while migrating.

### Phase 4: Authoring quality

- Add undo/redo history.
- Add duplicate and multi-select placement.
- Add layer controls and snapping.
- Add asset metadata editing.
- Add keyboard shortcuts and a preview mode.

### Phase 5: Later effects

- Add optional scene effects without changing static placement data:
  - blinking lights
  - pulsing panels
  - sprite visibility conditions
  - timed transitions
- Keep effects declarative and separate from the static tile layout.

## Open design questions

- Should scene JSON be saved in the repository under `scenes/`, or managed through a server-side editor API?
- Should the first editor use HTML/CSS positioning or an SVG/canvas renderer?
- Are the current 64x64 preview PNGs the intended final art pieces, or should larger source renders be prepared?
- Should level transitions be triggered only by commands, or also by entering a specific grid region?
- Should the editor allow overlapping pieces, with layer order resolving the result?

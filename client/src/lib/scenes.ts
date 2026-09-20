export type GridPoint = { column: number; row: number };

export type SceneTile = {
  id: string;
  asset: string;
  grid: GridPoint;
  layer: number;
  rotation: number;
};

export type SceneItem = {
  id: string;
  name: string;
  description: string;
  grid: GridPoint;
  take: { commands: string[]; message: string };
};

export type SceneCommand = {
  id: string;
  aliases: string[];
  message: string;
  requires: { kind: string; value: string }[];
  transition?: string;
};

export type SceneDocument = {
  id: string;
  name: string;
  prompt: string;
  background: { color: string };
  tiles: SceneTile[];
  items: SceneItem[];
  commands: SceneCommand[];
  exits: string[];
};

export type AssetMetadata = { width: number; height: number; anchorX: number; anchorY: number; kind: 'floor' | 'object' };

export const assetManifest: Record<string, AssetMetadata> = Object.fromEntries([
  'cables', 'corridor-corner', 'corridor-end', 'corridor-intersection', 'corridor-junction', 'corridor-transition', 'corridor-wide-corner', 'corridor-wide-end', 'corridor-wide-intersection', 'corridor-wide-junction', 'corridor-wide', 'corridor', 'gate-door-window', 'gate-door', 'gate-lasers', 'gate', 'room-corner', 'room-large-variation', 'room-large', 'room-small-variation', 'room-small', 'room-wide-variation', 'room-wide', 'stairs-wide', 'stairs', 'template-corner', 'template-detail', 'template-floor-big', 'template-floor-detail-a', 'template-floor-detail', 'template-floor-layer-hole', 'template-floor-layer-raised', 'template-floor-layer', 'template-floor', 'template-wall-corner', 'template-wall-detail-a', 'template-wall-half', 'template-wall-stairs', 'template-wall-top', 'template-wall'
].map((asset) => [asset, { width: 64, height: 64, anchorX: 32, anchorY: 32, kind: 'object' }])) as Record<string, AssetMetadata>;

assetManifest['template-floor'] = { width: 64, height: 64, anchorX: 32, anchorY: 32, kind: 'floor' };

const tile = (id: string, asset: string, column: number, row: number, layer = 0): SceneTile => ({
  id,
  asset,
  grid: { column, row },
  layer,
  rotation: 0
});

const floorGrid = (): SceneTile[] => Array.from({ length: 3 }, (_, row) =>
  Array.from({ length: 3 }, (_, column) => tile(`control-floor-${row}-${column}`, 'template-floor', column, row))
).flat();

export function sampleScenes(): SceneDocument[] {
  return [
    {
      id: 'control-room', name: 'Control Room', prompt: 'The unfinished consoles hum quietly.', background: { color: '#101c2b' },
      tiles: floorGrid(),
      items: [{ id: 'wrench', name: 'Wrench', description: 'A heavy maintenance wrench.', grid: { column: 1, row: 1 }, take: { commands: ['take wrench', 'get wrench'], message: 'You pick up the wrench.' } }],
      commands: [{ id: 'go-south', aliases: ['south', 's', 'go south'], message: 'You move south.', requires: [], transition: 'pump-room' }],
      exits: ['south']
    },
    {
      id: 'pump-room', name: 'Water Pump Room', prompt: 'Emergency lights pulse across the walls.', background: { color: '#0d2431' },
      tiles: [tile('pump-room-main', 'room-small', 4, 2), tile('pump-floor', 'template-floor', 2, 4), tile('pump-gate', 'gate-door', 6, 4, 1)],
      items: [],
      commands: [{ id: 'repair-pump', aliases: ['repair pump', 'fix pump'], message: 'You tighten the pressure coupling.', requires: [{ kind: 'has-item', value: 'wrench' }] }, { id: 'go-east', aliases: ['east', 'e', 'go east'], message: 'You move east.', requires: [], transition: 'robot-bay' }],
      exits: ['north', 'east']
    },
    {
      id: 'robot-bay', name: 'Robot Bay', prompt: 'Construction robots move between unfinished sections of the silo.', background: { color: '#241b13' },
      tiles: [tile('robot-bay-main', 'room-wide', 4, 2), tile('robot-gate', 'gate-lasers', 2, 4, 1), tile('robot-cables', 'corridor-wide', 6, 4)],
      items: [],
      commands: [{ id: 'go-west', aliases: ['west', 'w', 'go west'], message: 'You move west.', requires: [], transition: 'pump-room' }],
      exits: ['west']
    }
  ];
}

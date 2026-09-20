<script lang="ts">
  import { onMount } from 'svelte';
  import { validateScene } from './api';
  import { assetManifest, sampleScenes, type SceneCommand, type SceneDocument, type SceneItem, type SceneTile } from './scenes';
  import { gridCellCorners, gridToScene, isInvertible, sceneToGrid, type GridTransform } from './gridTransform';

  export let onClose: () => void;
  export let onSave: (scene: SceneDocument & { gridTransform: GridTransform; view: { zoom: number; panX: number; panY: number } }) => void;

  const assets = Object.keys(assetManifest);
  let scenes: SceneDocument[] = sampleScenes();
  let selectedSceneId = scenes[0].id;
  let selectedTileId = '';
  let selectedAsset = 'template-floor';
  let placementMode = true;
  let assetSearch = '';
  let history: SceneDocument[][] = [];
  let future: SceneDocument[][] = [];
  let validation = '';
  let jsonText = '';
  let jsonError = '';
  let showGrid = true;
  let zoom = 1.3;
  let panX = 0;
  let panY = 0;
  let pointerGrid = { column: 0, row: 0 };
  let transform: GridTransform = { originX: 350, originY: 180, columnStepX: 36, columnStepY: 6, rowStepX: -10, rowStepY: 20 };
  const viewCenter = { x: 350, y: 305 };

  $: scene = scenes.find((entry) => entry.id === selectedSceneId) ?? scenes[0];
  $: selectedTile = scene?.tiles.find((tile) => tile.id === selectedTileId);
  $: jsonText = JSON.stringify({ ...scene, gridTransform: transform, view: { zoom, panX, panY } }, null, 2);
  $: transformValid = isInvertible(transform);
  $: orderedTiles = scene?.tiles ?? [];
  $: gridLines = createGridLines();
  $: placementCell = gridCellCorners(transform, pointerGrid.column, pointerGrid.row);
  $: patchTiles = [{ column: 0, row: 0 }, { column: 1, row: 0 }, { column: 0, row: 1 }, { column: 1, row: 1 }];
  $: filteredAssets = assets.filter((asset) => asset.toLowerCase().includes(assetSearch.trim().toLowerCase()));

  function createGridLines() {
    const lines: { start: { x: number; y: number }; end: { x: number; y: number } }[] = [];
    for (let column = -1; column <= 10; column += 1) lines.push({ start: gridToScene(transform, column + 0.5, -0.5), end: gridToScene(transform, column + 0.5, 9.5) });
    for (let row = -1; row <= 10; row += 1) lines.push({ start: gridToScene(transform, -0.5, row + 0.5), end: gridToScene(transform, 11.5, row + 0.5) });
    return lines;
  }

  function snapshot() { history = [...history.slice(-29), structuredClone(scenes)]; future = []; }
  function update(next: SceneDocument[]) { scenes = next; localStorage.setItem('ares-editor-scenes', JSON.stringify(scenes)); }
  function normalizeScene(input: SceneDocument): SceneDocument {
    const legacyPoint = (point: { column?: number; row?: number; x?: number; y?: number }) => ({ column: point.column ?? point.x ?? 0, row: point.row ?? point.y ?? 0 });
    return { ...input, tiles: input.tiles.map((tile) => ({ ...tile, grid: legacyPoint(tile.grid as { column?: number; row?: number; x?: number; y?: number }) })), items: input.items.map((item) => ({ ...item, grid: legacyPoint(item.grid as { column?: number; row?: number; x?: number; y?: number }) })) };
  }
  function pointerToScene(event: MouseEvent, source = event.currentTarget as HTMLElement) { const canvas = source.closest('.editor-canvas') as HTMLElement; const bounds = canvas.getBoundingClientRect(); const viewX = event.clientX - bounds.left - panX; const viewY = event.clientY - bounds.top - panY; return { x: viewCenter.x + (viewX - viewCenter.x) / zoom, y: viewCenter.y + (viewY - viewCenter.y) / zoom }; }
  function updatePointer(event: MouseEvent) { if (!transformValid) return; const point = pointerToScene(event); const gridPoint = sceneToGrid(transform, point.x, point.y); pointerGrid = { column: Math.round(gridPoint.column), row: Math.round(gridPoint.row) }; }
  function placeTile(event: MouseEvent) { placeTileAt(event); }
  function placeTileAt(event: MouseEvent) { if (!transformValid) return; const point = pointerToScene(event); const gridPoint = sceneToGrid(transform, point.x, point.y); const column = Math.round(gridPoint.column); const row = Math.round(gridPoint.row); snapshot(); const newTile: SceneTile = { id: `${selectedAsset}-${Date.now()}`, asset: selectedAsset, grid: { column, row }, layer: 0, rotation: 0 }; update(scenes.map((entry) => entry.id === scene.id ? { ...entry, tiles: [...entry.tiles, newTile] } : entry)); selectedTileId = newTile.id; }
  function selectTile(event: MouseEvent, tile: SceneTile) { event.stopPropagation(); if (placementMode) placeTileAt(event); else selectedTileId = tile.id; }
  function removeTile() { if (!selectedTile) return; snapshot(); update(scenes.map((entry) => entry.id === scene.id ? { ...entry, tiles: entry.tiles.filter((tile) => tile.id !== selectedTile.id) } : entry)); selectedTileId = ''; }
  function rotateTile() { if (!selectedTile) return; snapshot(); update(scenes.map((entry) => entry.id === scene.id ? { ...entry, tiles: entry.tiles.map((tile) => tile.id === selectedTile.id ? { ...tile, rotation: (tile.rotation + 90) % 360 } : tile) } : entry)); }
  function setTransform(field: keyof GridTransform, value: number) { transform = { ...transform, [field]: Number(value) }; }
  function addLevel() { const id = `new-level-${scenes.length + 1}`; snapshot(); const newScene: SceneDocument = { id, name: 'New Level', prompt: 'Describe this level.', background: { color: '#101c2b' }, tiles: [], items: [], commands: [], exits: [] }; update([...scenes, newScene]); selectedSceneId = id; }
  function addItem() { snapshot(); const item: SceneItem = { id: `item-${scene.items.length + 1}`, name: 'New Item', description: '', grid: { column: 1, row: 1 }, take: { commands: ['take item'], message: 'You pick up the item.' } }; update(scenes.map((entry) => entry.id === scene.id ? { ...entry, items: [...entry.items, item] } : entry)); }
  function addCommand() { snapshot(); const command: SceneCommand = { id: `command-${scene.commands.length + 1}`, aliases: ['new command'], message: 'Command response.', requires: [] }; update(scenes.map((entry) => entry.id === scene.id ? { ...entry, commands: [...entry.commands, command] } : entry)); }
  function updateField(field: 'name' | 'prompt', value: string) { update(scenes.map((entry) => entry.id === scene.id ? { ...entry, [field]: value } : entry)); }
  function undo() { const previous = history.at(-1); if (!previous) return; future = [structuredClone(scenes), ...future]; history = history.slice(0, -1); scenes = previous; }
  function redo() { const next = future[0]; if (!next) return; history = [...history, structuredClone(scenes)]; future = future.slice(1); scenes = next; }
  function downloadJson() { const link = document.createElement('a'); link.href = URL.createObjectURL(new Blob([jsonText], { type: 'application/json' })); link.download = `${scene.id}.json`; link.click(); URL.revokeObjectURL(link.href); }
  function importJson(event: Event) { const input = event.currentTarget as HTMLInputElement; const file = input.files?.[0]; if (!file) return; const reader = new FileReader(); reader.onload = () => { try { const imported = JSON.parse(String(reader.result)) as SceneDocument & { gridTransform?: GridTransform; view?: { zoom: number; panX: number; panY: number } }; snapshot(); if (imported.gridTransform) transform = imported.gridTransform; if (imported.view) { zoom = imported.view.zoom; panX = imported.view.panX; panY = imported.view.panY; } const normalized = normalizeScene(imported); update([...scenes.filter((entry) => entry.id !== normalized.id), normalized]); selectedSceneId = normalized.id; jsonError = ''; } catch { jsonError = 'Invalid scene JSON.'; } }; reader.readAsText(file); }
  async function checkScene() { validation = 'VALIDATING...'; try { const result = await validateScene(scene); validation = result.valid ? 'JSON VALID // SERVER ACCEPTED' : result.errors.join(' | '); } catch (error) { validation = `SERVER UNAVAILABLE: ${error}`; } }
  function saveAndReturn() {
    const savedScene = { ...scene, gridTransform: transform, view: { zoom, panX, panY } };
    const stored = localStorage.getItem('ares-saved-scenes');
    const savedScenes = stored ? JSON.parse(stored) as (SceneDocument & { gridTransform: GridTransform; view: { zoom: number; panX: number; panY: number } })[] : [];
    localStorage.setItem('ares-saved-scenes', JSON.stringify([...savedScenes.filter((entry) => entry.id !== savedScene.id), savedScene]));
    onSave(savedScene);
  }
  function assetStyle(tile: SceneTile, small = false, order = 0) { const asset = assetManifest[tile.asset] ?? assetManifest['template-floor']; const point = gridToScene(transform, tile.grid.column, tile.grid.row); const scale = small ? .55 : 1; return `left: ${point.x * scale - asset.anchorX * scale}px; top: ${point.y * scale - asset.anchorY * scale}px; width: ${asset.width * scale}px; height: ${asset.height * scale}px; transform: rotate(${tile.rotation}deg); z-index: ${order + 1}; background-image: url('/Modular-Space-Kit/Previews/${tile.asset}.png');`; }

  onMount(() => { const stored = localStorage.getItem('ares-editor-scenes'); if (stored) { try { scenes = (JSON.parse(stored) as SceneDocument[]).map(normalizeScene); selectedSceneId = scenes[0]?.id ?? selectedSceneId; } catch { localStorage.removeItem('ares-editor-scenes'); } } });
</script>

<main class="editor-shell">
  <header class="topbar editor-topbar"><div class="brand-mark">A7</div><div><p class="eyebrow">ARES PROGRAM // AUTHORING SYSTEM</p><h1>SCENE EDITOR</h1></div><button class="back-button" on:click={onClose}>← RETURN TO GAME</button></header>
  <div class="editor-toolbar panel"><div class="toolbar-group"><label>LEVEL <select bind:value={selectedSceneId}>{#each scenes as entry}<option value={entry.id}>{entry.name}</option>{/each}</select></label><button on:click={addLevel}>+ NEW LEVEL</button></div><div class="toolbar-group"><button on:click={undo} disabled={!history.length}>UNDO</button><button on:click={redo} disabled={!future.length}>REDO</button><button on:click={checkScene}>VALIDATE JSON</button><label class="file-button">IMPORT JSON<input type="file" accept="application/json" on:change={importJson} /></label><button on:click={downloadJson}>EXPORT JSON</button><button class="save-button" on:click={saveAndReturn}>SAVE & RETURN</button></div></div>
  {#if validation}<div class="validation-banner">{validation}</div>{/if}{#if jsonError}<div class="validation-banner error">{jsonError}</div>{/if}
  <div class="editor-layout">
    <aside class="editor-panel panel"><div class="panel-title">ASSET PALETTE <span>{filteredAssets.length}/{assets.length} PNG</span></div><input class="asset-search" bind:value={assetSearch} placeholder="SEARCH ASSETS..." aria-label="Search assets" /><div class="asset-palette">{#each filteredAssets as asset}<button class:selected={selectedAsset === asset} on:click={() => selectedAsset = asset}><img src={`/Modular-Space-Kit/Previews/${asset}.png`} alt="" /><span>{asset}</span></button>{:else}<div class="empty-slot">NO MATCHING ASSETS</div>{/each}</div><p class="editor-help">Select an asset, then click the oblique grid to place it.</p></aside>
    <section class="editor-canvas-wrap panel"><div class="panel-title">{scene.name.toUpperCase()} <span>OBLIQUE GRID // {transformValid ? 'INVERTIBLE' : 'PARALLEL VECTORS'}</span></div><div class="canvas-controls"><label>ZOOM {Math.round(zoom * 100)}% <input type="range" min="0.5" max="3" step="0.05" bind:value={zoom} /></label><button on:click={() => { panX = 0; panY = 0; zoom = 1.3; }}>RESET VIEW</button><button class:active-mode={placementMode} on:click={() => placementMode = true}>PLACE MODE</button><button class:active-mode={!placementMode} on:click={() => placementMode = false}>SELECT MODE</button><label><input type="checkbox" bind:checked={showGrid} /> GRID</label></div>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="editor-canvas" role="application" aria-label="Oblique scene editor canvas" on:click={placeTile} on:mousemove={updatePointer} style={`background-color: ${scene.background.color}`}><div class="scene-stage" style={`transform: translate(${panX}px, ${panY}px) translate(${viewCenter.x}px, ${viewCenter.y}px) scale(${zoom}) translate(${-viewCenter.x}px, ${-viewCenter.y}px);`}>{#if showGrid}<svg class="grid-overlay" viewBox="0 0 700 610" preserveAspectRatio="none">{#each gridLines as line}<line x1={line.start.x} y1={line.start.y} x2={line.end.x} y2={line.end.y} />{/each}</svg>{/if}{#each orderedTiles as tile, index}<button class:selected={selectedTileId === tile.id} class="placed-tile" style={assetStyle(tile, false, index)} on:click={(event) => selectTile(event, tile)}><img src={`/Modular-Space-Kit/Previews/${tile.asset}.png`} width={assetManifest[tile.asset]?.width ?? 64} height={assetManifest[tile.asset]?.height ?? 64} alt={tile.asset} /></button>{/each}<svg class="placement-cell" viewBox="0 0 700 610" preserveAspectRatio="none"><polygon points={placementCell.map((point) => `${point.x},${point.y}`).join(' ')} /></svg></div></div><div class="canvas-actions"><span>{transformValid ? `POINTER // COLUMN ${pointerGrid.column}, ROW ${pointerGrid.row}` : 'INVALID GRID TRANSFORM'}</span><button on:click={rotateTile} disabled={!selectedTile}>ROTATE 90°</button><button on:click={removeTile} disabled={!selectedTile || placementMode}>DELETE</button></div></section>
    <aside class="editor-panel panel"><div class="panel-title">GRID & LEVEL DATA <span>JSON</span></div><div class="transform-section"><div class="data-heading">AFFINE TRANSFORM <span class:invalid={!transformValid}>{transformValid ? 'READY' : 'PARALLEL'}</span></div><div class="transform-fields"><label>ORIGIN X<input type="number" value={transform.originX} on:change={(event) => setTransform('originX', (event.currentTarget as HTMLInputElement).valueAsNumber)} /></label><label>ORIGIN Y<input type="number" value={transform.originY} on:change={(event) => setTransform('originY', (event.currentTarget as HTMLInputElement).valueAsNumber)} /></label><label>COLUMN X<input type="number" value={transform.columnStepX} on:change={(event) => setTransform('columnStepX', (event.currentTarget as HTMLInputElement).valueAsNumber)} /></label><label>COLUMN Y<input type="number" value={transform.columnStepY} on:change={(event) => setTransform('columnStepY', (event.currentTarget as HTMLInputElement).valueAsNumber)} /></label><label>ROW X<input type="number" value={transform.rowStepX} on:change={(event) => setTransform('rowStepX', (event.currentTarget as HTMLInputElement).valueAsNumber)} /></label><label>ROW Y<input type="number" value={transform.rowStepY} on:change={(event) => setTransform('rowStepY', (event.currentTarget as HTMLInputElement).valueAsNumber)} /></label></div><p class="calibration-help">Column step: (+36, +6) · Row step: (-10, +20). Integer coordinates are centers; boundaries use half-integers.</p></div><div class="data-section"><div class="data-heading">FLOOR PATCH PREVIEW <span>SEAM TEST</span></div><div class="patch-preview">{#each [{column: 0, row: 0}, {column: 1, row: 0}, {column: 0, row: 1}, {column: 1, row: 1}] as position}<img src="/Modular-Space-Kit/Previews/template-floor.png" alt="" style={`left: ${gridToScene(transform, position.column, position.row).x * .55 - 17.6}px; top: ${gridToScene(transform, position.column, position.row).y * .55 - 17.6}px;`} />{/each}</div></div><label class="field">DISPLAY NAME<input value={scene.name} on:change={(event) => updateField('name', (event.currentTarget as HTMLInputElement).value)} /></label><label class="field">PROMPT<textarea value={scene.prompt} on:change={(event) => updateField('prompt', (event.currentTarget as HTMLTextAreaElement).value)}></textarea></label><div class="data-section"><div class="data-heading">ITEMS <button on:click={addItem}>+</button></div>{#each scene.items as item}<div class="data-row">{item.name}<span>{item.id}</span></div>{:else}<div class="empty-slot">NO ITEMS</div>{/each}</div><div class="data-section"><div class="data-heading">COMMANDS <button on:click={addCommand}>+</button></div>{#each scene.commands as command}<div class="data-row">{command.aliases[0]}<span>{command.id}</span></div>{:else}<div class="empty-slot">NO COMMANDS</div>{/each}</div><details><summary>VIEW JSON</summary><textarea class="json-preview" readonly value={jsonText}></textarea></details></aside>
  </div>
</main>

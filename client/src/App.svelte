<script lang="ts">
  import { onMount } from 'svelte';
  import { newGame, sendCommand, type GameResponse } from './lib/api';
  import SceneEditor from './lib/SceneEditor.svelte';
  import { assetManifest, type SceneDocument } from './lib/scenes';
  import { gridToScene, type GridTransform } from './lib/gridTransform';

  type SavedScene = SceneDocument & { gridTransform: GridTransform; view?: { zoom: number; panX: number; panY: number } };

  let input = '';
  let game: GameResponse = { message: 'Connecting...', location: '', prompt: '', inventory: [], turn: 0, available_exits: [] };
  let history: { command?: string; message: string }[] = [];
  let editorMode = false;
  let savedScenes: SavedScene[] = [];

  const scenes: Record<string, { label: string; assets: string[]; accent: string }> = {
    'Control Room': {
      label: 'COMMAND DECK',
      assets: ['room-large', 'template-detail', 'cables', 'corridor-end'],
      accent: 'violet'
    },
    'Water Pump Room': {
      label: 'PUMP CHAMBER',
      assets: ['room-small', 'template-floor', 'corridor', 'gate-door'],
      accent: 'cyan'
    },
    'Robot Bay': {
      label: 'AUTOMATION BAY',
      assets: ['room-wide', 'gate-lasers', 'cables', 'corridor-wide'],
      accent: 'amber'
    }
  };

  $: scene = scenes[game.location] ?? { label: 'UNMAPPED SECTOR', assets: ['room-large'], accent: 'violet' };
  $: savedScene = savedScenes.find((entry) => entry.name === game.location);

  async function restart() {
    game = await newGame();
    history = [{ message: game.message }];
    input = '';
  }

  async function submit() {
    if (!input.trim()) return;
    const command = input;
    input = '';
    try {
      const result = await sendCommand(command);
      history = [...history, { command, message: result.message }];
      game = result;
    } catch (error) {
      history = [...history, { command, message: `Connection error: ${error}` }];
    }
  }

  function loadSavedScenes() {
    const stored = localStorage.getItem('ares-saved-scenes');
    if (stored) { try { savedScenes = JSON.parse(stored); } catch { localStorage.removeItem('ares-saved-scenes'); } }
  }

  function saveScene(savedScene: SavedScene) {
    savedScenes = [...savedScenes.filter((entry) => entry.id !== savedScene.id), savedScene];
    editorMode = false;
  }

  function savedTileStyle(tile: SavedScene['tiles'][number], transform: GridTransform) {
    const asset = assetManifest[tile.asset] ?? { width: 64, height: 64, anchorX: 32, anchorY: 32 };
    const point = gridToScene(transform, tile.grid.column, tile.grid.row);
    return `left: ${point.x - asset.anchorX}px; top: ${point.y - asset.anchorY}px; width: ${asset.width}px; height: ${asset.height}px; transform: rotate(${tile.rotation}deg);`;
  }

  function savedStageStyle(savedScene: SavedScene) {
    const view = savedScene.view ?? { zoom: 1.3, panX: 0, panY: 0 };
    return `transform: translate(${view.panX}px, ${view.panY}px) translate(350px, 305px) scale(${view.zoom}) translate(-350px, -305px);`;
  }

  onMount(() => { loadSavedScenes(); restart(); });
</script>

{#if editorMode}
  <SceneEditor onClose={() => editorMode = false} onSave={saveScene} />
{:else}
<main>
  <header class="topbar">
    <div class="brand-mark">A7</div>
    <div><p class="eyebrow">ARES PROGRAM // MARS COLONY</p><h1>ARES SILO 7</h1></div>
    <div class="connection"><span class="signal"></span> LINK STABLE</div>
  </header>

  <div class="layout">
    <section class="main-column">
      <section class={`scene-panel ${scene.accent}`}>
        <div class="scene-heading">
          <div><p class="eyebrow">CURRENT SECTOR</p><h2>{scene.label}</h2></div>
          <span class="scene-code">LVL // {String(game.turn).padStart(2, '0')}</span>
        </div>
        <div class:saved-scene-active={savedScene} class="scene-board" aria-label={`Static view of ${game.location}`}>
          <div class="scanlines"></div>
          {#if savedScene}
            <div class="saved-scene-stage" style={savedStageStyle(savedScene)}>{#each savedScene.tiles as tile, index}<img src={`/Modular-Space-Kit/Previews/${tile.asset}.png`} alt="" style={`${savedTileStyle(tile, savedScene.gridTransform)} z-index: ${index + 1};`} />{/each}</div>
          {:else}
            <div class="grid-floor"></div>
            {#each scene.assets as asset, index}<img class={`sprite sprite-${index + 1}`} src={`/Modular-Space-Kit/Previews/${asset}.png`} alt="" />{/each}
          {/if}
          <div class="scene-tag"><span class="dot"></span> LIVE SCENE MAP <small>STATIC DISPLAY</small></div>
        </div>
        <div class="scene-caption"><span>{game.location}</span><span>{game.prompt}</span></div>
      </section>

      <section class="console panel">
        <div class="panel-title"><span>MISSION LOG</span><span class="live-label">● REC</span></div>
        <div class="terminal">
          {#each history as entry}
            {#if entry.command}<div class="command">&gt; {entry.command}</div>{/if}
            <p>{entry.message}</p>
          {/each}
        </div>
        <form on:submit|preventDefault={submit}>
          <span class="prompt-mark">&gt;_</span>
          <input bind:value={input} placeholder="Enter a command..." aria-label="Game command" />
          <button>TRANSMIT</button>
        </form>
      </section>
    </section>

    <aside class="side-column">
      <section class="panel stats-panel">
        <div class="panel-title">SYSTEM STATUS <span class="status-chip">NOMINAL</span></div>
        <div class="stat-grid">
          <div><span class="stat-label">LOCATION</span><strong>{game.location}</strong></div>
          <div><span class="stat-label">TURN COUNT</span><strong>{String(game.turn).padStart(2, '0')}</strong></div>
          <div><span class="stat-label">OXYGEN</span><strong>87<span>%</span></strong></div>
          <div><span class="stat-label">SILO POWER</span><strong>64<span>%</span></strong></div>
        </div>
      </section>

      <section class="panel inventory-panel">
        <div class="panel-title">INVENTORY <span>{game.inventory.length.toString().padStart(2, '0')} SLOTS</span></div>
        <div class="inventory-list">
          {#each game.inventory as item}
            <div class="inventory-item"><span class="item-icon">✦</span><span>{item}</span><span class="item-ready">READY</span></div>
          {:else}
            <div class="empty-slot"><span>+</span> NO ITEMS ACQUIRED</div>
          {/each}
        </div>
      </section>

      <section class="panel exits-panel">
        <div class="panel-title">AVAILABLE ROUTES</div>
        <div class="route-list">
          {#each game.available_exits as exit}
            <div><span class="route-arrow">↗</span>{exit.toUpperCase()}<span class="route-open">OPEN</span></div>
          {:else}
            <div class="empty-slot">NO ROUTES DETECTED</div>
          {/each}
        </div>
      </section>

      <button class="restart-button" type="button" on:click={restart}>↻ &nbsp;REINITIALIZE SCENARIO</button>
      <button class="editor-button" type="button" on:click={() => editorMode = true}>⌘ &nbsp;SCENE EDITOR</button>
    </aside>
  </div>
</main>
{/if}

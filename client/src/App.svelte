<script lang="ts">
  import { onMount } from 'svelte';
  import { newGame, sendCommand, type GameResponse } from './lib/api';

  let input = '';
  let game: GameResponse = { message: 'Connecting...', location: '', inventory: [], turn: 0, available_exits: [] };
  let history: { command?: string; message: string }[] = [];

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

  onMount(restart);
</script>

<main>
  <header><h1>ARES SILO 7</h1><p>Mars Construction Facility</p></header>
  <section class="status"><span>LOCATION: {game.location}</span><span>TURN: {game.turn}</span></section>
  <section class="terminal">
    {#each history as entry}
      {#if entry.command}<div class="command">&gt; {entry.command}</div>{/if}
      <p>{entry.message}</p>
    {/each}
  </section>
  <aside>
    <div><h2>Inventory</h2>{#each game.inventory as item}<p>{item}</p>{:else}<p>Empty</p>{/each}</div>
    <div><h2>Exits</h2>{#each game.available_exits as exit}<p>{exit}</p>{/each}</div>
  </aside>
  <form on:submit|preventDefault={submit}>
    <input bind:value={input} autofocus placeholder="Enter a command..." />
    <button>Send</button><button type="button" on:click={restart}>Restart</button>
  </form>
</main>

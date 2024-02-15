<script lang="ts">
  import { onMount } from "svelte";
  import {
    solutionShape,
    currentLeapPositions,
    defaultOption,
    sampleSolution,
  } from "$lib/dog-shape";
  import SolutionCanvas from "$lib/SolutionCanvas.svelte";
  import { fetchPuzzles, hello, samplePuzzle } from "$lib/client";
  let time = 0;
  let step = 0;
  let leap = 0;
  $: {
    leap = time % 1;
    step = 0 | time;
  }
  let helloMsg = "";
  onMount(async () => {
    helloMsg = await hello();
  });

  let puzzles = Promise.resolve([samplePuzzle]);
  let cursor = 0;
  $: solution = puzzles.then((puzzle) => puzzle[cursor].solution);
  let page = 1;
  const limit = 30;
  $: puzzles = fetchPuzzles(page, limit);

  onMount(async () => {
    puzzles = fetchPuzzles(page, limit);
    puzzles.then((puzzles) => {
      helloMsg += ` ${puzzles.length} puzzles`;
    });
  });
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
  <h1>Puzzles</h1>
  <h2>{helloMsg}</h2>
  <div class="content">
    <div class="viewer">
      {#await solution then solution}
        <SolutionCanvas {solution} leap={{ step, leap }} />
        <input
          type="range"
          min="0"
          max={solution.moves.length}
          step="0.01"
          bind:value={time}
        />
        Step:{step}
        Leap:{leap}
      {/await}
      <button
        on:click={() => {
          page = page - 1;
        }}
      >
        Prev Page
      </button>
      <button
        on:click={() => {
          page = page + 1;
        }}
      >
        Next Page
      </button>
    </div>

    <div class="pane">
      {#await puzzles then puzzles}
        {#each puzzles as puzzle, i}
          <button
            class="selector"
            on:click={() => {
              cursor = i;
            }}
          >
            {i + 1}:{puzzle.name}
          </button>
        {/each}
      {/await}
    </div>
  </div>
</section>

<style>
  h2 {
    text-align: center;
  }
  .content {
    display: flex;
    justify-content: center;
  }
  .viewer {
    display: flex;
    flex-direction: column;
  }
  .pane {
    max-height: 100vh;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
  .selector {
    line-height: 2;
  }
</style>

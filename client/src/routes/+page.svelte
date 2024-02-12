<script lang="ts">
  import { onMount } from "svelte";
  import {
    solutionShape,
    currentLeapPositions,
    defaultOption,
    sampleSolution,
  } from "$lib/dog-shape";
  import SolutionCanvas from "$lib/SolutionCanvas.svelte";
  import { fetchPuzzles, hello } from "$lib/client";
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

  let puzzles = [];
  let solution = sampleSolution;
  onMount(async () => {
    puzzles = await fetchPuzzles();
    helloMsg += ` ${puzzles.length} puzzles`;
    solution = puzzles[0].solution;
  });
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
  <h1>Puzzles</h1>
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
  <p>
    {#each solution.pieces as piece, i}
      {i + 1}:{JSON.stringify(piece)}
    {/each}
  </p>
  <p>
    {#each solution.moves as move, i}
      {i + 1}:{JSON.stringify(move)}
    {/each}
  </p>
  <p>
    {#each currentLeapPositions(sampleSolution, { step, leap }) as piece, i}
      {i + 1}:{JSON.stringify(piece)}
    {/each}
  </p>
  <p>{helloMsg}</p>
</section>

<style>
</style>

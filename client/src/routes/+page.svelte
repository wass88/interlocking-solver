<script lang="ts">
  import { onMount } from "svelte";
  import {
    solutionShape,
    currentLeapPositions,
    defaultOption,
    sampleSolution,
  } from "$lib/dog-shape";
  import SolutionCanvas from "$lib/SolutionCanvas.svelte";
  let time = 0;
  let step = 0;
  let leap = 0;
  $: {
    step = time / 1;
    leap = time % 1;
  }
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
  <h1>Puzzles</h1>
  <SolutionCanvas solution={sampleSolution} leap={{ step, leap }} />
  <input
    type="range"
    min="0"
    max={sampleSolution.moves.length}
    step="0.01"
    bind:value={time}
  />
  Step:{step}
  Leap:{leap}
  <p>
    {#each sampleSolution.pieces as piece, i}
      {i + 1}:{JSON.stringify(piece)}
    {/each}
  </p>
  <p>
    {#each sampleSolution.moves as move, i}
      {i + 1}:{JSON.stringify(move)}
    {/each}
  </p>
  <p>
    {#each currentLeapPositions(sampleSolution, { step, leap }) as piece, i}
      {i + 1}:{JSON.stringify(piece)}
    {/each}
  </p>
</section>

<style>
</style>

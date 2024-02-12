<script lang="ts">
  import { writable } from "svelte/store";
  import Zdog from "zdog";
  import {
    defaultOption,
    type Solution,
    type MoveLeap,
    updateSolution,
    solutionShape,
  } from "./dog-shape";
  export let solution: Solution;
  export let leap: MoveLeap;

  const ref = writable<HTMLCanvasElement | null>(null);
  let illo: Zdog.Illustration | null = null;
  let option = defaultOption;
  $: {
    console.log("ref", illo, $ref, leap, solution);
    if (!illo && $ref) {
      console.log("update");
      illo = new Zdog.Illustration({
        element: $ref,
        dragRotate: true,
      });
      new Zdog.Ellipse({
        addTo: illo,
        diameter: 80,
        stroke: 10,
        color: "#636",
      });

      let puzzle = solutionShape(option, solution);
      illo.addChild(puzzle);

      function animate() {
        updateSolution(option, puzzle, solution, leap);
        illo.updateRenderGraph();
        requestAnimationFrame(animate);
      }
      animate();
    }
  }
</script>

<canvas bind:this={$ref} width="200" height="200"></canvas>

<style>
  canvas {
    border: 1px solid #444;
  }
</style>

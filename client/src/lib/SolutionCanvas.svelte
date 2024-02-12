<script lang="ts">
  import { writable } from "svelte/store";
  import Zdog from "zdog";
  import { defaultOption, updateSolution, solutionShape } from "./dog-shape";
  import { onMount } from "svelte";
  import type { MoveLeap, Solution } from "./types";
  export let solution: Solution;
  export let leap: MoveLeap;

  const ref = writable<HTMLCanvasElement | null>(null);
  let illo: Zdog.Illustration | null = null;
  let stage: Zdog.Anchor | null = null;
  let option = defaultOption;
  let rid = 0;
  let animate = () => {
    rid = requestAnimationFrame(animate);
  };
  $: {
    animate = () => {
      try {
        if (illo && stage) {
          updateSolution(option, stage.children[0], solution, leap);
          illo.updateRenderGraph();
        }
        rid = requestAnimationFrame(animate);
      } catch (e) {
        console.error(e);
      }
    };
  }
  onMount(() => {
    animate();
    return () => {
      cancelAnimationFrame(rid);
    };
  });
  let prev_solution: Solution | null = null;
  $: {
    if (!illo && $ref) {
      illo = new Zdog.Illustration({
        element: $ref,
        dragRotate: true,
      });
      stage = new Zdog.Anchor({
        addTo: illo,
        translate: { x: -40, y: -40 },
      });
      let puzzle = solutionShape(option, solution);
      stage.addChild(puzzle);
      prev_solution = solution;
    }
  }
  $: {
    if (prev_solution && prev_solution !== solution) {
      stage.children[0] = solutionShape(option, solution);
      prev_solution = solution;
    }
  }
</script>

<canvas bind:this={$ref} width="300" height="300"></canvas>

<style>
  canvas {
    border: 1px solid #444;
  }
</style>

import { sampleSolution } from "./dog-shape";
import { SolutionSchema } from "./types";
import { z } from "zod";

const PuzzleScheme = z.object({
  code: z.string(),
  name: z.string(),
  solution: SolutionSchema,
});
type Puzzle = z.infer<typeof PuzzleScheme>;
export async function fetchPuzzles(
  page: number,
  limit: number
): Promise<Puzzle[]> {
  const res = await fetch(`/api/puzzles?page=${page}&limit=${limit}`);
  try {
    const jsoned = await res.json();
    z.array(PuzzleScheme).parse(jsoned);
    return jsoned;
  } catch (e) {
    console.error(e);
  }

  return [];
}

export async function hello() {
  const res = await fetch("/api/hello");
  return res.text();
}

export const samplePuzzle = {
  code: "sample",
  name: "Sample Puzzle",
  solution: sampleSolution,
};

import Zdog from "zdog";
import type { RenderOption, Solution, MoveLeap, Piece } from "./types";
export function solutionShape(option: RenderOption, solution: Solution) {
  let anchor = new Zdog.Anchor();
  solution.pieces.forEach((piece, i) => {
    anchor.addChild(pieceShape(option, piece, colors[i]));
  });
  return anchor;
}
export function updateSolution(
  option: RenderOption,
  anchor: Zdog.Anchor,
  solution: Solution,
  leap: MoveLeap
) {
  let positions = currentLeapPositions(solution, leap);
  positions.forEach((pos, i) => {
    anchor.children[i].translate = {
      x: pos.x * option.cellSize,
      y: pos.y * option.cellSize,
      z: pos.z * option.cellSize,
    };
  });
}
export function pieceShape(
  option: RenderOption,
  block: Piece,
  color: string = "#636"
) {
  let anchor = new Zdog.Anchor();
  block.blocks.forEach(({ x, y, z }) => {
    new Zdog.Box({
      addTo: anchor,
      width: option.shrinkSize,
      height: option.shrinkSize,
      depth: option.shrinkSize,
      color,
      stroke: 3,
      fill: false,
      translate: {
        x: x * option.cellSize,
        y: y * option.cellSize,
        z: z * option.cellSize,
      },
    });
  });
  return anchor;
}

export function currentLeapPositions(solution: Solution, leap: MoveLeap) {
  if (leap.step >= solution.moves.length) {
    return currentPositions(solution, solution.moves.length);
  }
  const current = currentPositions(solution, leap.step);
  const next = currentPositions(solution, leap.step + 1);
  return current.map((pos, i) => ({
    x: pos.x + (next[i].x - pos.x) * leap.leap,
    y: pos.y + (next[i].y - pos.y) * leap.leap,
    z: pos.z + (next[i].z - pos.z) * leap.leap,
  }));
}
function currentPositions(solution: Solution, step: number) {
  let positions = solution.pieces.map((_, i) => {
    return solution.moves.slice(0, step).reduce(
      (pos, move) => {
        if (move.pieces.includes(i)) {
          pos.x += move.translate.x;
          pos.y += move.translate.y;
          pos.z += move.translate.z;
        }
        return pos;
      },
      { x: 0, y: 0, z: 0 }
    );
  });
  return positions;
}

export const defaultOption: RenderOption = {
  cellSize: 32,
  shrinkSize: 28,
};
export const samplePiece: Piece = {
  blocks: [
    { x: 0, y: 0, z: 0 },
    { x: 1, y: 0, z: 0 },
    { x: 0, y: 1, z: 0 },
    { x: 0, y: 0, z: 1 },
  ],
};
export const samplePiece2: Piece = {
  blocks: [
    { x: 1, y: 0, z: 0 },
    { x: 0, y: 1, z: 1 },
    { x: 1, y: 1, z: 1 },
  ],
};

export const sampleSolution: Solution = {
  pieces: [samplePiece, samplePiece2],
  moves: [
    { pieces: [0], translate: { x: 1, y: 0, z: 0 } },
    { pieces: [1], translate: { x: 0, y: 1, z: 0 } },
  ],
};

export const colors = [
  "#e39aac",
  "#c45d9f",
  "#634b7d",
  "#6461c2",
  "#2ba9b4",
  "#93d4b5",
];
export const backgroundColor = "#f0f6e8";

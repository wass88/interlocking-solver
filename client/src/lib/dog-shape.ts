import Zdog from "zdog";
import type { RenderOption, Solution, MoveLeap, Piece, Coord } from "./types";
export function solutionShape(option: RenderOption, solution: Solution) {
  let anchor = new Zdog.Anchor({
    rotate: {
      x: -Math.PI / 5,
      y: -Math.PI / 5,
    },
  });
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
  positions.forEach(({ pos, opacity }, i) => {
    anchor.children[i].translate = {
      x: pos.x * option.cellSize,
      y: pos.y * option.cellSize,
      z: pos.z * option.cellSize,
    };
    anchor.children[i].children.map((box: Zdog.Box) =>
      box.children.map(
        (face: Zdog.Rect) => (face.color = getOpacityColor(colors[i], opacity))
      )
    );
  });
}
function getOpacityColor(color: string, opacity: number) {
  let c = parseInt(color.slice(1), 16);
  let r = (c >> 16) & 0xff;
  let g = (c >> 8) & 0xff;
  let b = c & 0xff;
  return `rgba(${r},${g},${b},${opacity})`;
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

type PositionState = {
  pos: Coord;
  exists: boolean;
  opacity: number;
};

export function currentLeapPositions(
  solution: Solution,
  leap: MoveLeap
): PositionState[] {
  if (leap.step >= solution.moves.length) {
    return currentPositions(solution, solution.moves.length);
  }
  const current = currentPositions(solution, leap.step);
  const next = currentPositions(solution, leap.step + 1);
  return current.map(({ pos }, i) => ({
    pos: {
      x: pos.x + (next[i].pos.x - pos.x) * leap.leap,
      y: pos.y + (next[i].pos.y - pos.y) * leap.leap,
      z: pos.z + (next[i].pos.z - pos.z) * leap.leap,
    },
    exists: next[i].exists,
    opacity:
      current[i].opacity + (next[i].opacity - current[i].opacity) * leap.leap,
  }));
}
function currentPositions(solution: Solution, step: number): PositionState[] {
  return solution.pieces.map((_, i) => {
    return solution.moves.slice(0, step).reduce(
      (state: PositionState, move) => {
        if (move.pieces.includes(i)) {
          if (!state.exists || move.translate === null) {
            return {
              pos: state.pos,
              exists: false,
              opacity: 0,
            };
          }
          return {
            pos: {
              x: state.pos.x + move.translate.x,
              y: state.pos.y + move.translate.y,
              z: state.pos.z + move.translate.z,
            },
            exists: true,
            opacity: 1,
          };
        }
        return state;
      },
      { pos: { x: 0, y: 0, z: 0 }, exists: true, opacity: 1 }
    );
  });
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
    { pieces: [0], translate: null },
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
  "#a7d129",
  "#3e5f2d",
];
export const backgroundColor = "#f0f6e8";

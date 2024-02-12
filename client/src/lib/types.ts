import { z } from "zod";
export type RenderOption = {
  cellSize: number;
  shrinkSize: number;
};
export type MoveLeap = {
  step: number;
  leap: number;
};

export const CoordSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
export type Coord = z.infer<typeof CoordSchema>;
export const PieceSchema = z.object({ blocks: z.array(CoordSchema) });
export type Piece = z.infer<typeof PieceSchema>;
export const MoveSchema = z.object({
  pieces: z.array(z.number()),
  translate: CoordSchema,
});
export type Move = z.infer<typeof MoveSchema>;
export const SolutionSchema = z.object({
  pieces: z.array(PieceSchema),
  moves: z.array(MoveSchema),
});
export type Solution = z.infer<typeof SolutionSchema>;

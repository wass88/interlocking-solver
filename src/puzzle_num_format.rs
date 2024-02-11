use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

use crate::{
    cells::{Cells, V3Iter, V3},
    puzzle::{Piece, Puzzle},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleNumFormat {
    size: V3,
    piece: usize,
    cells: Vec<usize>,
}
impl PuzzleNumFormat {
    pub fn new(size: V3, piece: usize, cells: Vec<usize>) -> Self {
        Self { size, piece, cells }
    }
    fn from_puzzle(puzzle: &Puzzle) -> Self {
        let size = puzzle.size;
        let mut cells = vec![0; size * size * size];
        for (x, y, z) in V3Iter::cube(puzzle.size) {
            let index = Cells::to_index(puzzle.size, x, y, z);
            for i in 0..puzzle.pieces.len() {
                if puzzle.pieces[i].block.getv((x, y, z)) {
                    cells[index] = i + 1;
                }
            }
        }
        Self::new((size, size, size), puzzle.pieces.len(), cells)
    }
    fn to_puzzle(&self) -> Puzzle {
        let mut pieces = vec![Piece::empty(self.size.0); self.piece];
        for i in 0..self.cells.len() {
            let p = self.cells[i];
            if p > 0 {
                pieces[p - 1]
                    .block
                    .setv(Cells::from_index(self.size.0, i), true);
            }
        }
        Puzzle {
            size: self.size.0,
            pieces,
            margin: self.size.0,
            space: self.size.0 * 4,
            reach_limit: None,
        }
    }
    pub fn to_block_code(&self) -> String {
        let mut code = String::new();
        let (x, y, z) = self.size;
        code += &format!("{}{}{}:{}:", x, y, z, self.piece);
        for (x, y, z) in V3Iter::new(self.size) {
            let index = Cells::to_index(self.size.0, x, y, z);
            code += &format!("{}", self.cells[index]);
        }
        code
    }
    fn from_block_code(code: &str) -> Self {
        let code = code.split(':').collect::<Vec<_>>();
        let size = code[0].as_bytes();
        let size = (size[0] - b'0', size[1] - b'0', size[2] - b'0');
        let size = (size.0 as usize, size.1 as usize, size.2 as usize);
        let piece = code[1].parse().unwrap();
        let cells = code[2]
            .as_bytes()
            .iter()
            .map(|&c| (c - b'0') as usize)
            .collect();
        Self::new(size, piece, cells)
    }
    fn rotate(&self, rot: &V3Matrix) -> Self {
        let mut cells = self.clone();
        for (x, y, z) in V3Iter::new(self.size) {
            let (nx, ny, nz) = rot.mulvec(&(x, y, z));
            let index = Cells::to_index(self.size.0, x, y, z);
            let new_index = Cells::to_index(self.size.0, nx, ny, nz);
            cells.cells[new_index] = self.cells[index];
        }
        cells
    }
    fn rotate_index(&self, index: &[usize]) -> Self {
        let mut cells = self.clone();
        for i in 0..cells.cells.len() {
            cells.cells[i] = index[self.cells[i]];
        }
        cells
    }
    fn rotate_all(&self) -> Vec<Self> {
        let mut result = Vec::new();
        use itertools::Itertools;
        for mut index in (1..self.piece + 1).permutations(self.piece) {
            index.insert(0, 0);
            let rot_index = self.rotate_index(&index);
            for rot in V3Matrix::rot_all(self.size.0).iter() {
                result.push(rot_index.rotate(rot));
            }
        }
        result
    }
    pub fn normalize(&self) -> Self {
        self.rotate_all()
            .into_iter()
            .min_by_key(|format| format.to_block_code())
            .unwrap()
    }
    pub fn is_connected(&self) -> bool {
        self.to_puzzle()
            .pieces
            .iter()
            .all(|piece| piece.block.is_connected())
    }
    pub fn is_no_empty(&self) -> bool {
        self.to_puzzle()
            .pieces
            .iter()
            .all(|piece| piece.block.count() > 0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct V3Matrix {
    matrix: Vec<Vec<isize>>,
}
static ROT_ALL: Lazy<HashMap<usize, Vec<V3Matrix>>> = Lazy::new(|| {
    let mut rots = HashMap::new();
    for size in 2..=4 {
        rots.insert(size, V3Matrix::rot_all_gen(size));
    }
    rots
});
impl V3Matrix {
    const DIM: usize = 4;
    fn zero() -> Self {
        let matrix = vec![vec![0; V3Matrix::DIM]; V3Matrix::DIM];
        Self { matrix }
    }
    fn one() -> Self {
        let mut matrix = Self::zero();
        for i in 0..V3Matrix::DIM {
            matrix.matrix[i][i] = 1isize
        }
        matrix
    }
    fn mul(&self, other: &Self) -> Self {
        let mut matrix = Self::zero();
        for i in 0..V3Matrix::DIM {
            for j in 0..V3Matrix::DIM {
                for k in 0..V3Matrix::DIM {
                    matrix.matrix[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        matrix
    }
    fn mulvec(&self, vec: &V3) -> V3 {
        let mut result = [0; V3Matrix::DIM];
        let vec = [vec.0 as isize, vec.1 as isize, vec.2 as isize, 1isize];
        for i in 0..V3Matrix::DIM {
            for j in 0..V3Matrix::DIM {
                result[j] += vec[i] * self.matrix[i][j];
            }
        }
        let [x, y, z, _] = result;
        assert!(x >= 0);
        assert!(y >= 0);
        assert!(z >= 0);
        (x as usize, y as usize, z as usize)
    }
    fn rot_x(size: usize) -> Self {
        // 1 0 0 0 -> 1 0 0 0
        // 0 1 0 0 -> 0 0 1 0
        // 0 0 1 0 -> 0 -1 0 0
        // 0 0 0 1 -> 0 3 0 1
        Self {
            matrix: vec![
                vec![1, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, -1, 0, 0],
                vec![0, size as isize - 1, 0, 1],
            ],
        }
    }
    fn rot_y(size: usize) -> Self {
        Self {
            matrix: vec![
                vec![0, 0, -1, 0],
                vec![0, 1, 0, 0],
                vec![1, 0, 0, 0],
                vec![0, 0, size as isize - 1, 1],
            ],
        }
    }
    fn rot_z(size: usize) -> Self {
        Self {
            matrix: vec![
                vec![0, 1, 0, 0],
                vec![-1, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![size as isize - 1, 0, 0, 1],
            ],
        }
    }
    fn rot_all(size: usize) -> &'static [Self] {
        &ROT_ALL[&size]
    }
    fn rot_all_gen(size: usize) -> Vec<Self> {
        let mut matrixes = HashSet::new();
        let mut rot_x = V3Matrix::one();
        for _ in 0..4 {
            let mut rot_y = rot_x.clone();
            for _ in 0..4 {
                let mut rot_z = rot_y.clone();
                for _ in 0..4 {
                    matrixes.insert(rot_z.clone());
                    rot_z = rot_z.mul(&V3Matrix::rot_z(size));
                }
                rot_y = rot_y.mul(&V3Matrix::rot_y(size));
            }
            rot_x = rot_x.mul(&V3Matrix::rot_x(size));
        }
        let matrixes = matrixes.into_iter().collect::<Vec<_>>();
        assert_eq!(matrixes.len(), 24);
        matrixes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotate() {
        let size = (2, 2, 2);
        let cells = vec![0, 1, 0, 0, 0, 2, 0, 0];
        let format = PuzzleNumFormat::new(size, 2, cells);
        let rot = V3Matrix::rot_x(2);
        let format = format.rotate(&rot);
        assert_eq!(format.cells, vec![0, 2, 0, 1, 0, 0, 0, 0]);
    }
    #[test]
    fn test_rotate_index() {
        let size = (2, 2, 2);
        let cells = vec![0, 1, 0, 0, 0, 2, 0, 0];
        let format = PuzzleNumFormat::new(size, 2, cells);
        let format = format.rotate_index(&[0, 2, 1]);
        assert_eq!(format.cells, vec![0, 2, 0, 0, 0, 1, 0, 0]);
    }
    #[test]
    fn test_normalize() {
        let size = (2, 2, 2);
        let cells = vec![0, 1, 0, 0, 0, 2, 0, 0];
        let format = PuzzleNumFormat::new(size, 2, cells);
        let format = format.normalize();
        assert_eq!(format.cells, vec![0, 0, 0, 0, 0, 0, 1, 2]);
    }
    #[test]
    fn test_rotate_all() {
        let size = (2, 2, 2);
        let cells = vec![0, 1, 0, 0, 0, 2, 0, 0];
        let format = PuzzleNumFormat::new(size, 2, cells);
        let formats = format.rotate_all();
        assert_eq!(formats.len(), 24 * 2);
    }
    #[test]
    fn test_puzzle_num_format() {
        let size = (2, 2, 2);
        let cells = vec![0, 1, 0, 0, 0, 2, 0, 0];
        let format = PuzzleNumFormat::new(size, 2, cells);
        let puzzle = format.to_puzzle();
        assert_eq!(format.to_block_code(), "222:2:01000200");
        assert_eq!(format, PuzzleNumFormat::from_puzzle(&puzzle));
    }
    #[test]
    fn test_from_block_code() {
        let code = "222:2:01000200";
        let format = PuzzleNumFormat::from_block_code(code);
        assert_eq!(format.size, (2, 2, 2));
        assert_eq!(format.piece, 2);
        assert_eq!(format.cells, vec![0, 1, 0, 0, 0, 2, 0, 0]);
        assert_eq!(format.to_block_code(), code);
    }
    #[test]
    fn test_rot_all() {
        let mut rot_x = V3Matrix::rot_x(4);
        let mut rot_y = V3Matrix::rot_y(4);
        let mut rot_z = V3Matrix::rot_z(4);
        for _ in 0..3 {
            rot_x = rot_x.mul(&V3Matrix::rot_x(4));
            rot_y = rot_y.mul(&V3Matrix::rot_y(4));
            rot_z = rot_z.mul(&V3Matrix::rot_z(4));
        }
        assert_eq!(rot_x, V3Matrix::one());
        assert_eq!(rot_y, V3Matrix::one());
        assert_eq!(rot_z, V3Matrix::one());
        let matrixes = V3Matrix::rot_all(2);
        assert_eq!(matrixes.len(), 24);
    }
}

use crate::{puzzle_num_format::PuzzleNumFormat, v3::V3};

pub struct GenAllPuzzles {
    pub size: usize,
    pub piece: usize,
    pub holes: usize, // TODO
}
impl GenAllPuzzles {
    pub fn generate<W: GenAllWriter>(self, writer: &mut W) {
        let mut i = 0;
        loop {
            if i % 10000 == 0 {
                println!("# {}", i);
            }
            let cell_num = self.size * self.size * self.size;
            let mut cells = vec![0; cell_num];
            let mut a = i;
            for k in 0..cell_num {
                cells[k] = a % self.piece + 1;
                a /= self.piece;
            }
            if a > 0 {
                break;
            }
            let puzzle = PuzzleNumFormat::new(V3::cube(self.size), self.piece, cells);
            if puzzle.is_connected() && puzzle.is_no_empty() {
                writer.write_puzzle(&puzzle);
            }
            i += 1
        }
    }
}
trait GenAllWriter {
    fn write_puzzle(&mut self, puzzle: &PuzzleNumFormat);
}
struct DBWriter {
    client: mongodb::Client,
}
impl GenAllWriter for DBWriter {
    fn write_puzzle(&mut self, _puzzle: &PuzzleNumFormat) {
        unimplemented!()
    }
}
pub struct DebugWriter {
    pub codes: std::collections::HashSet<String>,
}
impl DebugWriter {
    pub fn new() -> Self {
        Self {
            codes: std::collections::HashSet::new(),
        }
    }
}
impl GenAllWriter for DebugWriter {
    fn write_puzzle(&mut self, puzzle: &PuzzleNumFormat) {
        let code = puzzle.normalize().to_block_code();
        if !self.codes.contains(&code) {
            println!("{}", code);
            self.codes.insert(code);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_all_puzzles() {
        let mut writer = DebugWriter {
            codes: std::collections::HashSet::new(),
        };
        GenAllPuzzles {
            size: 2,
            piece: 2,
            holes: 0,
        }
        .generate(&mut writer);
        assert_eq!(writer.codes.len(), 7);
    }
}

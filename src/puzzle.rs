#[derive(Clone)]
pub struct Puzzle {
    pieces: Vec<Piece>,
    size: usize,
    start: usize,
    space: usize,
}

use std::collections::HashMap;
#[derive(Clone)]
pub struct SolveResult {
    pub ok: bool,
    step: Option<usize>,
    reached: HashMap<Vec<usize>, Vec<usize>>,
    end_state: Option<Vec<usize>>,
}

fn state_to_vec(size: usize, state: &Vec<usize>) -> Vec<(usize, usize, usize)> {
    state
        .iter()
        .map(|&x| Cells::from_index(size, x))
        .collect::<Vec<_>>()
}
impl Puzzle {
    fn init_state(&self) -> Vec<usize> {
        let init_pos = Cells::to_index(self.space, self.start, self.start, self.start);
        vec![init_pos; self.pieces.len()]
    }
    pub fn solve(&self) -> SolveResult {
        let mut reached = HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.init_state(), 0));
        while let Some((state, step)) = queue.pop_front() {
            if self.collides(&state) {
                continue;
            }
            if self.is_solved(&state) {
                println!(
                    "solved step = {}, {:?},",
                    step,
                    state_to_vec(self.space, &state)
                );
                return SolveResult {
                    ok: true,
                    step: Some(step),
                    reached,
                    end_state: Some(state),
                };
            }
            for next_state in self.next_states(&state) {
                if reached.contains_key(&next_state) {
                    continue;
                }
                reached.insert(next_state.clone(), state.clone());
                queue.push_back((next_state, step + 1));
            }
        }
        println!("missing solution");
        SolveResult {
            ok: false,
            step: None,
            reached,
            end_state: None,
        }
    }
    fn is_solved(&self, state: &Vec<usize>) -> bool {
        let mut cells = Cells::empty(self.space);
        for (i, &pos) in state.iter().enumerate() {
            let (x, y, z) = Cells::from_index(self.space, pos);
            let boxed = self.pieces[i].block.boxed();
            for dx in 0..self.size {
                for dy in 0..self.size {
                    for dz in 0..self.size {
                        if !boxed.get(dx, dy, dz) {
                            continue;
                        }
                        let (nx, ny, nz) = (x + dx, y + dy, z + dz);
                        if cells.get(nx, ny, nz) {
                            return false;
                        }
                        cells.set(nx, ny, nz, true);
                    }
                }
            }
        }
        true
    }
    fn next_states(&self, state: &Vec<usize>) -> Vec<Vec<usize>> {
        let mut next_states = Vec::new();
        for (i, &pos) in state.iter().enumerate() {
            let (x, y, z) = Cells::from_index(self.space, pos);
            for (dx, dy, dz) in D6 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                let nz = z as isize + dz;
                if nx < 0 || ny < 0 || nz < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let nz = nz as usize;
                if nx >= self.space || ny >= self.space || nz >= self.space {
                    continue;
                }
                let mut next_state = state.clone();
                let next_pos = Cells::to_index(self.space, nx, ny, nz);
                next_state[i] = next_pos;
                next_states.push(next_state);
            }
        }
        next_states
    }
    fn collides(&self, state: &Vec<usize>) -> bool {
        let mut cells = Cells::empty(self.space);
        for (i, &pos) in state.iter().enumerate() {
            let piece = &self.pieces[i];
            let (x, y, z) = Cells::from_index(self.space, pos);
            for dx in 0..piece.size {
                for dy in 0..piece.size {
                    for dz in 0..piece.size {
                        let (nx, ny, nz) = (x + dx, y + dy, z + dz);
                        if nx >= self.space || ny >= self.space || nz >= self.space {
                            return true;
                        }
                        if !self.pieces[i].block.get(dx, dy, dz) {
                            continue;
                        }
                        if cells.get(nx, ny, nz) {
                            return true;
                        }
                        cells.set(nx, ny, nz, true);
                    }
                }
            }
        }
        false
    }
    fn base(size: usize) -> Puzzle {
        let mut pieces = vec![Piece::empty(size); size];
        for i in 0..size {
            for x in 0..size {
                for y in 0..size {
                    pieces[i].block.set(x, y, i, true);
                }
            }
        }
        Puzzle {
            pieces,
            size,
            start: size,
            space: size * 3,
        }
    }
    pub fn to_str(&self) -> String {
        let mut s = String::new();
        for (i, piece) in self.pieces.iter().enumerate() {
            s.push_str(&format!("#{}\n{}\n", i, piece.block.to_str()));
        }
        s
    }
}
type Moves = Vec<(usize, isize, isize, isize)>;
impl SolveResult {
    pub fn moves(&self, puzzle: &Puzzle) -> Moves {
        let mut moves = Vec::new();
        let mut end_state = self.end_state.clone().unwrap();
        let init_state = puzzle.init_state();
        while end_state != init_state {
            let prev_state = self.reached.get(&end_state).unwrap();
            for (i, (&pos, &prev_pos)) in end_state.iter().zip(prev_state.iter()).enumerate() {
                if pos != prev_pos {
                    let (x, y, z) = Cells::from_index(puzzle.space, pos);
                    let (px, py, pz) = Cells::from_index(puzzle.space, prev_pos);
                    moves.push((
                        i,
                        x as isize - px as isize,
                        y as isize - py as isize,
                        z as isize - pz as isize,
                    ));
                }
            }
            end_state = prev_state.clone();
        }
        moves.reverse();
        moves
    }
    pub fn shrink_move(&self, moves: Moves) -> Moves {
        if moves.len() == 0 {
            return moves;
        }
        let mut shrinked = Vec::new();
        let mut prev = moves[0];
        for &m in moves.iter().skip(1) {
            if prev.0 == m.0 {
                prev.1 += m.1;
                prev.2 += m.2;
                prev.3 += m.3;
            } else {
                shrinked.push(prev);
                prev = m;
            }
        }
        shrinked.push(prev);
        shrinked
    }
}

pub struct PuzzleSearcher {
    size: usize,
    tries: usize,
}
impl PuzzleSearcher {
    pub fn new(size: usize, tries: usize) -> PuzzleSearcher {
        PuzzleSearcher { size, tries }
    }
    pub fn search<T: PuzzleGenerator>(&self) -> Puzzle {
        let mut puzzle = Puzzle::base(self.size);
        let mut max_move = 0;
        let mut best_puzzle = puzzle.clone();
        for i in 0..self.tries {
            println!("Trying #{}(max={}) \n{}", i, max_move, puzzle.to_str());
            let result = puzzle.solve();
            if result.ok {
                let moves = result.shrink_move(result.moves(&puzzle)).len();
                if max_move < moves {
                    max_move = moves;
                    best_puzzle = puzzle.clone();
                }
            }
            puzzle = T::generate(&puzzle);
        }
        best_puzzle
    }
}

trait PuzzleGenerator {
    fn generate(puzzle: &Puzzle) -> Puzzle;
}

pub struct SwapPuzzleGenerator {}
impl PuzzleGenerator for SwapPuzzleGenerator {
    fn generate(puzzle: &Puzzle) -> Puzzle {
        let mut pieces = puzzle.pieces.clone();

        use rand::Rng;
        let mut rnd = rand::thread_rng();

        'retry: loop {
            let x = rnd.gen_range(0..puzzle.size);
            let y = rnd.gen_range(0..puzzle.size);
            let z = rnd.gen_range(0..puzzle.size);

            for a in 0..puzzle.size {
                if pieces[a].block.get(x, y, z) {
                    pieces[a].block.set(x, y, z, false);
                    let b = (a + rnd.gen_range(1..puzzle.size)) % puzzle.size;
                    pieces[b].block.set(x, y, z, true);
                    break;
                }
            }
            for piece in pieces.iter() {
                if !piece.block.is_connected() {
                    pieces = puzzle.pieces.clone();
                    continue 'retry;
                }
            }
            break;
        }
        Puzzle {
            pieces,
            size: puzzle.size,
            start: puzzle.start,
            space: puzzle.space,
        }
    }
}

#[derive(Clone)]
struct Piece {
    block: Cells,
    size: usize,
}

impl Piece {
    fn empty(size: usize) -> Piece {
        Piece {
            block: Cells::empty(size),
            size,
        }
    }
    fn from_str(size: usize, str: &str) -> Piece {
        let mut piece = Piece::empty(size);
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for c in str.chars() {
            match c {
                'X' => {
                    piece.block.set(x, y, z, true);
                }
                '.' => {
                    piece.block.set(x, y, z, false);
                }
                _ => continue,
            }
            x += 1;
            if x >= size {
                x = 0;
                z += 1;
            }
            if z >= size {
                z = 0;
                y += 1;
            }
        }
        piece
    }
}

use bitvec_simd::BitVec;
#[derive(Clone)]
struct Cells {
    bits: BitVec,
    size: usize,
}
impl Cells {
    fn empty(size: usize) -> Cells {
        let s = size * size * size;
        Cells {
            bits: BitVec::zeros(s),
            size,
        }
    }
    fn get(&self, x: usize, y: usize, z: usize) -> bool {
        let index = Cells::to_index(self.size, x, y, z);
        self.bits[index]
    }
    fn set(&mut self, x: usize, y: usize, z: usize, value: bool) {
        let index = Cells::to_index(self.size, x, y, z);
        self.bits.set(index, value);
    }
    fn to_index(size: usize, x: usize, y: usize, z: usize) -> usize {
        let index = x + y * size + z * size * size;
        index
    }
    fn from_index(size: usize, index: usize) -> (usize, usize, usize) {
        let x = index % size;
        let y = (index / size) % size;
        let z = index / size / size;
        (x, y, z)
    }
    fn to_str(&self) -> String {
        let mut s = String::new();
        for y in 0..self.size {
            s.push('"');
            for z in 0..self.size {
                for x in 0..self.size {
                    if self.get(x, y, z) {
                        s.push('x');
                    } else {
                        s.push('.');
                    }
                }
                if z < self.size - 1 {
                    s.push('|')
                };
            }
            s.push_str("\",\n");
        }
        s
    }
    fn boxed(&self) -> Cells {
        let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = self.bounding_box();
        let mut cells = Cells::empty(self.size);
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if min_x <= x
                        && x <= max_x
                        && min_y <= y
                        && y <= max_y
                        && min_z <= z
                        && z <= max_z
                    {
                        cells.set(x, y, z, true);
                    }
                }
            }
        }
        cells
    }
    fn bounding_box(&self) -> ((usize, usize, usize), (usize, usize, usize)) {
        let mut min_x = self.size;
        let mut min_y = self.size;
        let mut min_z = self.size;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.get(x, y, z) {
                        min_x = std::cmp::min(min_x, x);
                        min_y = std::cmp::min(min_y, y);
                        min_z = std::cmp::min(min_z, z);
                        max_x = std::cmp::max(max_x, x);
                        max_y = std::cmp::max(max_y, y);
                        max_z = std::cmp::max(max_z, z);
                    }
                }
            }
        }
        ((min_x, min_y, min_z), (max_x, max_y, max_z))
    }
    fn is_connected(&self) -> bool {
        let mut cells = Cells::empty(self.size);
        let mut queue = std::collections::VecDeque::new();
        'outer: for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.get(x, y, z) {
                        cells.set(x, y, z, true);
                        queue.push_back((x, y, z));
                        break 'outer;
                    }
                }
            }
        }
        while let Some((x, y, z)) = queue.pop_front() {
            for (dx, dy, dz) in D6 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                let nz = z as isize + dz;
                if nx < 0 || ny < 0 || nz < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let nz = nz as usize;
                if nx >= self.size || ny >= self.size || nz >= self.size {
                    continue;
                }
                if cells.get(nx, ny, nz) || !self.get(nx, ny, nz) {
                    continue;
                }
                cells.set(nx, ny, nz, true);
                queue.push_back((nx, ny, nz));
            }
        }
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.get(x, y, z) && !cells.get(x, y, z) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

const D6: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn piece_str() {
        let piece_a = Piece::from_str(
            3,
            "
        XXX|XXX|XXX
        ...|.X.|...
        ...|...|...",
        );
        println!("{}", piece_a.block.to_str());
        assert!(piece_a.block.is_connected());
        let piece_x = Piece::from_str(
            3,
            "
        ...|.X.;...
        ...|...;...
        ...|.X.;...",
        );
        println!("{}", piece_x.block.to_str());
        assert!(!piece_x.block.is_connected());
    }
    #[test]
    fn solver() {
        let piece_a = Piece::from_str(
            3,
            "
        XXX|XXX|XXX
        ...|.X.|...
        ...|...|...",
        );
        println!("{}", piece_a.block.to_str());
        let piece_b = Piece::from_str(
            3,
            "
        ...|...|...
        XXX|X.X|XXX
        ...|X..|...",
        );
        let piece_c = Piece::from_str(
            3,
            "
        ...|...|...
        ...|...|...
        XXX|.XX|XXX",
        );
        let puzzle = Puzzle {
            pieces: vec![piece_a, piece_b, piece_c],
            size: 3,
            start: 0,
            space: 6,
        };
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        println!("Shrink {:?}", result.shrink_move(result.moves(&puzzle)));
    }
    #[test]
    fn base_solve() {
        let puzzle = Puzzle::base(3);
        println!("{}", puzzle.pieces[0].block.to_str());
        assert!(puzzle.solve().ok);
    }
    #[test]
    fn puzzle_searcher() {
        let searcher = PuzzleSearcher::new(3, 10000);
        let puzzle = searcher.search::<SwapPuzzleGenerator>();
        println!("Found\n{}", puzzle.to_str());
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        let shrink = result.shrink_move(result.moves(&puzzle));
        println!("Shrink #{} {:?}", shrink.len(), shrink);
    }
}

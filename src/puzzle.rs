type V3 = (usize, usize, usize);
type V3I = (isize, isize, isize);
fn v3cube(x: usize) -> V3 {
    (x, x, x)
}
fn v3plus(a: V3, b: V3) -> V3 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

struct V3Iter {
    current: V3,
    end: V3,
}
impl V3Iter {
    fn new(v: V3) -> Self {
        V3Iter {
            current: (0, 0, 0),
            end: v,
        }
    }
    fn cube(x: usize) -> Self {
        Self::new(v3cube(x))
    }
}
impl Iterator for V3Iter {
    type Item = V3;
    fn next(&mut self) -> Option<Self::Item> {
        let (mut x, mut y, mut z) = self.current;
        let (ex, ey, ez) = self.end;
        x += 1;
        if x >= ex {
            x = 0;
            y += 1;
            if y >= ey {
                y = 0;
                z += 1;
                if z >= ez {
                    return None;
                }
            }
        }
        self.current = (x, y, z);
        return Some(self.current);
    }
}

#[derive(Clone)]
pub struct Puzzle {
    pieces: Vec<Piece>,
    size: usize,
    margin: usize,
    space: usize,
}

use std::collections::HashMap;
#[derive(Clone)]
pub struct SolveResult {
    pub ok: bool,
    step: Option<usize>,
    reached: HashMap<State, State>,
    end_state: Option<State>,
}

#[derive(Eq, Clone, Debug)]
struct State {
    indexes: Vec<Option<usize>>,
    shift: V3I,
}
impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for index in self.indexes.iter() {
            index.hash(state);
        }
    }
}
impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.indexes == other.indexes
    }
}

fn state_to_vec(size: usize, state: &State) -> Vec<Option<(usize, usize, usize)>> {
    state
        .indexes
        .iter()
        .map(|&x| x.map(|x| Cells::from_index(size, x)))
        .collect::<Vec<_>>()
}
fn state_to_str(size: usize, state: &State) -> String {
    let mut s = String::new();
    for (i, index) in state.indexes.iter().enumerate() {
        if let Some(index) = index {
            let (x, y, z) = Cells::from_index(size, *index);
            s.push_str(&format!("({}, {}, {}) ", x, y, z));
        } else {
            s.push_str("() ");
        }
    }
    s.push_str(&format!("shift {:?}", state.shift));
    s
}
impl Puzzle {
    fn init_state(&self) -> State {
        let init_pos = Cells::to_index(self.space, self.margin, self.margin, self.margin);
        State {
            indexes: vec![Some(init_pos); self.pieces.len()],
            shift: (0, 0, 0),
        }
    }
    pub fn solve(&self) -> SolveResult {
        let mut reached = HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.init_state(), 0, 0));
        while let Some((state, step, last_piece)) = queue.pop_front() {
            if self.is_solved(&state) {
                println!(
                    "solved step = {}, {:?},",
                    step,
                    state_to_str(self.space, &state)
                );
                return SolveResult {
                    ok: true,
                    step: Some(step),
                    reached,
                    end_state: Some(state),
                };
            }
            for (next_state, last_piece) in self.next_states(&state, last_piece) {
                if reached.contains_key(&next_state) {
                    continue;
                }
                reached.insert(next_state.clone(), state.clone());
                let removed_state = self.remove_pieces(&next_state);
                if reached.contains_key(&removed_state) {
                    continue;
                }
                reached.insert(removed_state.clone(), next_state.clone());
                println!(
                    "{:?} -> {:?} {:?}",
                    state_to_str(self.space, &state),
                    state_to_str(self.space, &next_state),
                    state_to_str(self.space, &removed_state)
                );
                queue.push_back((removed_state.clone(), step + 1, last_piece));
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
    fn is_solved(&self, state: &State) -> bool {
        state.indexes.iter().all(|pos| pos.is_none())
    }
    fn remove_pieces(&self, state: &State) -> State {
        let mut result = state.clone();
        let boxes = self.state_to_boxes(state);
        for k in 0..state.indexes.len() {
            if state.indexes[k].is_none() {
                continue;
            }
            let mut cells = Cells::empty(self.space);
            for o in 1..state.indexes.len() {
                let p = (o + k) % state.indexes.len();
                if let Some(piece) = &boxes[p] {
                    cells.or_inplace(piece)
                }
            }
            if !cells.overlap(boxes[k].as_ref().unwrap()) {
                result.indexes[k] = None
            }
        }
        result
    }
    fn next_states(&self, state: &State, last_piece: usize) -> Vec<(State, usize)> {
        let mut next_states = Vec::new();
        for c in 0..self.pieces.len() {
            if state.indexes[c].is_none() {
                continue;
            }
            let index = state.indexes[c].unwrap();
            let i = (last_piece + c) % self.pieces.len();
            let (x, y, z) = Cells::from_index(self.space, index);
            for (dx, dy, dz) in D6 {
                for s in 1..self.space {
                    let (nx, ny, nz) = (
                        x as isize + dx * s as isize,
                        y as isize + dy * s as isize,
                        z as isize + dz * s as isize,
                    );
                    if nx < 0 || ny < 0 || nz < 0 {
                        break;
                    }
                    let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);
                    if nx + self.size + self.margin >= self.space
                        || ny + self.size + self.margin >= self.space
                        || nz + self.size + self.margin >= self.space
                    {
                        break;
                    }
                    let mut next_state = state.clone();
                    let next_pos = Cells::to_index(self.space, nx, ny, nz);
                    next_state.indexes[i] = Some(next_pos);
                    if self.collides(&next_state) {
                        break;
                    }
                    let next_state = self.normalize_state(&next_state);
                    next_states.push((next_state, i));
                }
            }
        }
        next_states
    }
    fn state_to_cells(&self, state: &State) -> Vec<Option<Cells>> {
        state
            .indexes
            .iter()
            .enumerate()
            .map(|(i, index)| {
                index.map(|index| {
                    self.pieces[i]
                        .block
                        .shift_expand(self.space, Cells::from_index(self.space, index))
                })
            })
            .collect::<Vec<_>>()
    }
    fn state_to_boxes(&self, state: &State) -> Vec<Option<Cells>> {
        state
            .indexes
            .iter()
            .enumerate()
            .map(|(i, index)| {
                index.map(|index| {
                    self.pieces[i]
                        .block
                        .boxed()
                        .shift_expand(self.space, Cells::from_index(self.space, index))
                })
            })
            .collect::<Vec<_>>()
    }
    fn collides(&self, state: &State) -> bool {
        let mut cells = Cells::empty(self.space);
        let pieces = self.state_to_cells(state);
        for piece in pieces.iter() {
            if let Some(piece) = piece {
                if cells.overlap(piece) {
                    return true;
                }
                cells.or_inplace(piece);
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
            margin: size,
            space: size * 4,
        }
    }
    pub fn to_str(&self) -> String {
        let mut s = String::new();
        for (i, piece) in self.pieces.iter().enumerate() {
            s.push_str(&format!("#{}\n{}\n", i, piece.block.to_str()));
        }
        s
    }
    fn normalize_state(&self, state: &State) -> State {
        let mut state = state.clone();
        let (mut min_x, mut min_y, mut min_z) = (self.space, self.space, self.space);
        let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
        for i in 0..self.pieces.len() {
            if state.indexes[i].is_none() {
                continue;
            }
            let (x, y, z) = Cells::from_index(self.space, state.indexes[i].unwrap());
            let ((mx, my, mz), (ox, oy, oz)) = self.pieces[i].block.bounding_box();
            assert!(x + mx < self.space);
            assert!(y + my < self.space);
            assert!(z + mz < self.space);
            (min_x, min_y, min_z) = (
                std::cmp::min(min_x, x + mx),
                std::cmp::min(min_y, y + my),
                std::cmp::min(min_z, z + mz),
            );
            (max_x, max_y, max_z) = (
                std::cmp::max(max_x, x + ox),
                std::cmp::max(max_y, y + oy),
                std::cmp::max(max_z, z + oz),
            )
        }
        let (mut shift_x, mut shift_y, mut shift_z) = (
            self.margin as isize - min_x as isize,
            self.margin as isize - min_y as isize,
            self.margin as isize - min_z as isize,
        );
        if max_x as isize + shift_x >= self.space as isize {
            shift_x = 0;
        }
        if max_y as isize + shift_y >= self.space as isize {
            shift_y = 0;
        }
        if max_z as isize + shift_z >= self.space as isize {
            shift_z = 0;
        }
        for i in 0..self.pieces.len() {
            if state.indexes[i].is_none() {
                continue;
            }
            let (x, y, z) = Cells::from_index(self.space, state.indexes[i].unwrap());
            state.indexes[i] = Some(Cells::to_index(
                self.space,
                (x as isize + shift_x) as usize,
                (y as isize + shift_y) as usize,
                (z as isize + shift_z) as usize,
            ));
            let (nx, ny, nz) = Cells::from_index(self.space, state.indexes[i].unwrap());
            assert!(nx < self.space);
            assert!(ny < self.space);
            assert!(nz < self.space);
        }
        let (sx, sy, sz) = state.shift;
        state.shift = (sx + shift_x, sy + shift_y, sz + shift_z);
        state
    }
}
pub type Moves = Vec<Move>;
#[derive(Clone, Copy, Debug)]
pub enum Move {
    Shift(usize, V3I),
    Remove(usize, V3I),
}
impl SolveResult {
    pub fn moves(&self, puzzle: &Puzzle) -> Moves {
        let mut moves = Vec::new();
        let mut end_state = self.end_state.clone().unwrap();
        let init_state = puzzle.init_state();
        while end_state != init_state {
            let prev_state = self.reached.get(&end_state).unwrap();
            println!(
                "{} -> {}",
                state_to_str(puzzle.space, &prev_state),
                state_to_str(puzzle.space, &end_state)
            );
            for (i, (&pos, &prev_pos)) in end_state
                .indexes
                .iter()
                .zip(prev_state.indexes.iter())
                .enumerate()
            {
                if let Some(prev_pos) = prev_pos {
                    let (pax, pay, paz) = Cells::from_index(puzzle.space, prev_pos);
                    let (psx, psy, psz) = prev_state.shift;
                    let (px, py, pz) = (pax as isize - psx, pay as isize - psy, paz as isize - psz);
                    if let Some(pos) = pos {
                        let (ax, ay, az) = Cells::from_index(puzzle.space, pos);
                        let (sx, sy, sz) = end_state.shift;
                        let (x, y, z) = (ax as isize - sx, ay as isize - sy, az as isize - sz);
                        println!("#{}: {:?} -> {:?}", i, (px, py, pz), (x, y, z));
                        if (x, y, z) == (px, py, pz) {
                            continue;
                        }
                        moves.push(Move::Shift(
                            i,
                            (
                                x as isize - px as isize,
                                y as isize - py as isize,
                                z as isize - pz as isize,
                            ),
                        ));
                    } else {
                        moves.push(Move::Remove(i, (px, py, pz)))
                    }
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

        moves
    }
}

pub struct PuzzleSearcher {
    size: usize,
    tries: usize,
    stack: usize,
}
impl PuzzleSearcher {
    pub fn new(size: usize, tries: usize, stack: usize) -> PuzzleSearcher {
        PuzzleSearcher { size, tries, stack }
    }
    pub fn search<T: PuzzleGenerator>(&self) -> Puzzle {
        let base_puzzle = Puzzle::base(self.size);
        let mut puzzles = vec![(base_puzzle, 0); self.stack];
        let mut max_move = 0;
        for i in 0..self.tries {
            for k in 0..self.stack {
                let (puzzle, max_moves) = &puzzles[k];
                let new_puzzle = T::generate(&puzzle);
                let result = new_puzzle.solve();
                if result.ok {
                    let moves = result.shrink_move(result.moves(&new_puzzle)).len();
                    if max_move < moves {
                        puzzles[k] = (new_puzzle, moves);
                        max_move = moves;
                    }
                }
            }
            let best_puzzle = puzzles.iter().max_by_key(|p| p.1).unwrap();
            println!(
                "try #{} moves: #{} puzzle:{}",
                i,
                best_puzzle.1,
                best_puzzle.0.to_str()
            );
        }
        puzzles.iter().max_by_key(|p| p.1).unwrap().0.clone()
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
                if !piece.block.is_connected() || piece.block.count() == 0 {
                    pieces = puzzle.pieces.clone();
                    continue 'retry;
                }
            }
            break;
        }
        Puzzle {
            pieces,
            size: puzzle.size,
            margin: puzzle.margin,
            space: puzzle.space,
        }
    }
}

pub struct Swap5PuzzleGenerator {}
impl PuzzleGenerator for Swap5PuzzleGenerator {
    fn generate(puzzle: &Puzzle) -> Puzzle {
        let mut puzzle = puzzle.clone();
        for _ in 0..5 {
            puzzle = SwapPuzzleGenerator::generate(&puzzle);
        }
        puzzle
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
    fn getv(&self, v: V3) -> bool {
        self.get(v.0, v.1, v.2)
    }
    fn set(&mut self, x: usize, y: usize, z: usize, value: bool) {
        let index = Cells::to_index(self.size, x, y, z);
        self.bits.set(index, value);
    }
    fn setv(&mut self, v: V3, value: bool) {
        self.set(v.0, v.1, v.2, value)
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
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    cells.set(x, y, z, true);
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
    fn count(&self) -> usize {
        self.bits.count_ones()
    }
    fn or_inplace(&mut self, other: &Cells) {
        self.bits.or_inplace(&other.bits);
    }
    fn and_inplace(&mut self, other: &Cells) {
        self.bits.and_inplace(&other.bits);
    }
    fn overlap(&self, other: &Cells) -> bool {
        self.bits.and_cloned(&other.bits).any()
    }
    fn shift_expand(&self, space: usize, shift: V3) -> Cells {
        let mut res = Cells::empty(space);
        for d in V3Iter::cube(self.size) {
            let p = v3plus(d, shift);
            if self.getv(d) {
                res.setv(p, true)
            }
        }
        res
    }
}

const D6: [V3I; 6] = [
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
        XXX|...|...
        XXX|...|...
        XXX|...|...",
        );
        println!("{}", piece_a.block.to_str());
        let piece_b = Piece::from_str(
            3,
            "
        ...|XXX|...
        ...|XXX|.X.
        ...|XXX|...",
        );
        let piece_c = Piece::from_str(
            3,
            "
        ...|...|XXX
        ...|...|X.X
        ...|...|XXX",
        );
        let puzzle = Puzzle {
            pieces: vec![piece_a, piece_b, piece_c],
            size: 3,
            margin: 3,
            space: 12,
        };
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        println!("Shrink {:?}", result.shrink_move(result.moves(&puzzle)));
    }
    #[test]
    fn solver_cant() {
        let piece_a = Piece::from_str(
            3,
            "
            XX.|...|...
            .X.|...|...
            XXX|XX.|...",
        );
        println!("{}", piece_a.block.to_str());
        let piece_b = Piece::from_str(
            3,
            "
            ...|XX.|.X.
            X..|XXX|.X.
            ...|..X|...",
        );
        let piece_c = Piece::from_str(
            3,
            "
            ..X|..X|X.X
            ..X|...|X.X
            ...|...|XXX",
        );
        let puzzle = Puzzle {
            pieces: vec![piece_a, piece_b, piece_c],
            size: 3,
            margin: 3,
            space: 12,
        };
        let result = puzzle.solve();
        assert!(!result.ok);
    }
    #[test]
    fn solver_step() {
        let piece_a = Piece::from_str(
            3,
            "
            XXX|...|...
            .XX|.X.|...
            XX.|.X.|...",
        );
        println!("{}", piece_a.block.to_str());
        let piece_b = Piece::from_str(
            3,
            "
            ...|XXX|..X
            X..|X.X|..X
            ...|X..|X..",
        );
        let piece_c = Piece::from_str(
            3,
            "
            ...|...|XX.
            ...|...|XX.
            ..X|..X|.XX",
        );
        let puzzle = Puzzle {
            pieces: vec![piece_a, piece_b, piece_c],
            size: 3,
            margin: 6,
            space: 18,
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
        let searcher = PuzzleSearcher::new(3, 10, 2);
        let puzzle = searcher.search::<SwapPuzzleGenerator>();
        println!("Found\n{}", puzzle.to_str());
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        let shrink = result.shrink_move(result.moves(&puzzle));
        println!("Shrink #{} {:?}", shrink.len(), shrink);
    }
}

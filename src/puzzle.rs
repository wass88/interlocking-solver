use crate::cells::*;

#[derive(Clone)]
pub struct Puzzle {
    pub pieces: Vec<Piece>,
    pub size: usize,
    pub margin: usize,
    pub space: usize,
    pub reach_limit: Option<usize>,
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
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_piece = self.indexes.iter().filter(|x| x.is_some()).count();
        let other_piece = other.indexes.iter().filter(|x| x.is_some()).count();
        self_piece.cmp(&other_piece)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
        use std::cmp::Reverse;
        let mut reached = HashMap::new();
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(Reverse((self.init_state(), 0, 0)));
        while let Some(Reverse((state, step, last_piece))) = queue.pop() {
            if self.is_solved(&state) {
                return SolveResult {
                    ok: true,
                    step: Some(step),
                    reached,
                    end_state: Some(state),
                };
            }
            if let Some(reach_limit) = self.reach_limit {
                if reached.len() >= reach_limit {
                    println!("limit {} reached step={}", reached.len(), step);
                    break;
                }
            }
            for (next_state, last_piece) in self.next_states(&state, last_piece) {
                if reached.contains_key(&next_state) {
                    continue;
                }
                reached.insert(next_state.clone(), state.clone());
                let removed_state = self.remove_pieces(&next_state);
                if next_state != removed_state {
                    if reached.contains_key(&removed_state) {
                        continue;
                    }
                    reached.insert(removed_state.clone(), next_state.clone());
                }
                queue.push(Reverse((removed_state.clone(), step + 1, last_piece)));
            }
        }
        //println!("missing solution");
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
            let i = (last_piece + c) % self.pieces.len();
            let Some(index) = state.indexes[i] else {
                continue;
            };
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
    pub fn base(size: usize, num_pieces: usize, holes: usize) -> Puzzle {
        let mut pieces = vec![Piece::empty(size); num_pieces];
        for (x, y, z) in V3Iter::cube(size) {
            let pz = z;
            let py = if z % 2 == 0 { y } else { size - y - 1 };
            let px = if (y + z * size) % 2 == 0 {
                x
            } else {
                size - x - 1
            };
            let k = (pz * size + py) * size + px;
            if k < holes {
                continue;
            }
            let i = (k - holes) / ((size * size * size - holes + num_pieces - 1) / num_pieces);
            pieces[i].block.set(x, y, z, true);
        }
        Puzzle {
            pieces,
            size,
            margin: size,
            space: size * 5,
            reach_limit: Some(10000),
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

#[derive(Clone)]
pub struct Piece {
    pub block: Cells,
    pub size: usize,
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
    fn solver_normal() {
        let piece_a = Piece::from_str(
            3,
            "
        XXX|...|...
        XXX|...|...
        XXX|.X.|.X.",
        );
        println!("{}", piece_a.block.to_str());
        let piece_b = Piece::from_str(
            3,
            "
        ...|XXX|...
        ...|XXX|.X.
        ...|X.X|...",
        );
        let piece_c = Piece::from_str(
            3,
            "
        ...|...|XXX
        ...|...|X.X
        ...|...|X.X",
        );
        let puzzle = Puzzle {
            pieces: vec![piece_a, piece_b, piece_c],
            size: 3,
            margin: 3,
            space: 12,
            reach_limit: None,
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
            reach_limit: None,
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
            reach_limit: None,
        };
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        println!("Shrink {:?}", result.shrink_move(result.moves(&puzzle)));
    }
    #[test]
    fn test_base_puzzle() {
        let puzzle = Puzzle::base(3, 4, 1);
        for state in puzzle.next_states(&puzzle.init_state(), 0) {
            println!("!{:?}", state);
        }
        assert!(puzzle.solve().ok);
    }
    #[test]
    fn test_inbox_pieces() {
        let piece_a = Piece::from_str(
            4,
            "
....|....|....|....
....|....|..X.|....
....|....|..X.|....
....|....|....|....",
        );
        let piece_b = Piece::from_str(
            4,
            "
XXXX|X.XX|XXXX|XXXX
XXXX|X..X|X..X|XXXX
XXXX|X..X|X..X|XXXX
XXXX|XXXX|XXXX|XXX.",
        );
        let puzzle = Puzzle {
            pieces: vec![piece_a, piece_b],
            size: 4,
            margin: 4,
            space: 20,
            reach_limit: None,
        };
        let result = puzzle.solve();
        assert!(result.ok);
        println!("Moves {:?}", result.moves(&puzzle));
        assert!(result.moves(&puzzle).len() == 5);
    }
    #[test]
    fn test_rand_puzzle() {
        let piece_a = Piece::from_str(
            4,
            "..xx|..x.|....|....
        xx.x|.x..|....|....
        xxxx|...x|....|....
        xxxx|....|....|....",
        );
        let piece_b = Piece::from_str(
            4,
            "....|....|....|....
        ..x.|..x.|....|xxx.
        ....|xxx.|.x..|.x..
        ....|....|.x..|.x..",
        );
        let piece_c = Piece::from_str(
            4,
            "....|xx.x|x.xx|....
        ....|x..x|xxx.|....
        ....|....|x...|x...
        ....|....|....|....",
        );
        let piece_d = Piece::from_str(
            4,
            "....|....|....|....
        ....|....|....|....
        ....|....|....|...x
        ....|xxx.|x.x.|x.xx",
        );
        let piece_e = Piece::from_str(
            4,
            "....|....|.x..|xxxx
        ....|....|...x|...x
        ....|....|..xx|..x.
        ....|...x|...x|....",
        );
        let puzzle = Puzzle {
            pieces: vec![piece_a, piece_b, piece_c, piece_d, piece_e],
            size: 4,
            margin: 4,
            space: 4 * 4,
            reach_limit: Some(7),
        };
        let result = puzzle.solve();
        assert!(result.ok);
    }
    #[test]
    fn test_queue() {
        let mut queue = std::collections::BinaryHeap::new();
        let state = State {
            indexes: vec![Some(0), Some(1), Some(2)],
            shift: (0, 0, 0),
        };
        let removed_state = State {
            indexes: vec![Some(0), Some(1), None],
            shift: (0, 0, 0),
        };
        let other_state = State {
            indexes: vec![Some(0), Some(1), Some(3)],
            shift: (0, 0, 0),
        };
        use std::cmp::Reverse;
        queue.push(Reverse((state, 0)));
        queue.push(Reverse((removed_state, 1)));
        queue.push(Reverse((other_state, 2)));
        queue.clone().into_sorted_vec().iter().for_each(|state| {
            println!("in queue {:?}", state);
        });
        println!("{:?}", queue.pop());
    }
}

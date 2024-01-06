pub type V3 = (usize, usize, usize);
pub type V3I = (isize, isize, isize);
fn v3cube(x: usize) -> V3 {
    (x, x, x)
}
pub fn v3plus(a: V3, b: V3) -> V3 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

pub struct V3Iter {
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
    pub fn cube(x: usize) -> Self {
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

use bitvec_simd::BitVec;
#[derive(Clone, Debug)]
pub struct Cells {
    bits: BitVec,
    size: usize,
}
impl Cells {
    pub fn empty(size: usize) -> Cells {
        let s = size * size * size;
        Cells {
            bits: BitVec::zeros(s),
            size,
        }
    }
    pub fn get(&self, x: usize, y: usize, z: usize) -> bool {
        let index = Cells::to_index(self.size, x, y, z);
        self.bits[index]
    }
    pub fn getv(&self, v: V3) -> bool {
        self.get(v.0, v.1, v.2)
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: bool) {
        let index = Cells::to_index(self.size, x, y, z);
        self.bits.set(index, value);
    }
    pub fn setv(&mut self, v: V3, value: bool) {
        self.set(v.0, v.1, v.2, value)
    }
    pub fn to_index(size: usize, x: usize, y: usize, z: usize) -> usize {
        let index = x + y * size + z * size * size;
        index
    }
    pub fn from_index(size: usize, index: usize) -> (usize, usize, usize) {
        let x = index % size;
        let y = (index / size) % size;
        let z = index / size / size;
        (x, y, z)
    }
    pub fn to_str(&self) -> String {
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
    pub fn boxed(&self) -> Cells {
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
    pub fn bounding_box(&self) -> ((usize, usize, usize), (usize, usize, usize)) {
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
    pub fn is_connected(&self) -> bool {
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
    pub fn count(&self) -> usize {
        self.bits.count_ones()
    }
    pub fn or_inplace(&mut self, other: &Cells) {
        self.bits.or_inplace(&other.bits);
    }
    pub fn and_inplace(&mut self, other: &Cells) {
        self.bits.and_inplace(&other.bits);
    }
    pub fn overlap(&self, other: &Cells) -> bool {
        self.bits.and_cloned(&other.bits).any()
    }
    pub fn shift_expand(&self, space: usize, shift: V3) -> Cells {
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

pub const D6: [V3I; 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

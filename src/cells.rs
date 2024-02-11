use crate::iters::V3Iter;
use crate::v3::{V3, V3I};
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
    pub fn to_indexv(size: usize, v: V3) -> usize {
        Cells::to_index(size, v.0, v.1, v.2)
    }
    pub fn from_index(size: usize, index: usize) -> V3 {
        let x = index % size;
        let y = (index / size) % size;
        let z = index / size / size;
        V3(x, y, z)
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
        let mut count = 0;
        for x in V3Iter::cube(self.size) {
            if self.getv(x) {
                cells.setv(x, true);
                queue.push_back(x);
                break;
            }
        }
        while let Some(x) = queue.pop_front() {
            count += 1;
            for d in D6 {
                let Some(n) = (V3I::from(x) + d).into_v3_in(&V3::cube(self.size)) else {
                    continue;
                };
                if cells.getv(n) || !self.getv(n) {
                    continue;
                }
                cells.setv(n, true);
                queue.push_back(n);
            }
        }
        count == self.count()
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
            let p = d + shift;
            if self.getv(d) {
                res.setv(p, true)
            }
        }
        res
    }
}

pub const D6: [V3I; 6] = [
    V3I(1, 0, 0),
    V3I(-1, 0, 0),
    V3I(0, 1, 0),
    V3I(0, -1, 0),
    V3I(0, 0, 1),
    V3I(0, 0, -1),
];

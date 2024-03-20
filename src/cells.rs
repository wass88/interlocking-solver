use crate::iters::V3Iter;
use crate::v3::{V3, V3I};
use bitvec_simd::BitVec;
#[derive(Clone, Debug)]
pub struct Cells {
    bits: BitVec,
    pub size: usize,
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
    pub fn bounding_box(&self) -> (V3, V3) {
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
        (V3(min_x, min_y, min_z), V3(max_x, max_y, max_z))
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
}

pub const D6: [V3I; 6] = [
    V3I(1, 0, 0),
    V3I(-1, 0, 0),
    V3I(0, 1, 0),
    V3I(0, -1, 0),
    V3I(0, 0, 1),
    V3I(0, 0, -1),
];

#[derive(Clone, Debug)]
pub struct SparseCells {
    cells: Vec<V3>,
}

impl SparseCells {
    pub fn empty() -> SparseCells {
        SparseCells { cells: Vec::new() }
    }
    pub fn from_cells(cells: &Cells) -> SparseCells {
        let mut res = SparseCells::empty();
        for x in V3Iter::cube(cells.size) {
            if cells.getv(x) {
                res.cells.push(x);
            }
        }
        res
    }
    pub fn to_cells(&self, size: usize) -> Cells {
        let mut res = Cells::empty(size);
        for x in &self.cells {
            res.setv(*x, true);
        }
        res
    }
    pub fn iter(&self) -> std::slice::Iter<V3> {
        self.cells.iter()
    }
}

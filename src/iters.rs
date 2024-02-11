use crate::v3::V3;
use itertools::Itertools;

pub struct V3Iter {
    current: V3,
    end: V3,
}
impl V3Iter {
    pub fn new(v: V3) -> Self {
        V3Iter {
            current: V3(0, 0, 0),
            end: v,
        }
    }
    pub fn cube(x: usize) -> Self {
        Self::new(V3::cube(x))
    }
}
impl Iterator for V3Iter {
    type Item = V3;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        let V3(mut x, mut y, mut z) = self.current;
        let V3(ex, ey, ez) = self.end;
        if z >= ez {
            return None;
        }
        x += 1;
        if x >= ex {
            x = 0;
            y += 1;
            if y >= ey {
                y = 0;
                z += 1;
            }
        }
        self.current = V3(x, y, z);
        return Some(current);
    }
}

pub struct SubsetsIter {
    iter: Box<dyn Iterator<Item = Vec<usize>>>,
}

impl SubsetsIter {
    pub fn new(content: &[usize], take: usize) -> Self {
        let iter = Box::new(
            (1..=take)
                .flat_map(move |k| content.iter().cloned().combinations(k))
                .collect_vec()
                .into_iter(),
        );
        Self { iter }
    }
}
impl Iterator for SubsetsIter {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_v3_new() {
        let x = V3(1, 2, 3);
        let mut iter = super::V3Iter::new(V3(2, 2, 2));
        assert_eq!(iter.next(), Some(V3(0, 0, 0)));
        assert_eq!(iter.next(), Some(V3(1, 0, 0)));
        assert_eq!(iter.next(), Some(V3(0, 1, 0)));
        assert_eq!(iter.next(), Some(V3(1, 1, 0)));
        assert_eq!(iter.next(), Some(V3(0, 0, 1)));
        assert_eq!(iter.next(), Some(V3(1, 0, 1)));
        assert_eq!(iter.next(), Some(V3(0, 1, 1)));
        assert_eq!(iter.next(), Some(V3(1, 1, 1)));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_v3_cube() {
        let mut iter = super::V3Iter::cube(2);
        assert_eq!(iter.next(), Some(V3(0, 0, 0)));
        assert_eq!(iter.next(), Some(V3(1, 0, 0)));
        assert_eq!(iter.next(), Some(V3(0, 1, 0)));
        assert_eq!(iter.next(), Some(V3(1, 1, 0)));
        assert_eq!(iter.next(), Some(V3(0, 0, 1)));
        assert_eq!(iter.next(), Some(V3(1, 0, 1)));
        assert_eq!(iter.next(), Some(V3(0, 1, 1)));
        assert_eq!(iter.next(), Some(V3(1, 1, 1)));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_subsets() {
        let mut iter = super::SubsetsIter::new(&vec![1, 2, 3], 2);
        assert_eq!(iter.next(), Some(vec![1]));
        assert_eq!(iter.next(), Some(vec![2]));
        assert_eq!(iter.next(), Some(vec![3]));
        assert_eq!(iter.next(), Some(vec![1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 3]));
        assert_eq!(iter.next(), None);
    }
}

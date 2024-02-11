use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct V3(pub usize, pub usize, pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct V3I(pub isize, pub isize, pub isize);

impl V3 {
    pub fn cube(x: usize) -> Self {
        Self(x, x, x)
    }
}

impl Add for V3 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        V3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl Sub for V3 {
    type Output = V3;
    fn sub(self, other: V3) -> V3 {
        V3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl Mul<usize> for V3 {
    type Output = V3;
    fn mul(self, other: usize) -> V3 {
        V3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Add for V3I {
    type Output = V3I;
    fn add(self, other: V3I) -> V3I {
        V3I(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl Sub for V3I {
    type Output = V3I;
    fn sub(self, other: V3I) -> V3I {
        V3I(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl Mul<isize> for V3I {
    type Output = V3I;
    fn mul(self, other: isize) -> V3I {
        V3I(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl Mul<usize> for V3I {
    type Output = V3I;
    fn mul(self, other: usize) -> V3I {
        V3I(
            self.0 * other as isize,
            self.1 * other as isize,
            self.2 * other as isize,
        )
    }
}
impl V3I {
    pub fn into_v3_in(&self, outer: &V3) -> Option<V3> {
        let V3I(x, y, z) = self;
        let V3I(ox, oy, oz) = V3I::from(*outer);
        if 0 <= *x && *x < ox && 0 <= *y && *y < oy && 0 <= *z && *z < oz {
            Some(V3(*x as usize, *y as usize, *z as usize))
        } else {
            None
        }
    }
}

impl From<V3> for V3I {
    fn from(v: V3) -> V3I {
        V3I(v.0 as isize, v.1 as isize, v.2 as isize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_v3i_into_v3() {
        assert_eq!(V3I(0, 0, 0).into_v3_in(&V3(1, 1, 1)), Some(V3(0, 0, 0)));
        assert_eq!(V3I(1, 1, 1).into_v3_in(&V3(2, 2, 2)), Some(V3(1, 1, 1)));
        assert_eq!(V3I(1, 1, 1).into_v3_in(&V3(0, 2, 2)), None);
        assert_eq!(V3I(1, -1, 1).into_v3_in(&V3(2, 0, 2)), None);
    }
}

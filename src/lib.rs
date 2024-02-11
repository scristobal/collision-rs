use std::ops::Sub;

use wasm_bindgen::prelude::*;

// Point in a 2D space with coordinates (`x`,`y`)
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct P {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl P {
    pub fn new(x: i32, y: i32) -> P {
        P { x, y }
    }

    pub fn wedge(&self, rhs: &P) -> i32 {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Sub for P {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Segment in a 2D space, from `a` to `b`
#[wasm_bindgen]
pub struct L {
    a: P,
    b: P,
}

#[wasm_bindgen]
impl L {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> L {
        L {
            a: P::new(a, b),
            b: P::new(c, d),
        }
    }
}

// Axis aligned box, `l` = left, `t` = top, `r` = right, `b` = bottom
#[wasm_bindgen]
pub struct B {
    l: i32,
    t: i32,
    r: i32,
    b: i32,
}

#[wasm_bindgen]
impl B {
    pub fn new(l: i32, t: i32, r: i32, b: i32) -> B {
        B { l, t, r, b }
    }
}

// Computes the intersection between two boxes
#[wasm_bindgen]
pub fn intersect_box_box(n: &B, m: &B) -> bool {
    (n.l <= m.r) && (m.l <= n.r) && (n.t <= m.b) && (m.t <= n.b)
}

#[wasm_bindgen]
pub fn intersection_line_line(s: &L, t: &L) -> bool {
    let s_ab = s.b - s.a;
    let t_ab = t.b - t.a;

    s_ab.wedge(&(t.a - s.a)) * s_ab.wedge(&(t.b - s.a)) <= 0
        && t_ab.wedge(&(s.a - t.a)) * t_ab.wedge(&(s.b - t.a)) <= 0
}

#[cfg(test)]
mod intersect_box_box {
    use super::*;

    #[test]
    fn a_b_overlap() {
        let a = B::new(0, 0, 10, 10);
        let b = B::new(5, 5, 15, 15);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn a_b_no_overlap() {
        let a = B::new(0, 0, 10, 10);
        let b = B::new(15, 15, 20, 20);

        assert!(!intersect_box_box(&a, &b));
    }

    #[test]
    fn a_inside_b() {
        let a = B::new(0, 0, 10, 10);
        let b = B::new(2, 2, 8, 8);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn b_inside_a() {
        let a = B::new(2, 2, 8, 8);
        let b = B::new(0, 0, 10, 10);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn a_b_share_border() {
        let a = B::new(0, 0, 10, 10);
        let b = B::new(0, 10, 10, 20);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn a_b_share_corner() {
        let a = B::new(0, 0, 10, 10);
        let b = B::new(10, 10, 20, 20);

        assert!(intersect_box_box(&a, &b));
    }
}

#[cfg(test)]
mod intersect_line_line {
    use super::*;

    #[test]
    fn a_b_intersect() {
        let a = L::new(0, 0, 10, 10);
        let b = L::new(0, 10, 10, 0);

        assert!(intersection_line_line(&a, &b))
    }

    #[test]
    fn a_b_not_intersect() {
        let a = L::new(0, 0, 0, 10);
        let b = L::new(10, 0, 10, 10);

        assert!(!intersection_line_line(&a, &b))
    }

    #[test]
    fn a_b_collinear_touching() {
        let a = L::new(0, 0, 10, 10);
        let b = L::new(10, 10, 20, 20);

        assert!(intersection_line_line(&a, &b))
    }

    #[test]
    fn a_b_collinear_not_touching() {
        let a = L::new(0, 0, 10, 10);
        let b = L::new(11, 11, 20, 20);

        assert!(!intersection_line_line(&a, &b))
    }
}

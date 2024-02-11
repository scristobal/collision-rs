use std::ops::Sub;

use wasm_bindgen::prelude::*;

/**  Point in a 2D space with coordinates (`x`,`y`) */
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

/**  Segment in a 2D space, from `a` to `b` */
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

    pub fn vec(&self) -> P {
        self.b - self.a
    }
}

/** Axis aligned box, `l` = left, `t` = top, `r` = right, `b` = bottom */
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

    pub fn tl(&self) -> P {
        P::new(self.l, self.t)
    }

    pub fn tr(&self) -> P {
        P::new(self.r, self.t)
    }

    pub fn bl(&self) -> P {
        P::new(self.l, self.b)
    }

    pub fn br(&self) -> P {
        P::new(self.r, self.b)
    }
}

/** Returns true iff boxes overlap */
#[wasm_bindgen]
pub fn intersect_box_box(n: &B, m: &B) -> bool {
    //  intersection on  `x` projection
    if (n.l <= m.r) && (m.l <= n.r) {
        return true;
    }

    //  intersection on `y` projection
    if (n.t <= m.b) && (m.t <= n.b) {
        return true;
    }

    false
}

#[wasm_bindgen]
pub fn intersection_line_line(s: &L, t: &L) -> bool {
    let s_ab = s.vec();
    let t_ab = t.vec();

    //  both points of `t` are on the same side of `s`
    if s_ab.wedge(&(t.a - s.a)) * s_ab.wedge(&(t.b - s.a)) > 0 {
        return false;
    }

    //  both points of `s` are on the same side of `t`
    if t_ab.wedge(&(s.a - t.a)) * t_ab.wedge(&(s.b - t.a)) > 0 {
        return false;
    }

    // colinear case, check intersection on `x` projections
    (s.a.x.min(s.b.x) <= t.a.x.max(t.b.x)) && (t.a.x.min(t.b.x) <= s.a.x.max(s.b.x))
}

/** Returns true iff segment has any point inside box */
#[wasm_bindgen]
pub fn intersection_line_box(s: &L, b: &B) -> bool {
    let tl = b.tl().wedge(&s.vec());
    let tr = b.tr().wedge(&s.vec());
    let bl = b.bl().wedge(&s.vec());
    let br = b.br().wedge(&s.vec());

    // all four corners are on the same side of `s`
    if tl > 0 && tr > 0 && bl > 0 && br > 0 {
        println!("exit on 1st condition");
        return false;
    }

    if tl < 0 && tr < 0 && bl < 0 && br < 0 {
        println!("exit on 2nd condition");
        return false;
    }

    // check `x`` projections
    if s.a.x.max(s.b.x) < b.l || s.a.x.min(s.b.x) > b.r {
        println!("exit on 3rd condition");
        return false;
    }

    // check `y` projections
    if s.a.y.max(s.b.y) < b.b || s.a.y.min(s.b.y) > b.t {
        println!("exit on 4th condition");

        return false;
    }

    true
}

/** Returns true iff segments have any point in common */
#[cfg(test)]
mod intersect_box_box {
    use super::*;

    #[test]
    fn overlap() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(5, 15, 15, 5);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn no_overlap() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(15, 20, 20, 15);

        assert!(!intersect_box_box(&a, &b));
    }

    #[test]
    fn contain_outer() {
        let a = B::new(0, 10, 10, 1);
        let b = B::new(2, 8, 8, 2);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn contain_inner() {
        let a = B::new(2, 8, 8, 2);
        let b = B::new(0, 10, 10, 0);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn share_border() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(0, 20, 10, 10);

        assert!(intersect_box_box(&a, &b));
    }

    #[test]
    fn share_corner() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(10, 20, 20, 10);

        assert!(intersect_box_box(&a, &b));
    }
}

#[cfg(test)]
mod intersect_line_line {
    use super::*;

    #[test]
    fn intersect() {
        let a = L::new(0, 0, 10, 10);
        let b = L::new(0, 10, 10, 0);

        assert!(intersection_line_line(&a, &b))
    }

    #[test]
    fn not_intersect() {
        let a = L::new(0, 0, 0, 10);
        let b = L::new(10, 0, 10, 10);

        assert!(!intersection_line_line(&a, &b))
    }

    #[test]
    fn colinear_touching() {
        let a = L::new(0, 0, 10, 10);
        let b = L::new(10, 10, 20, 20);

        assert!(intersection_line_line(&a, &b))
    }

    #[test]
    fn colinear_overlap() {
        let a = L::new(0, 0, 10, 10);
        let b = L::new(5, 5, 20, 20);

        assert!(intersection_line_line(&a, &b))
    }

    #[test]
    fn colinear_not_touching() {
        let a = L::new(0, 0, 10, 10);
        let b = L::new(11, 11, 20, 20);

        assert!(!intersection_line_line(&a, &b))
    }
}

#[cfg(test)]
mod intersection_line_box {
    use super::*;

    #[test]
    fn intersect() {
        let s = L::new(0, 0, 10, 10);
        let b = B::new(5, 15, 15, 5);

        assert!(intersection_line_box(&s, &b));
    }

    #[test]
    fn no_intersect() {
        let s = L::new(0, 0, 10, 10);
        let b = B::new(15, 20, 20, 15);

        assert!(!intersection_line_box(&s, &b));
    }

    #[test]
    fn line_contained() {
        let s = L::new(5, 5, 10, 10);
        let b = B::new(0, 15, 15, 0);

        assert!(intersection_line_box(&s, &b));
    }
}

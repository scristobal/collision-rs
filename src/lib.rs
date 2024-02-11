mod test;

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
pub struct S {
    a: P,
    b: P,
}

#[wasm_bindgen]
impl S {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> S {
        S {
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
pub fn intersect_boxes(n: &B, m: &B) -> bool {
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

/** Returns true iff segments have any point in common */
#[wasm_bindgen]
pub fn intersection_segments(s: &S, t: &S) -> bool {
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
pub fn intersection_segment_box(s: &S, b: &B) -> bool {
    let tl = b.tl().wedge(&s.vec());
    let tr = b.tr().wedge(&s.vec());
    let bl = b.bl().wedge(&s.vec());
    let br = b.br().wedge(&s.vec());

    // all four corners are on the same side of `s`
    if (tl > 0 && tr > 0 && bl > 0 && br > 0) || (tl < 0 && tr < 0 && bl < 0 && br < 0) {
        return false;
    }

    // check `x`` projections
    if s.a.x.max(s.b.x) < b.l || s.a.x.min(s.b.x) > b.r {
        return false;
    }

    // check `y` projections
    if s.a.y.max(s.b.y) < b.b || s.a.y.min(s.b.y) > b.t {
        return false;
    }

    true
}

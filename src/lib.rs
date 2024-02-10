use glam::{IVec2, IVec4};

pub fn overlap(a: &IVec4, b: &IVec4) -> bool {
    (a.x <= b.z) && (b.x <= a.z) && (a.y <= b.w) && (b.y <= a.w)
}

pub fn intersection([a, b]: [&IVec2; 2], [c, d]: [&IVec2; 2]) -> bool {
    let ab = *b - *a;
    let cd = *d - *c;

    ab.perp_dot(*c - *a) * ab.perp_dot(*d - *a) <= 0
        && cd.perp_dot(*a - *c) * cd.perp_dot(*b - *c) <= 0
}


#[cfg(test)]
mod overlap {
    use super::*;

    #[test]
    fn a_b_overlap() {
        let a = IVec4::new(0, 0, 10, 10);
        let b = IVec4::new(5, 5, 15, 15);
        assert!(overlap(&a, &b));
    }

    #[test]
    fn a_b_no_overlap() {
        let a = IVec4::new(0, 0, 10, 10);
        let b = IVec4::new(15, 15, 20, 20);
        assert!(!overlap(&a, &b));
    }

    #[test]
    fn a_inside_b() {
        let a = IVec4::new(0, 0, 10, 10);
        let b = IVec4::new(2, 2, 8, 8);
        assert!(overlap(&a, &b));
    }

    #[test]
    fn b_inside_a() {
        let a = IVec4::new(0, 0, 10, 10);
        let b = IVec4::new(2, 2, 8, 8);

        assert!(overlap(&a, &b));
    }

    #[test]
    fn a_b_share_border() {
        let a = IVec4::new(0, 0, 10, 10);
        let b = IVec4::new(0, 10, 10, 20);

        assert!(overlap(&a, &b));
    }

    #[test]
    fn a_b_share_corner() {
        let a = IVec4::new(0, 0, 10, 10);
        let b = IVec4::new(10, 10, 20, 20);

        assert!(overlap(&a, &b));
    }
}

#[cfg(test)]
mod intersection {
    use super::*;

    #[test]
    fn a_b_intersect() {
        let a = [&IVec2::new(0, 0), &IVec2::new(10, 10)];
        let b = [&IVec2::new(0,10), &IVec2::new(10,0)];

        assert!(intersection(a,b))
    }


    #[test]
    fn a_b_not_intersect() {
        let a = [&IVec2::new(0, 0), &IVec2::new(0, 10)];
        let b = [&IVec2::new(10,0), &IVec2::new(10,10)];

        assert!(!intersection(a,b))
    }

    #[test]
    fn a_b_collinear() {
        let a = [&IVec2::new(0, 0), &IVec2::new(10, 10)];
        let b = [&IVec2::new(10,10), &IVec2::new(20,20)];

        assert!(intersection(a,b))
    }
}

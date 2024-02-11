#[cfg(test)]
mod intersect_box_box {
    use crate::*;

    #[test]
    fn overlap() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(5, 15, 15, 5);

        assert!(intersect_boxes(&a, &b));
    }

    #[test]
    fn no_overlap() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(15, 20, 20, 15);

        assert!(!intersect_boxes(&a, &b));
    }

    #[test]
    fn contain_outer() {
        let a = B::new(0, 10, 10, 1);
        let b = B::new(2, 8, 8, 2);

        assert!(intersect_boxes(&a, &b));
    }

    #[test]
    fn contain_inner() {
        let a = B::new(2, 8, 8, 2);
        let b = B::new(0, 10, 10, 0);

        assert!(intersect_boxes(&a, &b));
    }

    #[test]
    fn share_border() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(0, 20, 10, 10);

        assert!(intersect_boxes(&a, &b));
    }

    #[test]
    fn share_corner() {
        let a = B::new(0, 10, 10, 0);
        let b = B::new(10, 20, 20, 10);

        assert!(intersect_boxes(&a, &b));
    }
}

#[cfg(test)]
mod intersect_segments {
    use crate::*;

    #[test]
    fn intersect() {
        let a = S::new(0, 0, 10, 10);
        let b = S::new(0, 10, 10, 0);

        assert!(intersection_segments(&a, &b))
    }

    #[test]
    fn not_intersect() {
        let a = S::new(0, 0, 0, 10);
        let b = S::new(10, 0, 10, 10);

        assert!(!intersection_segments(&a, &b))
    }

    #[test]
    fn colinear_touching() {
        let a = S::new(0, 0, 10, 10);
        let b = S::new(10, 10, 20, 20);

        assert!(intersection_segments(&a, &b))
    }

    #[test]
    fn colinear_overlap() {
        let a = S::new(0, 0, 10, 10);
        let b = S::new(5, 5, 20, 20);

        assert!(intersection_segments(&a, &b))
    }

    #[test]
    fn colinear_not_touching() {
        let a = S::new(0, 0, 10, 10);
        let b = S::new(11, 11, 20, 20);

        assert!(!intersection_segments(&a, &b))
    }
}

#[cfg(test)]
mod intersection_segment_box {
    use crate::*;

    #[test]
    fn intersect() {
        let s = S::new(0, 0, 10, 10);
        let b = B::new(5, 15, 15, 5);

        assert!(intersection_segment_box(&s, &b));
    }

    #[test]
    fn no_intersect() {
        let s = S::new(0, 0, 10, 10);
        let b = B::new(15, 20, 20, 15);

        assert!(!intersection_segment_box(&s, &b));
    }

    #[test]
    fn segment_contained() {
        let s = S::new(5, 5, 10, 10);
        let b = B::new(0, 15, 15, 0);

        assert!(intersection_segment_box(&s, &b));
    }
}

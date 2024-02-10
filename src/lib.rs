use glam::IVec4;

pub fn overlap(a: &IVec4, b: &IVec4) -> bool {
    (a.x <= b.z) && (b.x <= a.z) && (a.y <= b.w) && (b.y <= a.w)  
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
        let a = IVec4::new(0,0,10,10);
        let b = IVec4::new(2,2,8,8);
        assert!(overlap(&a,&b));
    }
    
    #[test]
    fn b_inside_a() {

        let a = IVec4::new(0,0,10,10);
        let b = IVec4::new(2,2,8,8);
    
        assert!(overlap(&a,&b));

    }

    #[test]
    fn a_b_share_border() {
    
        let a = IVec4::new(0,0,10,10);
        let b = IVec4::new(0,10,10,20);
    
        assert!(overlap(&a,&b));

    }

    #[test]
    fn a_b_share_corner() {

        let a = IVec4::new(0,0,10,10);
        let b = IVec4::new(10,10,20,20);
    
        assert!(overlap(&a,&b));

    }
     
}

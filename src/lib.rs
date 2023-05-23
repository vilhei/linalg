mod matrix;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use matrix::*;
    #[test]
    fn creates_m4() {
        let m: M4<f64> = M4::new();
        println!("{m}");
    }

    #[test]
    fn creates_identity_matrix() {
        let m: M4<i64> = M4::eye();
        println!("{m}");
        assert_eq!(1, m[0][0]);
        assert_eq!(1, m[1][1]);
        assert_eq!(1, m[2][2]);
        assert_eq!(1, m[3][3]);
        assert_eq!(0, m[1][3]);
    }

    #[test]
    fn creates_diag_matrix() {
        let m = M4::diag(-1);
        println!("{m}");
        assert_eq!(-1, m[0][0]);
        assert_eq!(-1, m[1][1]);
        assert_eq!(-1, m[2][2]);
        assert_eq!(-1, m[3][3]);
        assert_eq!(0, m[1][3]);
    }

    #[test]
    fn defaults_to_f64_matrix() {
        let mut m = M4::new();
        m[0][0] = 1;
    }

    // #[test]
    // fn index_matrix() {
    //     let m: M4<u64> = M4::eye();
    // }

    #[test]
    fn should_transpose() {
        let m = M4::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        let t = m.transpose();

        println!("{t}");

        let expected = M4::from([
            [1, 5, 9, 13],
            [2, 6, 10, 14],
            [3, 7, 11, 15],
            [4, 8, 12, 16],
        ]);

        assert_eq!(expected, t);
        assert_eq!(m, t.transpose());
    }
}

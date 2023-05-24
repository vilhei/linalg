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

    #[test]
    fn adds_2_matrices() {
        let m1 = M4::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        let m2 = M4::from([[1; 4]; 4]);

        let expected1 = M4::from([
            [2, 3, 4, 5],
            [6, 7, 8, 9],
            [10, 11, 12, 13],
            [14, 15, 16, 17],
        ]);

        let result1 = m1.clone() + m2;
        assert_eq!(expected1, result1);

        let expected2 = M4::from([
            [1 - 1, 2 - 1, 3 - 1, 4 - 1],
            [5 - 1, 6 - 1, 7 - 1, 8 - 1],
            [9 - 1, 10 - 1, 11 - 1, 12 - 1],
            [13 - 1, 14 - 1, 15 - 1, 16 - 1],
        ]);
        let m2 = M4::from([[-1; 4]; 4]);
        let result2 = m1 + m2;
        assert_eq!(expected2, result2);
        // let result2 = m1 - m2;
    }

    #[test]
    fn get_returns_none() {
        let m: M4<f64> = M4::eye();
        let result = m.get(3, 4);
        assert!(result.is_none());
    }

    #[test]
    fn get_returns_some() {
        let m: M4<f64> = M4::eye();
        let result = m.get(3, 3);
        assert!(result.is_some());
    }

    #[test]
    fn get_returns_correct_value() {
        let m: M4<f64> = M4::eye();
        let result = m.get(3, 3);
        assert_eq!(*result.unwrap(), 1.0);
    }
    #[test]
    fn get_mut_allows_mutating() {
        let mut m: M4<f64> = M4::eye();
        let el = m.get_mut(3, 3);
        *el.unwrap() = -23.0;
        assert_eq!(m[3][3], -23.0);
    }
}

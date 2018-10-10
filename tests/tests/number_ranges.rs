use super::*;

pub static ARRAY: &str = r#"[0,1,2,3,4,5,6,7,8,9,10,11]"#;

mod dot_dot {
    use super::*;

    #[test]
    fn int_int_at() {
        let results = query("@[1..5]", ARRAY);

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn all_range() {
        let results = query("@[..]", ARRAY);

        assert_eq!(results.len(), 12);
    }

    #[test]
    fn int_int() {
        let results = query("[1..5]", ARRAY);

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn one_elem_at() {
        let results = query("@[5..5]", ARRAY);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn one_elem() {
        let results = query("[5..5]", ARRAY);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn _int_at() {
        let results = query("@[..5]", ARRAY);

        assert_eq!(results.len(), 6);
    }

    #[test]
    fn _int() {
        let results = query("[..5]", ARRAY);

        assert_eq!(results.len(), 6);
    }

    #[test]
    fn int_() {
        let results = query("[5..]", ARRAY);

        assert_eq!(results.len(), 7);
    }

    #[test]
    fn int_at() {
        let results = query("@[5..]", ARRAY);

        assert_eq!(results.len(), 7);
    }
}

// TODO ws more tests for colon notation
mod colon {
    use super::*;

    #[test]
    fn int_int_at() {
        let results = query("@[1:5]", ARRAY);

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn one_elem_at() {
        let results = query("@[5:5]", ARRAY);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn one_elem() {
        let results = query("[5:5]", ARRAY);

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn int_int() {
        let results = query("[1:5]", ARRAY);

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn _int_at() {
        let results = query("@[:5]", ARRAY);

        assert_eq!(results.len(), 6);
    }

    #[test]
    fn _int() {
        let results = query("[:5]", ARRAY);

        assert_eq!(results.len(), 6);
    }

    #[test]
    fn int_() {
        let results = query("[5:]", ARRAY);

        assert_eq!(results.len(), 7);
    }

    #[test]
    fn int_at() {
        let results = query("@[5:]", ARRAY);

        assert_eq!(results.len(), 7);
    }

    #[test]
    fn int_int_int() {
        let results = query("[0:2:10]", ARRAY);

        assert_eq!(results.len(), 6);
    }

    #[test]
    fn int_float_int() {
        let expected: Vec<f64> = vec![0.0, 2.0, 5.0,7.0, 10.0];

        let results = query("[0:2.5:10]", ARRAY);

        assert_eq!(results.len(), 5);

        results
            .iter()
            .zip(expected)
            .for_each(|(res, exp)| {
                assert_eq!(res.as_float(), exp)
            })

    }

    #[test]
    fn neg_start() {
        let expected: Vec<f64> = vec![6.0, 8.0, 10.0];

        let results = query("[-6:2:10]", ARRAY);

        assert_eq!(results.len(), 3);

        results
            .iter()
            .zip(expected)
            .for_each(|(res, exp)| {
                assert_eq!(res.as_float(), exp)
            })
    }

    #[test]
    fn neg_stop() {
        let expected: Vec<f64> = vec![0.0, 2.0, 4.0, 6.0];

        let results = query("[0:2:-6]", ARRAY);

        assert_eq!(results.len(), 4);

        results
            .iter()
            .zip(expected)
            .for_each(|(res, exp)| {
                assert_eq!(res.as_float(), exp)
            })
    }
}

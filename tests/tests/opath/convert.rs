use std::borrow::Cow;
use std::f64;

use super::*;

mod as_string {
    // TODO ws
}

mod as_boolean {
    use super::*;

    #[test]
    fn null() {
        let node = NodeRef::null();
        assert_eq!(node.as_boolean(), false)
    }

    #[test]
    fn boolean() {
        let node = NodeRef::boolean(true);
        assert_eq!(node.as_boolean(), true)
    }

    #[test]
    fn zero_integer() {
        let node = NodeRef::integer(0);
        assert_eq!(node.as_boolean(), false)
    }

    #[test]
    fn non_zero_integer() {
        let node = NodeRef::integer(12);
        assert_eq!(node.as_boolean(), true)
    }

    #[test]
    fn zero_float() {
        let node = NodeRef::float(0.0);
        assert_eq!(node.as_boolean(), false)
    }

    #[test]
    fn non_zero_float() {
        let node = NodeRef::float(1.1);
        assert_eq!(node.as_boolean(), true)
    }

    #[test]
    fn infinite_float() {
        let node = NodeRef::float(f64::INFINITY);
        assert_eq!(node.as_boolean(), false)
    }

    #[test]
    fn nan_float() {
        let node = NodeRef::float(f64::NAN);
        assert_eq!(node.as_boolean(), false)
    }

    #[test]
    fn empty_string() {
        let node = NodeRef::string(Cow::from(""));
        assert_eq!(node.as_boolean(), false)
    }

    #[test]
    fn non_empty_string() {
        let node = NodeRef::string(Cow::from("string"));
        assert_eq!(node.as_boolean(), true)
    }

    #[test]
    fn array() {
        let node = NodeRef::array(Elements::new());
        assert_eq!(node.as_boolean(), true)
    }

    #[test]
    fn object() {
        let node = NodeRef::object(Properties::new());
        assert_eq!(node.as_boolean(), true)
    }
}

mod as_float {
    use super::*;

    #[test]
    fn null() {
        let node = NodeRef::null();
        assert_eq!(node.as_float(), 0.0)
    }

    #[test]
    fn boolean() {
        let node = NodeRef::boolean(true);
        assert_eq!(node.as_float(), 1.0)
    }

    #[test]
    fn zero_integer() {
        let node = NodeRef::integer(0);
        assert_eq!(node.as_float(), 0.0)
    }

    #[test]
    fn non_zero_integer() {
        let node = NodeRef::integer(12);
        assert_eq!(node.as_float(), 12.0)
    }

    #[test]
    fn zero_float() {
        let node = NodeRef::float(0.0);
        assert_eq!(node.as_float(), 0.0)
    }

    #[test]
    fn non_zero_float() {
        let node = NodeRef::float(1.1);
        assert_eq!(node.as_float(), 1.1)
    }

    #[test]
    fn infinite_float() {
        let node = NodeRef::float(f64::INFINITY);
        assert!(f64::is_infinite(node.as_float()))
    }

    #[test]
    fn nan_float() {
        let node = NodeRef::float(f64::NAN);
        assert!(f64::is_nan(node.as_float()))
    }

    #[test]
    fn empty_string() {
        let node = NodeRef::string(Cow::from(""));
        assert!(f64::is_nan(node.as_float()))
    }

    #[test]
    fn non_empty_string() {
        let node = NodeRef::string(Cow::from("string"));
        assert!(f64::is_nan(node.as_float()))
    }

    #[test]
    fn number_string() {
        let node = NodeRef::string(Cow::from("12"));
        assert_eq!(node.as_float(), 12.0)
    }

    #[test]
    fn array() {
        let node = NodeRef::array(vec![NodeRef::null()]);
        assert!(f64::is_nan(node.as_float()))
    }
    #[test]
    fn empty_array() {
        let node = NodeRef::array(Elements::new());
        assert!(f64::is_nan(node.as_float()))
    }
    #[test]
    fn object() {
        let node = NodeRef::object(Properties::new());
        assert!(f64::is_nan(node.as_float()))
    }
}

mod as_integer {
    use super::*;

    #[test]
    fn null() {
        let node = NodeRef::null();
        assert_eq!(node.as_integer().unwrap(), 0)
    }

    #[test]
    fn boolean() {
        let node = NodeRef::boolean(true);
        assert_eq!(node.as_integer().unwrap(), 1)
    }

    #[test]
    fn zero_integer() {
        let node = NodeRef::integer(0);
        assert_eq!(node.as_integer().unwrap(), 0)
    }

    #[test]
    fn non_zero_integer() {
        let node = NodeRef::integer(12);
        assert_eq!(node.as_integer().unwrap(), 12)
    }

    #[test]
    fn zero_float() {
        let node = NodeRef::float(0.0);
        assert_eq!(node.as_integer().unwrap(), 0)
    }

    #[test]
    fn non_zero_float() {
        let node = NodeRef::float(1.1);
        assert_eq!(node.as_integer().unwrap(), 1)
    }

    #[test]
    #[should_panic]
    fn infinite_float() {
        let node = NodeRef::float(f64::INFINITY);
        node.as_integer().unwrap();
    }

    #[test]
    #[should_panic]
    fn nan_float() {
        let node = NodeRef::float(f64::NAN);
        node.as_integer().unwrap();
    }

    #[test]
    #[should_panic]
    fn empty_string() {
        let node = NodeRef::string(Cow::from(""));
        node.as_integer().unwrap();
    }

    #[test]
    #[should_panic]
    fn non_empty_string() {
        let node = NodeRef::string(Cow::from("string"));
        node.as_integer().unwrap();
    }

    #[test]
    fn number_string() {
        let node = NodeRef::string(Cow::from("12"));
        assert_eq!(node.as_integer().unwrap(), 12)
    }

    #[test]
    #[should_panic]
    fn array() {
        let node = NodeRef::array(Elements::new());
        node.as_integer().unwrap();
    }

    #[test]
    #[should_panic]
    fn object() {
        let node = NodeRef::object(Properties::new());
        node.as_integer().unwrap();
    }
}

use super::*;

#[test]
fn gt() {
    assert_bool_op("2>3", false);
    assert_bool_op("3>2", true);
}

#[test]
fn ge() {
    assert_bool_op("2 >= 3", false);
    assert_bool_op("2 >= 2", true);
    assert_bool_op("3 >= 2", true);
}

#[test]
fn lt() {
    assert_bool_op("2 < 3", true);
    assert_bool_op("3 < 2", false);
}

#[test]
fn le() {
    assert_bool_op("2 <= 3", true);
    assert_bool_op("3 <= 3", true);
    assert_bool_op("3 <= 2", false);
}

#[test]
fn eq() {
    assert_bool_op("2 == 3", false);
    assert_bool_op("3 == 3", true);
}

#[test]
fn ne() {
    assert_bool_op("2 != 3", true);
    assert_bool_op("3 != 3", false)
}

#[test]
fn starts_with() {
    assert_bool_op("'aaabbb' ^= 'aa'", true);
    assert_bool_op("'aaabbb' ^= 'bb'", false);
}

#[test]
fn contains() {
    assert_bool_op("'aaabbb' *= 'aa'", true);
    assert_bool_op("'aaabbb' *= 'ccc'", false);
}

#[test]
fn ends_with() {
    assert_bool_op("'aaabbb' $= 'aa'", false);
    assert_bool_op("'aaabbb' $= 'bb'", true);
}

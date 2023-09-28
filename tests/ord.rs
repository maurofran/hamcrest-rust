use hamcrest::*;
use hamcrest::prelude::*;

#[test]
pub fn compares_equal_to_success() {
    let fixture = 1;
    assert_that!(fixture, compares_equals_to(1));
}

#[test]
#[should_panic(expected = "Expected: a value equal to 2\n     but: was 1")]
pub fn compares_equal_to_failure() {
    let fixture = 1;
    assert_that!(fixture, compares_equals_to(2));
}

#[test]
pub fn greater_than_success() {
    let fixture = 3;
    assert_that!(fixture, greater_than(2));
}

#[test]
#[should_panic(expected = "Expected: a value greater than 4\n     but: was 3")]
pub fn greater_than_failure() {
    let fixture = 3;
    assert_that!(fixture, greater_than(4));
}

#[test]
#[should_panic(expected = "Expected: a value greater than 3\n     but: was 3")]
pub fn greater_than_failure_equal() {
    let fixture = 3;
    assert_that!(fixture, greater_than(3));
}

#[test]
pub fn greater_than_or_equal_to_success() {
    let fixture = 3;
    assert_that!(fixture, greater_than_or_equal_to(2));
}

#[test]
#[should_panic(expected = "Expected: a value greater than or equal to 4\n     but: was 3")]
pub fn greater_than_or_equal_to_failure() {
    let fixture = 3;
    assert_that!(fixture, greater_than_or_equal_to(4));
}

#[test]
pub fn greater_than_or_equal_to_success_equal() {
    let fixture = 3;
    assert_that!(fixture, greater_than_or_equal_to(3));
}

#[test]
pub fn less_than_success() {
    let fixture = 2;
    assert_that!(fixture, less_than(3));
}

#[test]
#[should_panic(expected = "Expected: a value less than 3\n     but: was 4")]
pub fn less_than_failure() {
    let fixture = 4;
    assert_that!(fixture, less_than(3));
}

#[test]
#[should_panic(expected = "Expected: a value less than 3\n     but: was 3")]
pub fn less_than_failure_equal() {
    let fixture = 3;
    assert_that!(fixture, less_than(3));
}

#[test]
pub fn less_than_or_equal_to_success() {
    let fixture = 2;
    assert_that!(fixture, less_than_or_equal_to(3));
}

#[test]
#[should_panic(expected = "Expected: a value less than or equal to 3\n     but: was 4")]
pub fn less_than_or_equal_to_failure() {
    let fixture = 4;
    assert_that!(fixture, less_than_or_equal_to(3));
}

#[test]
pub fn less_than_or_equal_to_success_equal() {
    let fixture = 3;
    assert_that!(fixture, less_than_or_equal_to(3));
}
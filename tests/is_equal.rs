use hamcrest::assert_that;
use hamcrest::prelude::*;

#[test]
fn test_assert_equals_success() {
    assert_that!(1, equal_to(1))
}

#[test]
#[should_panic(expected = "Expected: 2\n     but: was 1")]
fn test_assert_equals_fail() {
    assert_that!(1, equal_to(2))
}

#[test]
fn test_assert_is_equals_success() {
    assert_that!(1, is(equal_to(1)))
}

#[test]
#[should_panic(expected = "Expected: is 2\n     but: was 1")]
fn test_assert_is_equals_fail() {
    assert_that!(1, is(equal_to(2)))
}

#[test]
fn test_assert_not_equals_success() {
    assert_that!(1, not(equal_to(2)))
}

#[test]
#[should_panic(expected = "Expected: not 1\n     but: was 1")]
fn test_assert_not_equals_fail() {
    assert_that!(1, not(equal_to(1)))
}
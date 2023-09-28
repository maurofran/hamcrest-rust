use hamcrest::prelude::*;
use hamcrest::*;

#[test]
fn test_one_matcher_success() {
    assert_that!(1, any_of!(equal_to(1)));
}

#[test]
#[should_panic(expected = "Expected: (2)\n     but: was 1")]
fn test_one_matcher_failure() {
    assert_that!(1, any_of!(equal_to(2)));
}

#[test]
fn test_two_matchers_first_success() {
    assert_that!(2, any_of!(equal_to(2), equal_to(3)));
}

#[test]
fn test_two_matchers_second_success() {
    assert_that!(3, any_of!(equal_to(2), equal_to(3)));
}

#[test]
#[should_panic(expected = "Expected: (2 or 3)\n     but: was 1")]
fn test_two_matchers_fails() {
    assert_that!(1, any_of!(equal_to(2), equal_to(3)));
}
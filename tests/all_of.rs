use hamcrest::prelude::*;
use hamcrest::*;

#[test]
fn test_one_matcher_success() {
    assert_that!(1, all_of!(equal_to(1)));
}

#[test]
#[should_panic(expected = "Expected: (2)\n     but: (2) was 1")]
fn test_one_matcher_failure() {
    assert_that!(1, all_of!(equal_to(2)));
}

#[test]
fn test_two_matchers_success() {
    assert_that!(1, all_of!(not(equal_to(2)), not(equal_to(3))));
}

#[test]
#[should_panic(expected = "Expected: (not 2 and not 3)\n     but: (not 2 and not 3) was 2")]
fn test_two_matchers_fails_first() {
    assert_that!(2, all_of!(not(equal_to(2)), not(equal_to(3))));
}

#[test]
#[should_panic(expected = "Expected: (not 2 and not 3)\n     but: (not 2 and not 3) was 3")]
fn test_two_matchers_fails_second() {
    assert_that!(3, all_of!(not(equal_to(2)), not(equal_to(3))));
}
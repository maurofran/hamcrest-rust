use hamcrest::prelude::*;
use hamcrest::*;

#[test]
pub fn test_has_exact_length_success() {
    assert_that!("foo".to_string(), has_exact_length(3));
}

#[test]
#[should_panic(expected = "Expected: a string with length 4\n     but: a string with length was 3")]
pub fn test_has_exact_length_failure() {
    assert_that!("foo".to_string(), has_exact_length(4));
}

#[test]
pub fn test_has_length_success() {
    assert_that!("foo".to_string(), has_length(less_than_or_equal_to(4)));
}

#[test]
#[should_panic(expected = "Expected: a string with length a value less than or equal to 2\n     but: a string with length was 3")]
pub fn test_has_length_failure() {
    assert_that!("foo".to_string(), has_length(less_than_or_equal_to(2)));
}
use hamcrest::*;
use hamcrest::prelude::*;

#[test]
#[should_panic(expected = "Expected: a number\n     but: was 1")]
fn test_described_as() {
    assert_that!(1, described_as("a number".to_string(), equal_to(2)));
}

#[test]
#[should_panic(expected = "Expected: a number (3)\n     but: was 1")]
fn test_described_as_with_values() {
    assert_that!(1, described_as(format!("{} ({})", "a number", 3), equal_to(2)));
}
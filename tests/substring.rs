use hamcrest::*;
use hamcrest::prelude::*;

#[test]
pub fn test_contains_string_success() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, contains_string("baz"));
}

#[test]
pub fn test_contains_string_ignoring_case_success() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, contains_string("BAR").ignoring_case());
}

#[test]
#[should_panic(expected = "Expected: a string containing \"123\"\n     but: was foobarbaz")]
pub fn test_contains_string_failure() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, contains_string("123"));
}

#[test]
pub fn test_starts_with_success() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, starts_with("foo"));
}

#[test]
pub fn test_starts_with_ignoring_case_success() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, starts_with("FOO").ignoring_case());
}

#[test]
#[should_panic(expected = "Expected: a string starting with \"123\"\n     but: was foobarbaz")]
pub fn test_starts_with_failure() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, starts_with("123"));
}


#[test]
pub fn test_ends_with_success() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, ends_with("baz"));
}

#[test]
pub fn test_ends_with_ignoring_case_success() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, ends_with("BAZ").ignoring_case());
}

#[test]
#[should_panic(expected = "Expected: a string ending with \"123\"\n     but: was foobarbaz")]
pub fn test_ends_with_failure() {
    let fixture = "foobarbaz".to_string();
    assert_that!(fixture, ends_with("123"));
}
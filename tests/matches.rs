use regex::Regex;

use hamcrest::*;
use hamcrest::prelude::*;

#[test]
pub fn test_matches_regex_success() {
    let regex = Regex::new("[a-z]+").unwrap();
    let fixture = String::from("abc");
    assert_that!(fixture, matches_regexp(regex));
}

#[test]
#[should_panic(expected = "Expected: a string matching the pattern /[a-z]+/\n     but: the string was \"123\"")]
pub fn test_matches_regex_failure() {
    let regex = Regex::new("[a-z]+").unwrap();
    let fixture = String::from("123");
    assert_that!(fixture, matches_regexp(regex));
}

#[test]
pub fn test_matches_pattern_success() {
    let fixture = String::from("abc");
    assert_that!(fixture, matches_pattern("[a-z]+"));
}

#[test]
#[should_panic(expected = "Expected: a string matching the pattern /[a-z]+/\n     but: the string was \"123\"")]
pub fn test_matches_pattern_failure() {
    let fixture = String::from("123");
    assert_that!(fixture, matches_pattern("[a-z]+"));
}

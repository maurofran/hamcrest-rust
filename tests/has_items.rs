use hamcrest::prelude::*;
use hamcrest::*;

#[test]
fn test_has_items_success() {
    let fixture = vec![1, 2, 3];
    assert_that!(fixture, has_item(equal_to(1)));
}

#[test]
fn test_has_items_multiple_success() {
    let fixture = vec![1, 2, 3];
    assert_that!(fixture, has_item(any_of!(equal_to(1), equal_to(4))));
}

#[test]
#[should_panic(expected = "Expected: a collection containing 4\n     but: mismatches where: [was 1, was 2, was 3]")]
fn test_has_items_failure() {
    let fixture = vec![1, 2, 3];
    assert_that!(fixture, has_item(equal_to(4)));
}

#[test]
#[should_panic(expected = "Expected: a collection containing (1 and 4)\n     but: mismatches where: [(1 and 4) was 1, (1 and 4) was 2, (1 and 4) was 3]")]
fn test_has_items_multiple_failure() {
    let fixture = vec![1, 2, 3];
    assert_that!(fixture, has_item(all_of!(equal_to(1), equal_to(4))));
}
use hamcrest::prelude::*;
use hamcrest::*;

#[test]
fn test_every_items_success() {
    let fixture = vec![1, 1, 1];
    assert_that!(fixture, every_item(equal_to(1)));
}

#[test]
#[should_panic(expected = "Expected: every item is 1\n     but: an item was 2")]
fn test_every_item_failure() {
    let fixture = vec![1, 2, 3];
    assert_that!(fixture, every_item(equal_to(1)));
}
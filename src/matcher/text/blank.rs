use crate::description::{Description, SelfDescribing};
use crate::matcher::Matcher;

pub struct BlankMatcher;

impl Matcher<String> for BlankMatcher {
    fn matches(&self, value: &String) -> bool {
        value.trim().is_empty()
    }
}

impl SelfDescribing for BlankMatcher {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("a blank string");
    }
}

pub fn blank() -> BlankMatcher {
    BlankMatcher
}
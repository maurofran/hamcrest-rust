use crate::description::Description;
use crate::matcher::Matcher;
use crate::prelude::SelfDescribing;

pub struct IsEqual<T: PartialEq> {
    expected: T,
}

impl<T: PartialEq + SelfDescribing> Matcher<T> for IsEqual<T> {
    fn matches(&self, value: &T) -> bool {
        return value == &self.expected;
    }
}

impl<T: PartialEq + SelfDescribing> SelfDescribing for IsEqual<T> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_description_of(&self.expected);
    }
}

pub fn equal_to<T: PartialEq>(expected: T) -> IsEqual<T> {
    IsEqual { expected }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn matches_match() {
        let fixture = equal_to("foo");
        assert_eq!(fixture.matches(&"foo"), true);
    }

    #[test]
    pub fn matches_not_match() {
        let fixture = equal_to("foo");
        assert_eq!(fixture.matches(&"bar"), false);
    }
}
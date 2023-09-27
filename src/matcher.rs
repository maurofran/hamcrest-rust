mod core;

pub use core::*;

use std::fmt;

use crate::description::{Description, SelfDescribing};

/// A matcher over acceptable values.
/// A matcher is able to describe itself to give feedback when it fails.
pub trait Matcher<T: fmt::Display + Sized>: SelfDescribing {
    /// Evaluates the matcher for argument `value`.
    ///
    /// # Arguments
    /// * `value` - The value to evaluate the matcher against.
    fn matches(&self, value: &T) -> bool;

    /// Generate a description of why the matcher has not accepted the item.
    /// The description will be part of a larger description of why a matching failed, so it should
    /// be concise.
    /// This method assumes that [matches]> is false, but will not check this.
    ///
    /// # Arguments
    /// * `value` - The value to evaluate the matcher against.
    /// * `description` - The description of why the matcher has not accepted the item.
    fn describe_mismatch<D>(&self, value: &T, description: &mut D) where D: Description {
        description.append_text("was ").append_description_of(value);
    }
}

/// Utility class for writing one off matchers.
///
pub struct CustomMatcher<'a, T: fmt::Display> {
    matcher: fn(&T) -> bool,
    description: &'a str,
}

impl<'a, T: fmt::Display> CustomMatcher<'a, T> {
    /// Creates a new matcher with the given matcher function and description.
    pub fn new(matcher: fn(&T) -> bool, description: &'a str) -> Self {
        CustomMatcher {
            matcher,
            description,
        }
    }
}

impl<'a, T: fmt::Display> Matcher<T> for CustomMatcher<'a, T> {
    fn matches(&self, value: &T) -> bool {
        (self.matcher)(value)
    }

    fn describe_mismatch<D>(&self, _value: &T, description: &mut D) where D: Description {
        description.append_text(self.description);
    }
}

impl<'a, T: fmt::Display> SelfDescribing for CustomMatcher<'a, T> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text(self.description);
        return;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::prelude::StringDescription;
    use super::*;

    #[test]
    pub fn custom_matcher_matches() {
        let fixture: CustomMatcher<String> = CustomMatcher::new(|value| value.as_str() == "foo", "baz");
        assert!(fixture.matches(&"foo".to_string()));
    }

    #[test]
    pub fn custom_matcher_doesnt_matches() {
        let fixture: CustomMatcher<String> = CustomMatcher::new(|value| value.as_str() == "foo", "baz");
        assert!(!fixture.matches(&"bar".to_string()));
    }

    #[test]
    pub fn custom_matcher_describe_mismatch() {
        let fixture: CustomMatcher<String> = CustomMatcher::new(|value| value.as_str() == "foo", "baz");
        let mut description = StringDescription::new();
        fixture.describe_mismatch(&"bar".to_string(), &mut description);
        assert_eq!(description.to_string(), "baz");
    }
}
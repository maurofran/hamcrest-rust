pub use core::all_of::all_of;
pub use core::any_of::any_of;
pub use core::described_as::described_as;
pub use core::every::every_item;
pub use core::has_items::has_item;
pub use core::is::is;
pub use core::is_equal::equal_to;
pub use core::is_not::not;
pub use core::matches::{matches_pattern, matches_regexp};
pub use core::substring::{contains_string, ends_with, starts_with};
pub use ord::{compares_equals_to, greater_than, greater_than_or_equal_to, less_than, less_than_or_equal_to};
pub use text::{has_exact_length, has_length};

use crate::description::{Description, SelfDescribing};

mod core;
mod ord;
mod text;

/// A matcher over acceptable values.
/// A matcher is able to describe itself to give feedback when it fails.
pub trait Matcher<T: SelfDescribing + Sized>: SelfDescribing {
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

pub trait DiagnosingMatcher<T: SelfDescribing + Sized> {
    fn matches_describing<D>(&self, value: &T, description: &mut D) -> bool where D: Description;
}

/// Utility class for writing one off matchers.
///
pub struct CustomMatcher<'a, T: SelfDescribing> {
    matcher: fn(&T) -> bool,
    description: &'a str,
}

impl<'a, T: SelfDescribing> CustomMatcher<'a, T> {
    /// Creates a new matcher with the given matcher function and description.
    pub fn new(matcher: fn(&T) -> bool, description: &'a str) -> Self {
        CustomMatcher {
            matcher,
            description,
        }
    }
}

impl<'a, T: SelfDescribing> Matcher<T> for CustomMatcher<'a, T> {
    fn matches(&self, value: &T) -> bool {
        (self.matcher)(value)
    }

    fn describe_mismatch<D>(&self, _value: &T, description: &mut D) where D: Description {
        description.append_text(self.description);
    }
}

impl<'a, T: SelfDescribing> SelfDescribing for CustomMatcher<'a, T> {
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
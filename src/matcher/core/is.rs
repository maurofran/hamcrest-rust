use std::fmt;
use std::marker::PhantomData;

use crate::description::Description;
use crate::matcher::Matcher;
use crate::prelude::SelfDescribing;

pub struct IsMatcher<T: fmt::Display, M: Matcher<T>> {
    matcher: M,
    marker: PhantomData<T>
}

impl<T: fmt::Display, M: Matcher<T>> Matcher<T> for IsMatcher<T, M> {
    fn matches(&self, value: &T) -> bool {
        self.matcher.matches(value)
    }

    fn describe_mismatch<D>(&self, value: &T, description: &mut D) where D: Description {
        self.matcher.describe_mismatch(value, description)
    }
}

impl<T: fmt::Display, M: Matcher<T>> SelfDescribing for IsMatcher<T, M> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("is ").append_description_of(&self.matcher);
    }
}

/// Factory function for creating an [IsMatcher].
pub fn is<T: fmt::Display, M: Matcher<T>>(matcher: M) -> IsMatcher<T, M> {
    IsMatcher { matcher, marker: PhantomData }
}

#[cfg(test)]
pub mod tests {
    use crate::matcher::CustomMatcher;
    use crate::prelude::StringDescription;
    pub use super::*;

    fn make_matcher<M>() -> CustomMatcher<'static, String> {
        return CustomMatcher::new(|s: &String| s.len() > 0, "string is not empty");
    }

    #[test]
    pub fn is_matches_delegates() {
        let matcher = make_matcher::<CustomMatcher<'static, String>>();
        assert_eq!(is(matcher).matches(&"foo".to_string()), true);
    }

    #[test]
    pub fn is_matches_describe_mismatch() {
        let matcher = make_matcher::<CustomMatcher<'static, String>>();
        let mut description = StringDescription::new();
        is(matcher).describe_to(&mut description);
        assert_eq!(description.to_string(), "is string is not empty");
    }
}
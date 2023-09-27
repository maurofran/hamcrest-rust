use std::fmt;
use std::marker::PhantomData;

use crate::description::{Description, SelfDescribing};
use crate::matcher::Matcher;

pub struct IsNotMatcher<T: fmt::Display, M: Matcher<T>> {
    matcher: M,
    marker: PhantomData<T>,
}

impl<T: fmt::Display, M: Matcher<T>> Matcher<T> for IsNotMatcher<T, M> {
    fn matches(&self, value: &T) -> bool {
        !self.matcher.matches(value)
    }
}

impl<T: fmt::Display, M: Matcher<T>> SelfDescribing for IsNotMatcher<T, M> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("not ").append_description_of(&self.matcher);
    }
}

pub fn not<T: fmt::Display, M: Matcher<T>>(matcher: M) -> IsNotMatcher<T, M> {
    IsNotMatcher { matcher, marker: PhantomData }
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
        assert_eq!(not(matcher).matches(&"".to_string()), true);
    }

    #[test]
    pub fn is_matches_describe_mismatch() {
        let matcher = make_matcher::<CustomMatcher<'static, String>>();
        let mut description = StringDescription::new();
        not(matcher).describe_to(&mut description);
        assert_eq!(description.to_string(), "not string is not empty");
    }
}
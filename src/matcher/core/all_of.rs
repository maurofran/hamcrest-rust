use std::fmt;
use std::marker::PhantomData;

use crate::description::{Description, NoDescription, SelfDescribing};
use crate::matcher::{DiagnosingMatcher, Matcher};

pub struct AllOf<T: fmt::Display, M: Matcher<T>> {
    matchers: Vec<M>,
    marker: PhantomData<T>,
}

impl<T: fmt::Display, M: Matcher<T>> DiagnosingMatcher<T> for AllOf<T, M> {
    fn matches_describing<D>(&self, value: &T, description: &mut D) -> bool where D: Description {
        for matcher in &self.matchers {
            if !matcher.matches(value) {
                description.append_description_of(self).append_text(" ");
                matcher.describe_mismatch(value, description);
                return false;
            }
        }
        true
    }
}

impl<T: fmt::Display, M: Matcher<T>> Matcher<T> for AllOf<T, M> {
    fn matches(&self, value: &T) -> bool {
        let mut description = NoDescription;
        self.matches_describing(value, &mut description)
    }

    fn describe_mismatch<D>(&self, value: &T, description: &mut D) where D: Description {
        self.matches_describing(value, description);
    }
}

impl<T: fmt::Display, M: Matcher<T>> SelfDescribing for AllOf<T, M> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("(");
        for (i, matcher) in self.matchers.iter().enumerate() {
            if i > 0 {
                description.append_text(" and ");
            }
            matcher.describe_to(description);
        }
        description.append_text(")");
    }
}

/// Factory function for creating a matcher that matches if all of the given matchers match.
pub fn all_of<T: fmt::Display, M: Matcher<T>>(matchers: Vec<M>) -> AllOf<T, M> {
    AllOf { matchers, marker: PhantomData }
}

#[macro_export]
macro_rules! all_of {
    ($($matcher:expr),+$(,)?) => {{
        all_of(vec![$($matcher),+])
    }}
}
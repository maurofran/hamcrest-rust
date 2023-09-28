use std::marker::PhantomData;

use crate::description::{Description, SelfDescribing};
use crate::matcher::Matcher;

pub struct AnyOf<T: SelfDescribing, M: Matcher<T>> {
    matchers: Vec<M>,
    marker: PhantomData<T>,
}

impl<T: SelfDescribing, M: Matcher<T>> Matcher<T> for AnyOf<T, M> {
    fn matches(&self, value: &T) -> bool {
        for matcher in &self.matchers {
            if matcher.matches(value) {
                return true;
            }
        }
        return false;
    }
}

impl<T: SelfDescribing, M: Matcher<T>> SelfDescribing for AnyOf<T, M> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("(");
        for (i, matcher) in self.matchers.iter().enumerate() {
            if i > 0 {
                description.append_text(" or ");
            }
            matcher.describe_to(description);
        }
        description.append_text(")");
    }
}

pub fn any_of<T: SelfDescribing, M: Matcher<T>>(matchers: Vec<M>) -> AnyOf<T, M> {
    AnyOf { matchers, marker: PhantomData }
}

#[macro_export]
macro_rules! any_of {
    ($($matcher:expr),*) => {
        any_of(vec![$($matcher),*])
    };
}
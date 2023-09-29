use std::marker::PhantomData;

use crate::description::Description;
use crate::matcher::Matcher;
use crate::prelude::SelfDescribing;

pub struct DescribedAs<M, T> where M: Matcher<T>, T: SelfDescribing + Sized {
    matcher: M,
    message: String,
    marker: PhantomData<T>,
}

impl<M, T> Matcher<T> for DescribedAs<M, T> where M: Matcher<T>, T: SelfDescribing + Sized {
    fn matches(&self, value: &T) -> bool {
        self.matcher.matches(value)
    }

    fn describe_mismatch<D>(&self, value: &T, description: &mut D) where D: Description {
        self.matcher.describe_mismatch(value, description)
    }
}

impl<M, T> SelfDescribing for DescribedAs<M, T> where M: Matcher<T>, T: SelfDescribing + Sized {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text(self.message.as_str());
    }
}

pub fn described_as<M, T>(message: String, matcher: M) -> DescribedAs<M, T> where M: Matcher<T>, T: SelfDescribing + Sized {
    DescribedAs { matcher, message, marker: PhantomData }
}
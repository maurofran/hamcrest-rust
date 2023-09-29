use std::marker::PhantomData;

use crate::description::{Description, NoDescription, SelfDescribing};
use crate::matcher::DiagnosingMatcher;
use crate::prelude::Matcher;

pub struct HasItemsMatcher<T, I, M>
    where T: SelfDescribing + Sized,
          I: IntoIterator<Item=T> + SelfDescribing + Copy,
          M: Matcher<T> {
    element_matcher: M,
    marker: PhantomData<I>,
}

impl<T, I, M> DiagnosingMatcher<I> for HasItemsMatcher<T, I, M>
    where T: SelfDescribing + Sized,
          I: IntoIterator<Item=T> + SelfDescribing + Copy,
          M: Matcher<T> {
    fn matches_describing<D>(&self, value: &I, description: &mut D) -> bool where D: Description {
        if value.into_iter().count() == 0 {
            description.append_text("was empty");
            return false;
        }
        for element in value.into_iter() {
            if self.element_matcher.matches(&element) {
                return true;
            }
        }
        description.append_text("mismatches where: [");
        for (i, element) in value.into_iter().enumerate() {
            if i > 0 {
                description.append_text(", ");
            }
            self.element_matcher.describe_mismatch(&element, description);
        }
        description.append_text("]");
        false
    }
}

impl<T, I, M> SelfDescribing for HasItemsMatcher<T, I, M>
    where T: SelfDescribing + Sized,
          I: IntoIterator<Item=T> + SelfDescribing + Copy,
          M: Matcher<T> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description
            .append_text("a collection containing ")
            .append_description_of(&self.element_matcher);
    }
}

impl<T, I, M> Matcher<I> for HasItemsMatcher<T, I, M>
    where T: SelfDescribing + Sized,
          I: IntoIterator<Item=T> + SelfDescribing + Copy,
          M: Matcher<T> {
    fn matches(&self, value: &I) -> bool {
        let mut description = NoDescription;
        self.matches_describing(value, &mut description)
    }

    fn describe_mismatch<D>(&self, value: &I, description: &mut D) where D: Description {
        self.matches_describing(value, description);
    }
}

pub fn has_item<T, I, M>(matcher: M) -> HasItemsMatcher<T, I, M>
    where T: SelfDescribing + Sized,
          I: IntoIterator<Item=T> + SelfDescribing + Copy,
          M: Matcher<T> {
    HasItemsMatcher {
        element_matcher: matcher,
        marker: PhantomData,
    }
}
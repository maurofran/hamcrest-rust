use std::marker::PhantomData;

use crate::description::{Description, NoDescription, SelfDescribing};
use crate::matcher::{DiagnosingMatcher, Matcher};

pub struct EveryItemMatcher<T, M> where T: SelfDescribing + Sized, M: Matcher<T> {
    matcher: M,
    marker: PhantomData<T>,
}

impl<T, M> Matcher<Vec<T>> for EveryItemMatcher<T, M> where T: SelfDescribing + Sized, M: Matcher<T> {
    fn matches(&self, value: &Vec<T>) -> bool {
        let mut description = NoDescription;
        self.matches_describing(value, &mut description)
    }

    fn describe_mismatch<D>(&self, value: &Vec<T>, description: &mut D) where D: Description {
        self.matches_describing(value, description);
    }
}

impl<T, M> DiagnosingMatcher<Vec<T>> for EveryItemMatcher<T, M> where T: SelfDescribing + Sized, M: Matcher<T> {
    fn matches_describing<D>(&self, value: &Vec<T>, description: &mut D) -> bool where D: Description {
        for item in value.iter() {
            if !self.matcher.matches(item) {
                description.append_text("an item ");
                self.matcher.describe_mismatch(item, description);
                return false;
            }
        }
        true
    }
}

impl<T, M> SelfDescribing for EveryItemMatcher<T, M> where T: SelfDescribing + Sized, M: Matcher<T> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("every item is ").append_description_of(&self.matcher);
    }
}

pub fn every_item<T, M>(matcher: M) -> EveryItemMatcher<T, M> where T: SelfDescribing + Sized, M: Matcher<T> {
    EveryItemMatcher {
        matcher,
        marker: PhantomData,
    }
}
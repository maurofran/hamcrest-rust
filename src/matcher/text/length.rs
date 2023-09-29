use crate::description::{Description, NoDescription, SelfDescribing};
use crate::matcher::{DiagnosingMatcher, Matcher};
use crate::matcher::core::is_equal::{equal_to, IsEqual};

pub struct StringLengthMatcher<M> where M: Matcher<usize> {
    length_matcher: M,
}

impl<M> Matcher<String> for StringLengthMatcher<M> where M: Matcher<usize> {
    fn matches(&self, value: &String) -> bool {
        let mut description = NoDescription;
        self.matches_describing(value, &mut description)
    }

    fn describe_mismatch<D>(&self, value: &String, description: &mut D) where D: Description {
        self.matches_describing(value, description);
    }
}

impl<M> DiagnosingMatcher<String> for StringLengthMatcher<M> where M: Matcher<usize> {
    fn matches_describing<D>(&self, value: &String, description: &mut D) -> bool where D: Description {
        let length = value.len();
        if !self.length_matcher.matches(&length) {
            description.append_text("a string with length ");
            self.length_matcher.describe_mismatch(&length, description);
            return false;
        }
        true
    }
}

impl<M> SelfDescribing for StringLengthMatcher<M> where M: Matcher<usize> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("a string with length ").append_description_of(&self.length_matcher);
    }
}

pub fn has_length<M>(length_matcher: M) -> StringLengthMatcher<M> where M: Matcher<usize> {
    StringLengthMatcher { length_matcher }
}

pub fn has_exact_length(length: usize) -> StringLengthMatcher<IsEqual<usize>> {
    has_length(equal_to(length))
}
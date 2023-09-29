use regex::Regex;

use crate::description::{Description, NoDescription, SelfDescribing};
use crate::matcher::{DiagnosingMatcher, Matcher};

pub struct RegexpMatcher {
    regex: Regex,
}

impl SelfDescribing for RegexpMatcher {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("a string matching the pattern ").append_description_of(&self.regex);
    }
}

impl DiagnosingMatcher<String> for RegexpMatcher {
    fn matches_describing<D>(&self, value: &String, description: &mut D) -> bool where D: Description {
        if !self.regex.is_match(value) {
            description.append_text("the string was ").append_description_of(value);
            return false;
        }
        true
    }
}

impl Matcher<String> for RegexpMatcher {
    fn matches(&self, value: &String) -> bool {
        let mut description = NoDescription;
        self.matches_describing(value, &mut description)
    }

    fn describe_mismatch<D>(&self, value: &String, description: &mut D) where D: Description {
        self.matches_describing(value, description);
    }
}

pub fn matches_regexp(regexp: Regex) -> RegexpMatcher {
    RegexpMatcher { regex: regexp }
}

pub fn matches_pattern(pattern: &str) -> RegexpMatcher {
    RegexpMatcher { regex: Regex::new(pattern).unwrap() }
}
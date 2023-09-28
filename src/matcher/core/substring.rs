use crate::description::Description;
use crate::matcher::Matcher;
use crate::prelude::SelfDescribing;

pub struct SubstringMatcher<'a> {
    relationship: &'a str,
    ignoring_case: bool,
    substring: &'a str,
    eval_substring_of: fn(String, &str) -> bool,
}

impl<'a> SubstringMatcher<'a> {
    pub fn ignoring_case(self) -> Self {
        Self {
            ignoring_case: true,
            ..self
        }
    }

    fn converted<'b>(&self, arg: &String) -> String {
        if self.ignoring_case {
            arg.to_lowercase()
        } else {
            arg.to_string()
        }
    }
}

impl<'a> Matcher<String> for SubstringMatcher<'a> {
    fn matches(&self, value: &String) -> bool {
        let converted = self.converted(value);
        let substring = self.converted(&self.substring.to_string());
        (self.eval_substring_of)(converted, substring.as_str())
    }
}

impl<'a> SelfDescribing for SubstringMatcher<'a> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("a string ")
            .append_text(self.relationship)
            .append_text(" ")
            .append_string_value(self.substring);
        if self.ignoring_case {
            description.append_text(" ignoring case");
        }
    }
}

pub fn contains_string(substring: &str) -> SubstringMatcher {
    SubstringMatcher {
        relationship: "containing",
        ignoring_case: false,
        substring,
        eval_substring_of: |s, sub| s.contains(sub)
    }
}

pub fn ends_with(suffix: &str) -> SubstringMatcher {
    SubstringMatcher {
        relationship: "ending with",
        ignoring_case: false,
        substring: suffix,
        eval_substring_of: |s, sub| s.ends_with(sub)
    }
}

pub fn starts_with(prefix: &str) -> SubstringMatcher {
    SubstringMatcher {
        relationship: "starting with",
        ignoring_case: false,
        substring: prefix,
        eval_substring_of: |s, sub| s.starts_with(sub)
    }
}
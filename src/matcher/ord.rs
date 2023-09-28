use std::cmp::Ordering;

use crate::description::{Description, SelfDescribing};
use crate::matcher::Matcher;

enum Comparison {
    LessThan,
    LessThanOrEqualTo,
    Equal,
    GreaterThan,
    GreaterThanOrEqualTo,
}

impl Comparison {
    fn compare<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        match self {
            Comparison::LessThan => a.partial_cmp(b).map_or(false, |o| o == Ordering::Less),
            Comparison::LessThanOrEqualTo => a.partial_cmp(b).map_or(false, |o| o == Ordering::Less || o == Ordering::Equal),
            Comparison::Equal => a.partial_cmp(b).map_or(false, |o| o == Ordering::Equal),
            Comparison::GreaterThan => a.partial_cmp(b).map_or(false, |o| o == Ordering::Greater),
            Comparison::GreaterThanOrEqualTo => a.partial_cmp(b).map_or(false, |o| o == Ordering::Greater || o == Ordering::Equal),
        }
    }
}

impl SelfDescribing for Comparison {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        match self {
            Comparison::LessThan => description.append_text("less than"),
            Comparison::LessThanOrEqualTo => description.append_text("less than or equal to"),
            Comparison::Equal => description.append_text("equal to"),
            Comparison::GreaterThan => description.append_text("greater than"),
            Comparison::GreaterThanOrEqualTo => description.append_text("greater than or equal to"),
        };
    }
}

pub struct PartialOrdMatcher<T: PartialOrd + SelfDescribing> {
    value: T,
    comparison: Comparison,
}

impl<T: PartialOrd + SelfDescribing> Matcher<T> for PartialOrdMatcher<T> {
    fn matches(&self, value: &T) -> bool {
        self.comparison.compare(value, &self.value)
    }
}

impl<T: PartialOrd + SelfDescribing> SelfDescribing for PartialOrdMatcher<T> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("a value ")
            .append_description_of(&self.comparison)
            .append_text(" ")
            .append_description_of(&self.value);
    }
}

pub fn compares_equals_to<T: PartialOrd + SelfDescribing>(value: T) -> PartialOrdMatcher<T> {
    PartialOrdMatcher {
        value,
        comparison: Comparison::Equal,
    }
}

pub fn greater_than<T: PartialOrd + SelfDescribing>(value: T) -> PartialOrdMatcher<T> {
    PartialOrdMatcher {
        value,
        comparison: Comparison::GreaterThan,
    }
}

pub fn greater_than_or_equal_to<T: PartialOrd + SelfDescribing>(value: T) -> PartialOrdMatcher<T> {
    PartialOrdMatcher {
        value,
        comparison: Comparison::GreaterThanOrEqualTo,
    }
}

pub fn less_than<T: PartialOrd + SelfDescribing>(value: T) -> PartialOrdMatcher<T> {
    PartialOrdMatcher {
        value,
        comparison: Comparison::LessThan,
    }
}

pub fn less_than_or_equal_to<T: PartialOrd + SelfDescribing>(value: T) -> PartialOrdMatcher<T> {
    PartialOrdMatcher {
        value,
        comparison: Comparison::LessThanOrEqualTo,
    }
}
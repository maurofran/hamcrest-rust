use std::fmt;
use regex::Regex;

pub trait SelfDescribing {
    fn describe_to<D>(&self, description: &mut D) where D: Description;
}

impl<T: SelfDescribing> SelfDescribing for [T] {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("[");
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                description.append_text(", ");
            }
            item.describe_to(description);
        }
        description.append_text("]");
    }
}

impl<T: SelfDescribing> SelfDescribing for Vec<T> {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("[");
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                description.append_text(", ");
            }
            item.describe_to(description);
        }
        description.append_text("]");
    }
}

#[cfg(feature = "regex")]
impl SelfDescribing for Regex {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text(format!("/{}/", self.as_str()).as_str());
    }
}

macro_rules! impl_self_describing {
    ($t:ty) => {
        impl SelfDescribing for $t {
            fn describe_to<D>(&self, description: &mut D) where D: Description {
                description.append_text(self.to_string().as_str());
            }
        }
    }
}
impl_self_describing!(bool);
impl_self_describing!(u8);
impl_self_describing!(u16);
impl_self_describing!(u32);
impl_self_describing!(u64);
impl_self_describing!(u128);
impl_self_describing!(usize);
impl_self_describing!(i8);
impl_self_describing!(i16);
impl_self_describing!(i32);
impl_self_describing!(i64);
impl_self_describing!(i128);
impl_self_describing!(isize);
impl_self_describing!(f32);
impl_self_describing!(f64);

impl SelfDescribing for char {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("'").append_text(self.to_string().as_str()).append_text("'");
    }
}

impl SelfDescribing for &str {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("\"").append_text(self).append_text("\"");
    }
}

impl SelfDescribing for String {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text("\"").append_text(self).append_text("\"");
    }
}

/// A description of a [Matcher].
/// A [Matcher] will describe itself to a description which can later be used for reporting.
pub trait Description: Sized {
    /// Appends some plain text to the description.
    ///
    /// # Arguments
    /// * `text` - The text to append.
    fn append_text(&mut self, text: &str) -> &mut Self;

    /// Appends the description of a [SelfDescribing] value to this description.
    ///
    /// # Arguments
    /// * `value` - The value to describe.
    fn append_description_of(&mut self, value: &impl SelfDescribing) -> &mut Self {
        value.describe_to(self);
        self
    }
}

/// A description that does nothing.
#[derive(Debug)]
pub struct NoDescription;

impl Description for NoDescription {
    fn append_text(&mut self, _text: &str) -> &mut Self {
        self
    }

    fn append_description_of(&mut self, _value: &impl SelfDescribing) -> &mut Self {
        self
    }
}

#[derive(Debug, Clone)]
pub struct StringDescription {
    text: String,
}

impl StringDescription {
    /// Creates a new empty [StringDescription].
    pub fn new() -> Self {
        "".into()
    }

    /// Return the description of a [SelfDescribing] object as a String.
    ///
    /// # Arguments
    /// * `value` - The value to describe.
    pub fn describe<S>(value: &S) -> String where S: SelfDescribing {
        let mut description = StringDescription::new();
        value.describe_to(&mut description);
        description.to_string()
    }
}

impl fmt::Display for StringDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<String> for StringDescription {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl From<&str> for StringDescription {
    fn from(text: &str) -> Self {
        Self { text: text.to_string() }
    }
}

impl Description for StringDescription {
    fn append_text(&mut self, text: &str) -> &mut Self {
        self.text.push_str(text);
        self
    }
}
use std::fmt;

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

impl<T: fmt::Display> SelfDescribing for T {
    fn describe_to<D>(&self, description: &mut D) where D: Description {
        description.append_text(self.to_string().as_str());
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
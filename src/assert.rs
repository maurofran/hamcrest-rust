#[macro_export]
macro_rules! assert_that {
    ($actual:expr, $matcher:expr) => {
        assert_that!("", $actual, $matcher);
    };
    ($reason:literal, $actual:expr, $matcher:expr) => {{
        let m = $matcher;
        if !m.matches(&$actual) {
            let mut description = StringDescription::new();
            description.append_text($reason)
                       .append_text("\n")
                       .append_text("Expected: ")
                       .append_description_of(&m)
                       .append_text("\n")
                       .append_text("     but: ");
            m.describe_mismatch(&$actual, &mut description);
            panic!("{}", description.to_string());
        }
    }};
    ($reason:literal, $assertion:expr) => {{
        if !$assertion {
            panic("{}", $reason);
        }
    }};
}
#![cfg(all(feature = "serialize", feature = "deserialize"))]

use serde::Serialize;
use serde_saphyr::{QuoteStyle, from_str, ser_options, to_string_with_options};

#[derive(Serialize)]
struct Payload<'a> {
    value: &'a str,
}

#[test]
fn quote_style_default_punctuation_uses_single_quotes() {
    let opts = ser_options! {};

    assert_eq!(to_string_with_options(&".", opts).unwrap(), "'.'\n");
    assert_eq!(to_string_with_options(&"#", opts).unwrap(), "'#'\n");
    assert_eq!(to_string_with_options(&"-", opts).unwrap(), "'-'\n");
}

#[test]
fn quote_style_double_punctuation_uses_double_quotes() {
    let opts = ser_options! {
        quote_style: QuoteStyle::Double,
    };

    assert_eq!(to_string_with_options(&".", opts).unwrap(), "\".\"\n");
    assert_eq!(to_string_with_options(&"#", opts).unwrap(), "\"#\"\n");
    assert_eq!(to_string_with_options(&"-", opts).unwrap(), "\"-\"\n");
}

#[test]
fn quote_style_double_uses_double_quotes_for_unsafe_strings() {
    let opts = ser_options! {
        quote_style: QuoteStyle::Double,
    };

    assert_eq!(to_string_with_options(&"123", opts).unwrap(), "\"123\"\n");
    assert_eq!(to_string_with_options(&"true", opts).unwrap(), "\"true\"\n");
    assert_eq!(
        to_string_with_options(&"key: value", opts).unwrap(),
        "\"key: value\"\n"
    );
}

#[test]
fn quote_style_single_with_quote_all_prefers_single_quotes() {
    let opts = ser_options! { quote_all: true, quote_style: QuoteStyle::Single };
    let yaml = to_string_with_options(&"hello world", opts).unwrap();

    assert_eq!(yaml, "'hello world'\n");
}

#[test]
fn quote_style_double_with_quote_all_prefers_double_quotes() {
    let opts = ser_options! { quote_all: true, quote_style: QuoteStyle::Double };
    let yaml = to_string_with_options(&"hello world", opts).unwrap();

    assert_eq!(yaml, "\"hello world\"\n");
}

#[test]
fn quote_style_single_falls_back_to_double_for_single_quote() {
    let opts = ser_options! { quote_all: true, quote_style: QuoteStyle::Single };
    let yaml = to_string_with_options(&"it's here", opts).unwrap();

    assert_eq!(yaml, "\"it's here\"\n");
}

#[test]
fn quote_style_single_falls_back_to_double_for_control_chars() {
    let opts = ser_options! { quote_all: true, quote_style: QuoteStyle::Single };
    let yaml = to_string_with_options(&"line1\nline2", opts).unwrap();

    assert_eq!(yaml, "\"line1\\nline2\"\n");
}

#[test]
fn quote_style_controls_auto_quoted_values_when_quote_all_is_off() {
    let single_opts = ser_options! { quote_style: QuoteStyle::Single };
    let single_yaml = to_string_with_options(&"true", single_opts).unwrap();
    assert_eq!(single_yaml, "'true'\n");

    let double_opts = ser_options! { quote_style: QuoteStyle::Double };
    let double_yaml = to_string_with_options(&"true", double_opts).unwrap();
    assert_eq!(double_yaml, "\"true\"\n");
}

#[test]
fn quote_style_does_not_force_quotes_for_plain_safe_scalars() {
    let opts = ser_options! { quote_style: QuoteStyle::Double };
    let yaml = to_string_with_options(&"plain", opts).unwrap();

    assert_eq!(yaml, "plain\n");
}

#[test]
fn quote_style_applies_to_nested_values_that_require_quotes() {
    let value = Payload { value: "0123" };

    let single_opts = ser_options! { quote_style: QuoteStyle::Single };
    let single_yaml = to_string_with_options(&value, single_opts).unwrap();
    assert_eq!(single_yaml, "value: '0123'\n");

    let double_opts = ser_options! { quote_style: QuoteStyle::Double };
    let double_yaml = to_string_with_options(&value, double_opts).unwrap();
    assert_eq!(double_yaml, "value: \"0123\"\n");
}

#[test]
fn quote_style_changes_backslash_representation() {
    let input = r"path\to\file";

    let single_opts = ser_options! { quote_all: true, quote_style: QuoteStyle::Single };
    let single_yaml = to_string_with_options(&input, single_opts).unwrap();
    assert_eq!(single_yaml, "'path\\to\\file'\n");

    let double_opts = ser_options! { quote_all: true, quote_style: QuoteStyle::Double };
    let double_yaml = to_string_with_options(&input, double_opts).unwrap();
    assert_eq!(double_yaml, "\"path\\\\to\\\\file\"\n");
}

#[test]
fn quote_style_roundtrips_for_both_styles() {
    let original = "line1\nit's \"ok\"";

    for style in [QuoteStyle::Single, QuoteStyle::Double] {
        let opts = ser_options! { quote_all: true, quote_style: style };
        let yaml = to_string_with_options(&original, opts).unwrap();
        let parsed: String = from_str(&yaml).unwrap();
        assert_eq!(parsed, original);
    }
}

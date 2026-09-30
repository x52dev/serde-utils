//! Tests for the miniserde string wrappers.

#![cfg(feature = "miniserde")]

use std::borrow::Cow;

use detrim::miniserde::{NonEmptyString, OptionNonEmptyString, Trimmed};
use miniserde::json;

#[test]
fn trims_unicode_whitespace_and_preserves_inner_whitespace() {
    let value: Trimmed<String> = json::from_str(r#""\u2003 ferris rust \t\n""#).unwrap();

    assert_eq!(value.0, "ferris rust");
    assert_eq!(json::to_string(&value), r#""ferris rust""#);
    assert_eq!(
        json::to_string(&Trimmed(" ferris ".to_owned())),
        r#"" ferris ""#
    );
}

#[test]
fn cow_strings_are_owned() {
    let value: Trimmed<Cow<'_, str>> = json::from_str(r#"" ferris ""#).unwrap();

    assert!(matches!(value.0, Cow::Owned(_)));
    assert_eq!(value.0, "ferris");
    assert_eq!(json::to_string(&value), r#""ferris""#);
}

#[test]
fn rejects_empty_required_strings() {
    for input in [r#""""#, r#"" \t\n ""#, "null", "true", "42", "[]", "{}"] {
        assert!(json::from_str::<NonEmptyString>(input).is_err(), "{input}");
    }

    let value: NonEmptyString = json::from_str(r#"" ferris ""#).unwrap();

    assert_eq!(value.0, "ferris");
    assert_eq!(json::to_string(&value), r#""ferris""#);
}

#[test]
fn optional_strings_map_null_and_empty_to_none() {
    for input in ["null", r#""""#, r#""  ""#] {
        let value: OptionNonEmptyString = json::from_str(input).unwrap();

        assert_eq!(value.0, None);
        assert_eq!(json::to_string(&value), "null");
    }

    let value: OptionNonEmptyString = json::from_str(r#"" ferris ""#).unwrap();

    assert_eq!(value.0.as_deref(), Some("ferris"));
}

#[test]
fn trims_every_list_element() {
    for (input, expected) in [
        (r#"[]"#, vec![]),
        (r#"[" a ","\t", " b "]"#, vec!["a", "", "b"]),
    ] {
        let value: Trimmed<Vec<String>> = json::from_str(input).unwrap();

        assert_eq!(value.0, expected);
        assert_eq!(json::to_string(&value), json::to_string(&expected));
    }

    for input in [r#"[" a ",null]"#, r#"[42]"#, "null", r#"" a ""#] {
        assert!(
            json::from_str::<Trimmed<Vec<String>>>(input).is_err(),
            "{input}"
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn deduplicates_set_elements_after_trimming() {
    use std::collections::HashSet;

    let value: Trimmed<HashSet<String>> = json::from_str(r#"[" a ","a", " a"]"#).unwrap();

    assert_eq!(value.0, HashSet::from(["a".to_owned()]));
    assert_eq!(json::to_string(&value), r#"["a"]"#);
}

#[test]
fn supports_derived_structs_and_nullable_strings() {
    #[derive(miniserde::Deserialize, miniserde::Serialize)]
    struct Form {
        name: Trimmed<String>,
        alias: Option<Trimmed<String>>,
        note: OptionNonEmptyString,
    }

    let value: Form = json::from_str(r#"{"name":" ferris ","note":" "}"#).unwrap();

    assert_eq!(
        json::to_string(&value),
        r#"{"name":"ferris","alias":null,"note":null}"#
    );
    assert!(json::from_str::<Form>(r#"{"name":"ferris"}"#).is_err());

    for input in ["null", "true", "42", "[]", "{}"] {
        assert!(json::from_str::<Trimmed<String>>(input).is_err(), "{input}");
    }
}

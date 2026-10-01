//! Tests for deser support.

#![cfg(feature = "deser")]

use std::borrow::Cow;

use deser::adapters::{Borrowed, TrimWhitespace};
use detrim::deser::{EmptyAsNone, NonEmpty, Trim};

#[test]
fn trim_strings_and_collections() {
    #[derive(Debug, deser::Deserialize, PartialEq)]
    struct Form<'a> {
        #[deser(deserialize_as = Trim)]
        name: String,
        #[deser(deserialize_as = Trim)]
        borrowed: &'a str,
        #[deser(deserialize_as = TrimWhitespace<Borrowed>)]
        cow: Cow<'a, str>,
        #[deser(deserialize_as = Vec<Trim>)]
        tags: Vec<String>,
        #[deser(deserialize_as = Option<Trim>)]
        optional: Option<String>,
    }

    let input = r#"{"name":"  ferris\t","borrowed":" ferris ","cow":" ferris ","tags":[" a ","\u2003b\u2003"],"optional":null}"#;
    let form = deser_json::from_str::<Form<'_>>(input).unwrap();

    assert_eq!(form.name, "ferris");
    assert_eq!(form.borrowed, "ferris");
    assert!(matches!(form.cow, Cow::Borrowed("ferris")));
    assert_eq!(form.tags, ["a", "b"]);
    assert_eq!(form.optional, None);

    assert!(deser_json::from_str::<Form<'_>>(r#"{"name":true}"#).is_err());
}

#[test]
fn non_empty_strings() {
    #[derive(Debug, deser::Deserialize)]
    struct Form {
        #[deser(deserialize_as = NonEmpty)]
        name: String,
    }

    assert_eq!(
        deser_json::from_str::<Form>(r#"{"name":" ferris "}"#)
            .unwrap()
            .name,
        "ferris",
    );

    for input in [
        r#"{"name":""}"#,
        r#"{"name":" \t "}"#,
        r#"{"name":null}"#,
        r#"{"name":42}"#,
        "{}",
    ] {
        assert!(deser_json::from_str::<Form>(input).is_err(), "{input}");
    }
}

#[test]
fn empty_strings_become_none() {
    #[derive(Debug, deser::Deserialize)]
    struct Form {
        #[deser(deserialize_as = EmptyAsNone)]
        name: Option<String>,
    }

    for input in [
        r#"{"name":""}"#,
        r#"{"name":" \t "}"#,
        r#"{"name":null}"#,
        "{}",
    ] {
        assert_eq!(deser_json::from_str::<Form>(input).unwrap().name, None);
    }

    assert_eq!(
        deser_json::from_str::<Form>(r#"{"name":" ferris "}"#)
            .unwrap()
            .name
            .as_deref(),
        Some("ferris"),
    );
    assert!(deser_json::from_str::<Form>(r#"{"name":42}"#).is_err());
}

#[cfg(feature = "std")]
#[test]
fn deduplicate_after_trimming() {
    #[derive(Debug, deser::Deserialize)]
    struct Form {
        #[deser(deserialize_as = std::collections::HashSet<Trim>)]
        tags: std::collections::HashSet<String>,
    }

    let form = deser_json::from_str::<Form>(r#"{"tags":["a"," a "," b "]}"#).unwrap();

    assert_eq!(form.tags.len(), 2);
    assert!(form.tags.contains("a"));
    assert!(form.tags.contains("b"));
}

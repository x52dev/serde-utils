//! Single value, true or false, boolean types for `deser`.
//!
//! [True] accepts only `true`, and [False] accepts only `false`.
//! Use these types when a boolean field must have one fixed value.

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod deser_impl;

/// Type that only deserializes from the `true` boolean value.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     deser_json::from_str::<deser_bool::True>("true").unwrap().as_bool(),
///     true,
/// );
///
/// deser_json::from_str::<deser_bool::True>("false").unwrap_err();
/// deser_json::from_str::<deser_bool::True>("42").unwrap_err();
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct True;

impl True {
    /// Returns `true`.
    pub const fn as_bool(self) -> bool {
        true
    }
}

impl From<True> for bool {
    fn from(_: True) -> Self {
        true
    }
}

impl PartialEq<False> for True {
    fn eq(&self, _: &False) -> bool {
        false
    }
}

impl PartialEq<bool> for True {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

impl PartialEq<True> for bool {
    fn eq(&self, other: &True) -> bool {
        *self == other.as_bool()
    }
}

/// Type that only deserializes from the `false` boolean value.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     deser_json::from_str::<deser_bool::False>("false").unwrap().as_bool(),
///     false,
/// );
///
/// deser_json::from_str::<deser_bool::False>("true").unwrap_err();
/// deser_json::from_str::<deser_bool::False>("42").unwrap_err();
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct False;

impl False {
    /// Returns `false`.
    pub const fn as_bool(self) -> bool {
        false
    }
}

impl From<False> for bool {
    fn from(_: False) -> Self {
        false
    }
}

impl PartialEq<True> for False {
    fn eq(&self, _: &True) -> bool {
        false
    }
}

impl PartialEq<bool> for False {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

impl PartialEq<False> for bool {
    fn eq(&self, other: &False) -> bool {
        *self == other.as_bool()
    }
}

#[cfg(test)]
mod tests {
    use deser::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Tru {
        foo: True,
    }

    #[test]
    fn de_true() {
        assert_eq!(
            Tru { foo: True },
            deser_json::from_str::<Tru>(r#"{"foo": true}"#).unwrap(),
        );

        deser_json::from_str::<Tru>(r#"{"foo": false}"#).unwrap_err();
        deser_json::from_str::<Tru>(r#"{"foo": 42}"#).unwrap_err();
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Fal {
        foo: False,
    }

    #[test]
    fn de_false() {
        assert_eq!(
            Fal { foo: False },
            deser_json::from_str::<Fal>(r#"{"foo": false}"#).unwrap(),
        );

        deser_json::from_str::<Fal>(r#"{"foo": true}"#).unwrap_err();
        deser_json::from_str::<Fal>(r#"{"foo": 42}"#).unwrap_err();
    }

    #[test]
    fn ser() {
        assert_eq!("true", deser_json::to_string(&True).unwrap());
        assert_eq!("false", deser_json::to_string(&False).unwrap());
    }

    #[test]
    fn as_bool() {
        assert!(True.as_bool());
        assert!(!False.as_bool());
    }

    #[test]
    fn from() {
        assert!(bool::from(True));
        assert!(!bool::from(False));
    }

    #[test]
    fn eq() {
        assert_eq!(True, True);
        assert_eq!(True, true);
        assert_eq!(true, True);
        assert_eq!(False, False);
        assert_eq!(False, false);
        assert_eq!(false, False);

        assert_ne!(True, False);
        assert_ne!(True, false);
        assert_ne!(False, True);
        assert_ne!(false, True);

        assert_ne!(False, True);
        assert_ne!(False, true);
        assert_ne!(True, False);
        assert_ne!(true, False);
    }

    #[test]
    fn formatting() {
        let _ = format_args!("{:?}", True);
        let _ = format_args!("{:?}", False);
    }

    #[test]
    fn other_implementations() {
        #![allow(clippy::default_constructed_unit_structs)]

        assert_eq!(True.clone(), True);
        assert_eq!(False.clone(), False);

        assert_eq!(True::default(), True);
        assert_eq!(False::default(), False);
    }
}

//! Tests for deser decimal adapters.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

macro_rules! decimal_tests {
    ($module:ident, $nullable:ty, $non_required:ty, $double_option:ty, $value:literal) => {
        mod $module {
            use super::*;

            #[derive(Debug, deser::Serialize, deser::Deserialize, PartialEq)]
            struct Nullable {
                #[deser(as = $nullable)]
                amount: Option<Decimal>,
            }

            #[derive(Debug, deser::Serialize, deser::Deserialize, PartialEq)]
            struct NonRequired {
                #[deser(as = $non_required, skip_serializing_if = Option::is_none)]
                amount: Option<Decimal>,
            }

            #[derive(Debug, deser::Serialize, deser::Deserialize, PartialEq)]
            struct DoubleOption {
                #[deser(as = $double_option, skip_serializing_if = Option::is_none)]
                amount: Option<Option<Decimal>>,
            }

            #[derive(Debug, deser::Serialize, deser::Deserialize, PartialEq)]
            struct Flattened {
                #[deser(flatten)]
                inner: DoubleOption,
            }

            #[test]
            fn required_nullable_field() {
                let json = concat!(r#"{"amount":"#, $value, "}");
                let expected = Nullable {
                    amount: Some(dec!(0.1)),
                };

                assert_eq!(deser_json::from_str::<Nullable>(json).unwrap(), expected);
                assert_eq!(deser_json::to_string(&expected).unwrap(), json);
                assert_eq!(
                    deser_json::from_str::<Nullable>(r#"{"amount":null}"#)
                        .unwrap()
                        .amount,
                    None,
                );
                assert_eq!(
                    deser_json::to_string(&Nullable { amount: None }).unwrap(),
                    r#"{"amount":null}"#,
                );
                assert!(deser_json::from_str::<Nullable>("{}").is_err());
            }

            #[test]
            fn optional_non_nullable_field() {
                let json = concat!(r#"{"amount":"#, $value, "}");
                let expected = NonRequired {
                    amount: Some(dec!(0.1)),
                };

                assert_eq!(deser_json::from_str::<NonRequired>(json).unwrap(), expected);
                assert_eq!(deser_json::to_string(&expected).unwrap(), json);
                assert_eq!(
                    deser_json::from_str::<NonRequired>("{}").unwrap().amount,
                    None
                );
                assert_eq!(
                    deser_json::to_string(&NonRequired { amount: None }).unwrap(),
                    "{}"
                );
                assert!(deser_json::from_str::<NonRequired>(r#"{"amount":null}"#).is_err());
            }

            #[test]
            fn double_option_field() {
                let json = concat!(r#"{"amount":"#, $value, "}");

                for (input, amount) in [
                    (json, Some(Some(dec!(0.1)))),
                    (r#"{"amount":null}"#, Some(None)),
                    ("{}", None),
                ] {
                    let expected = DoubleOption { amount };

                    assert_eq!(
                        deser_json::from_str::<DoubleOption>(input).unwrap(),
                        expected
                    );
                    assert_eq!(deser_json::to_string(&expected).unwrap(), input);

                    let expected = Flattened { inner: expected };

                    assert_eq!(deser_json::from_str::<Flattened>(input).unwrap(), expected);
                    assert_eq!(deser_json::to_string(&expected).unwrap(), input);
                }
            }

            #[test]
            fn invalid_decimal_values() {
                for input in [
                    r#"{"amount":true}"#,
                    r#"{"amount":"bad"}"#,
                    r#"{"amount":[]}"#,
                    r#"{"amount":{}}"#,
                ] {
                    assert!(deser_json::from_str::<Nullable>(input).is_err(), "{input}");
                    assert!(
                        deser_json::from_str::<NonRequired>(input).is_err(),
                        "{input}"
                    );
                    assert!(
                        deser_json::from_str::<DoubleOption>(input).is_err(),
                        "{input}"
                    );
                }
            }
        }
    };
}

decimal_tests!(
    string,
    deser_decimal::NullableStr,
    deser_decimal::NonRequiredStr,
    deser_decimal::DoubleOptionStr,
    r#""0.1""#
);

decimal_tests!(
    float,
    deser_decimal::NullableFloat,
    deser_decimal::NonRequiredFloat,
    deser_decimal::DoubleOptionFloat,
    "0.1"
);

decimal_tests!(
    arbitrary_precision,
    deser_decimal::NullableArbitraryPrecision,
    deser_decimal::NonRequiredArbitraryPrecision,
    deser_decimal::DoubleOptionArbitraryPrecision,
    "0.1"
);

#[test]
fn arbitrary_precision_roundtrip() {
    #[derive(Debug, deser::Serialize, deser::Deserialize, PartialEq)]
    struct Price {
        #[deser(as = deser_decimal::NullableArbitraryPrecision)]
        amount: Option<Decimal>,
    }

    let json = r#"{"amount":123456789.123456789123456789}"#;
    let price = deser_json::from_str::<Price>(json).unwrap();

    assert_eq!(price.amount, Some(dec!(123456789.123456789123456789)));
    assert_eq!(deser_json::to_string(&price).unwrap(), json);
}

#[test]
fn forward_adapter_finish() {
    use deser::{adapters::SerializeAs, ser::Chunk, Atom, Error, ErrorKind, State};

    struct RejectFinish;

    impl SerializeAs<Decimal> for RejectFinish {
        fn serialize_as<'a>(_value: &'a Decimal, _state: &mut State) -> Result<Chunk<'a>, Error> {
            Ok(Chunk::Atom(Atom::Bool(true)))
        }

        fn finish_as(_value: &Decimal, _state: &mut State) -> Result<(), Error> {
            Err(Error::new(ErrorKind::Unexpected, "finish failed"))
        }
    }

    macro_rules! check_finish {
        ($adapter:ty, $value:expr) => {{
            #[derive(deser::Serialize)]
            struct Field {
                #[deser(serialize_as = $adapter)]
                amount: Option<Decimal>,
            }

            assert!(deser_json::to_string(&Field { amount: $value }).is_err());
            assert_eq!(
                deser_json::to_string(&Field { amount: None }).unwrap(),
                r#"{"amount":null}"#,
            );
        }};
    }

    check_finish!(deser_decimal::Nullable<RejectFinish>, Some(dec!(0.1)));
    check_finish!(deser_decimal::NonRequired<RejectFinish>, Some(dec!(0.1)));

    #[derive(deser::Serialize)]
    struct DoubleField {
        #[deser(serialize_as = deser_decimal::DoubleOption<RejectFinish>)]
        amount: Option<Option<Decimal>>,
    }

    assert!(deser_json::to_string(&DoubleField {
        amount: Some(Some(dec!(0.1)))
    })
    .is_err());

    for amount in [None, Some(None)] {
        assert_eq!(
            deser_json::to_string(&DoubleField { amount }).unwrap(),
            r#"{"amount":null}"#,
        );
    }
}

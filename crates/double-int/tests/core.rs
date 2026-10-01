//! Tests for the integer type without a serialization backend.

use double_int::DoubleInt;

#[test]
fn primitive_conversions() {
    assert_eq!(DoubleInt::from(42_u32).as_i64(), 42);
    assert_eq!(DoubleInt::from(-42_i32).as_i64(), -42);
    assert_eq!(DoubleInt::default().as_i64(), 0);
}

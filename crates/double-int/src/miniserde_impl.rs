use miniserde::{de::Visitor, ser::Fragment, Deserialize, Error, Result, Serialize};

use crate::DoubleInt;

miniserde::make_place!(Place);

impl Deserialize for DoubleInt {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl Visitor for Place<DoubleInt> {
    fn negative(&mut self, value: i64) -> Result<()> {
        if (value as i128) < DoubleInt::MIN {
            return Err(Error);
        }

        self.out = Some(DoubleInt(value));

        Ok(())
    }

    fn nonnegative(&mut self, value: u64) -> Result<()> {
        if (value as u128) > DoubleInt::UMAX {
            return Err(Error);
        }

        self.out = Some(DoubleInt(value as i64));

        Ok(())
    }
}

impl Serialize for DoubleInt {
    fn begin(&self) -> Fragment<'_> {
        Fragment::I64(self.as_i64())
    }
}

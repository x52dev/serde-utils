use miniserde::{de::Visitor, ser::Fragment, Deserialize, Error, Result, Serialize};

use crate::{False, True};

miniserde::make_place!(Place);

macro_rules! boolean_impls {
    ($ty:ident, $value:literal) => {
        impl Deserialize for $ty {
            fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
                Place::new(out)
            }
        }

        impl Visitor for Place<$ty> {
            fn boolean(&mut self, value: bool) -> Result<()> {
                if value != $value {
                    return Err(Error);
                }

                self.out = Some($ty);

                Ok(())
            }
        }

        impl Serialize for $ty {
            fn begin(&self) -> Fragment<'_> {
                Fragment::Bool($value)
            }
        }
    };
}

boolean_impls!(True, true);
boolean_impls!(False, false);

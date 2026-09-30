use deser::{
    de::{Sink, SinkHandle},
    ser::Chunk,
    Atom, Error, ErrorKind, State,
};

use crate::{False, True};

deser::make_slot_wrapper!(BoolSlot);

macro_rules! impl_bool {
    ($ty:ty, $value:literal) => {
        impl<'de> Sink<'de> for BoolSlot<$ty> {
            fn atom(&mut self, atom: Atom<'_>, state: &mut State) -> Result<(), Error> {
                match atom {
                    Atom::Bool($value) => {
                        **self = Some(<$ty>::default());
                        Ok(())
                    }

                    Atom::Bool(_) => Err(Error::new(
                        ErrorKind::Unexpected,
                        concat!("expected the `", stringify!($value), "` boolean"),
                    )),

                    other => self.unexpected_atom(other, state),
                }
            }
        }

        impl<'de> deser::Deserialize<'de> for $ty {
            fn deserialize_into<'out>(
                out: &'out mut Option<Self>,
                _state: &mut State,
            ) -> SinkHandle<'out, 'de> {
                BoolSlot::make_handle(out)
            }
        }

        impl deser::Serialize for $ty {
            fn serialize(&self, _state: &mut State) -> Result<Chunk<'_>, Error> {
                Ok(Chunk::Atom(Atom::Bool($value)))
            }
        }
    };
}

impl_bool!(True, true);
impl_bool!(False, false);

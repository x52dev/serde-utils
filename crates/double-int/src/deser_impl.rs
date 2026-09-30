use deser::{
    de::{Sink, SinkHandle},
    ser::Chunk,
    Atom, Error, ErrorKind, State,
};

use crate::DoubleInt;

deser::make_slot_wrapper!(DoubleIntSlot);

impl<'de> Sink<'de> for DoubleIntSlot<DoubleInt> {
    fn atom(&mut self, atom: Atom<'_>, state: &mut State) -> Result<(), Error> {
        let value = match atom {
            Atom::I64(value) => i128::from(value),
            Atom::U64(value) => i128::from(value),
            other => return self.unexpected_atom(other, state),
        };

        if !(DoubleInt::MIN..=DoubleInt::MAX).contains(&value) {
            return Err(Error::new(
                ErrorKind::Unexpected,
                "expected an integer between -9007199254740991 and 9007199254740991",
            ));
        }

        **self = Some(DoubleInt(value as i64));
        Ok(())
    }
}

impl<'de> deser::Deserialize<'de> for DoubleInt {
    fn deserialize_into<'out>(
        out: &'out mut Option<Self>,
        _state: &mut State,
    ) -> SinkHandle<'out, 'de> {
        DoubleIntSlot::make_handle(out)
    }
}

impl deser::Serialize for DoubleInt {
    fn serialize(&self, _state: &mut State) -> Result<Chunk<'_>, Error> {
        Ok(Chunk::Atom(Atom::I64(self.0)))
    }
}

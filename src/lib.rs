use enumflags2::{BitFlag, BitFlags};

#[derive(Debug, Clone, Copy)]
pub struct Flagged<T, F: BitFlag> {
    pub value: T,
    pub flags: BitFlags<F>,
}

impl<T, F: BitFlag> Flagged<T, F> {
    pub fn and_then<U>(self, f: impl FnOnce(T) -> Flagged<U, F>) -> Flagged<U, F> {
        let inner = f(self.value);
        Flagged {
            value: inner.value,
            flags: self.flags | inner.flags, //merge flags
        }
    }

    pub fn into_result(self) -> Result<T, BitFlags<F>> {
        if self.flags != BitFlags::EMPTY {
            Err(self.flags)
        } else {
            Ok(self.value)
        }
    }

    pub fn into_result_against(self, flags: BitFlags<F>) -> Result<T, BitFlags<F>> {
        if self.flags.intersects(flags) {
            Err(self.flags & flags) //only include error flags
        } else {
            Ok(self.value)
        }
    }
}

impl<T, F: BitFlag> From<T> for Flagged<T, F> {
    fn from(value: T) -> Self {
        Self {
            value,
            flags: BitFlags::EMPTY,
        }
    }
}

impl<T, F: BitFlag> FromIterator<Flagged<T, F>> for Flagged<Vec<T>, F> {
    fn from_iter<I: IntoIterator<Item = Flagged<T, F>>>(iter: I) -> Self {
        let mut value = vec![];
        let mut flags = BitFlags::EMPTY;
        for flagged in iter {
            value.push(flagged.value);
            flags |= flagged.flags;
        }
        Self { value, flags }
    }
}

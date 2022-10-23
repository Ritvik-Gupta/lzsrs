use std::fmt::Debug;
use EncodedRef::*;

#[derive(PartialEq, Eq, Clone)]
pub enum EncodedRef<T> {
    Token(T),
    BackReference { offset: usize, length: usize },
}

impl<T> Debug for EncodedRef<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token(token) => write!(f, "'{token:?}'"),
            BackReference { offset, length } => write!(f, "off {offset} len {length}"),
        }?;

        Ok(())
    }
}

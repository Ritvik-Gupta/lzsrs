use EncodedRef::*;

#[derive(PartialEq, Eq, Clone)]
pub enum EncodedRef {
    Token(char),
    BackReference { offset: usize, length: usize },
}

impl std::fmt::Debug for EncodedRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token(token) => write!(f, "'{token}'"),
            BackReference { offset, length } => write!(f, "off {offset} len {length}"),
        }?;

        Ok(())
    }
}

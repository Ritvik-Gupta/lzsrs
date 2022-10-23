use thiserror::Error;

#[derive(Error, Debug)]
pub enum LzssError {
    #[error("Illegal window size")]
    IllegalWindowSize,
}

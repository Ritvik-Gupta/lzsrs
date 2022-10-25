use thiserror::Error;

#[derive(Error, Debug)]
pub enum LzssError {
    #[error(
        "Illegal window size : sliding window ( ={sliding_window_size} ) should be \
        strictly greater than the lookahead buffer ( ={lookahead_buffer_size} )"
    )]
    IllegalSlidingWindowSize {
        sliding_window_size: usize,
        lookahead_buffer_size: usize,
    },
    #[error("Lookahead Buffer is empty")]
    EmptyLookaheadBuffer,
}

use super::{
    encoded_reference::EncodedRef::*, encoding_consumer::EncodingConsumer, errors::LzssError,
};
use anyhow::Error;
use std::collections::VecDeque;

pub struct LzssWindow<T> {
    sliding_window: VecDeque<Option<T>>,
    lookahead_buffer_size: usize,
}

impl<T> LzssWindow<T>
where
    T: PartialEq + Eq + Clone,
{
    pub fn new(
        sliding_window_size: usize,
        initial_lookahead_buffer: Vec<T>,
    ) -> Result<Self, Error> {
        let lookahead_buffer_size = initial_lookahead_buffer.len();

        if sliding_window_size <= lookahead_buffer_size {
            return Err(LzssError::IllegalSlidingWindowSize {
                sliding_window_size,
                lookahead_buffer_size,
            }
            .into());
        }

        let mut window = Self {
            sliding_window: std::iter::repeat_with(|| None)
                .take(sliding_window_size)
                .collect(),
            lookahead_buffer_size,
        };

        initial_lookahead_buffer
            .into_iter()
            .for_each(|token| window.push_optional(Some(token)));

        Ok(window)
    }

    pub(super) fn push_optional(&mut self, token: Option<T>) {
        self.sliding_window.pop_front();
        self.sliding_window.push_back(token);
    }

    pub(super) fn find_back_ref_match(&mut self) -> Result<EncodingConsumer<T>, Error> {
        let lookahead_start_idx = self.sliding_window.len() - self.lookahead_buffer_size;
        let start_token = self.sliding_window[lookahead_start_idx]
            .clone()
            .ok_or_else(|| LzssError::EmptyLookaheadBuffer)?;

        let encoded_ref = (0..lookahead_start_idx)
            .filter(|&idx| Some(start_token.clone()) == self.sliding_window[idx])
            .map(|search_start_idx| {
                let mut search_size = 0;

                while lookahead_start_idx + search_size < self.sliding_window.len()
                    && self.sliding_window[search_start_idx + search_size]
                        == self.sliding_window[lookahead_start_idx + search_size]
                {
                    search_size += 1;
                }

                (lookahead_start_idx - search_start_idx, search_size)
            })
            .max_by_key(|&(_, length)| length)
            .map_or_else(
                || Token(start_token),
                |(offset, length)| BackReference { offset, length },
            );

        Ok(EncodingConsumer::new(self, encoded_ref))
    }
}

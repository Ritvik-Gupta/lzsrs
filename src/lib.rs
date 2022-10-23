pub mod encoded_reference;
pub mod errors;
pub mod window;

use anyhow::{Context, Error};
use encoded_reference::EncodedRef::{self, *};
use window::LzssWindow;

pub fn lzss_encode_dataset<'a, T>(
    dataset: &'a mut impl Iterator<Item = T>,
    sliding_window_size: usize,
    lookahead_buffer_size: usize,
) -> Result<impl Iterator<Item = EncodedRef<T>> + 'a, Error>
where
    T: PartialEq + Eq + Clone + 'a,
{
    let mut window = LzssWindow::new(
        sliding_window_size,
        dataset.by_ref().take(lookahead_buffer_size).collect(),
    )
    .with_context(|| format!("could not create the lzss-window"))?;

    let encoded_dataset = (0..).map_while(move |_| {
        let encoded_ref = window.find_back_ref_match()?;

        let consuming_length = match encoded_ref {
            Token(_) => 1,
            BackReference { length, .. } => length,
        };

        (0..consuming_length).for_each(|_| window.push_optional(dataset.next()));

        Some(encoded_ref)
    });

    Ok(encoded_dataset)
}

pub fn lzss_decode_dataset<T>(encoded_dataset: impl Iterator<Item = EncodedRef<T>>) -> Vec<T>
where
    T: Clone,
{
    let mut result = Vec::new();
    encoded_dataset.for_each(|encoded_ref| match encoded_ref {
        Token(token) => result.push(token),
        BackReference { offset, length } => {
            (0..length).for_each(|_| result.push(result[result.len() - offset].clone()));
        }
    });
    result
}

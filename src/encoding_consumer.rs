use super::{
    encoded_reference::EncodedRef::{self, *},
    window::LzssWindow,
};

pub struct EncodingConsumer<'a, T> {
    lzss_window: &'a mut LzssWindow<T>,
    encoded_ref: EncodedRef<T>,
}

impl<'a, T> EncodingConsumer<'a, T>
where
    T: PartialEq + Eq + Clone,
{
    pub fn new(lzss_window: &'a mut LzssWindow<T>, encoded_ref: EncodedRef<T>) -> Self {
        Self {
            lzss_window,
            encoded_ref,
        }
    }

    pub fn consume(self, dataset: &mut impl Iterator<Item = T>) -> EncodedRef<T> {
        let consuming_length = match self.encoded_ref {
            Token(_) => 1,
            BackReference { length, .. } => length,
        };

        (0..consuming_length).for_each(|_| self.lzss_window.push_optional(dataset.next()));

        self.encoded_ref
    }
}

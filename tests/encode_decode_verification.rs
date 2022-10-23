use lzsrs::{
    encoded_reference::EncodedRef::{self, *},
    lzss_decode_dataset, lzss_encode_dataset,
};
use pretty_assertions::assert_eq;

macro_rules! lzss_encoding {
    (create $($tail: tt)*) => {{
        let mut buffer = Vec::new();
        lzss_encoding![&mut buffer => $($tail)*];
        buffer
    }};
    ($buffer: expr =>) => {};
    ($buffer: expr => $token: literal, $($tail: tt)*) => {{
        let buffer = $buffer;
        buffer.push(Token($token));

        lzss_encoding!(buffer => $($tail)*);
    }};
    ($buffer: expr => off $offset: literal len $length: literal, $($tail: tt)*) => {{
        let buffer = $buffer;
        buffer.push(BackReference {
            offset: $offset,
            length: $length
        });

        lzss_encoding!(buffer => $($tail)*);
    }};
}

fn test_particular_case(dataset: &str, expected_encoded_dataset: Vec<EncodedRef<char>>) {
    let encoded_dataset = lzss_encode_dataset(&mut dataset.chars(), 15, 4).collect::<Vec<_>>();

    assert_eq!(encoded_dataset, expected_encoded_dataset);
    assert_eq!(
        lzss_decode_dataset(encoded_dataset.into_iter()),
        dataset.chars().collect::<Vec<_>>()
    );
}

#[test]
fn test1() {
    test_particular_case(
        "repetitive repeat",
        lzss_encoding![create
            'r', 'e', 'p', off 2 len 1, 't', 'i', off 2 len 2,
            'v', off 6 len 1, ' ', off 11 len 4, 'a', off 10 len 1,
        ],
    );
}

#[test]
fn test2() {
    test_particular_case(
        "abcdedede",
        lzss_encoding!(create 'a', 'b', 'c', 'd', 'e', off 2 len 4,),
    )
}

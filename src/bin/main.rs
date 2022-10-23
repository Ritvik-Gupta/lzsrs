use anyhow::Error;
use lzsrs::{encoded_reference::EncodedRef, lzss_encode_dataset};

fn main() -> Result<(), Error> {
    let dataset = "abcdedede";
    let encoded_dataset = lzss_encode_dataset(&mut dataset.chars(), 5, 5)?.collect::<Vec<_>>();

    println!(
        "\nEncoded Reference Size: {}\n",
        std::mem::size_of::<EncodedRef<char>>(),
    );

    println!("Original dataset size : {}", std::mem::size_of_val(dataset));
    println!(
        "Encoded  dataset size : {}",
        std::mem::size_of_val(encoded_dataset.as_slice())
    );
    println!("\n{:#?}", encoded_dataset);

    Ok(())
}

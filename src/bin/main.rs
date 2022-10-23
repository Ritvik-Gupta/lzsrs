use lzsrs::lzss_encode_dataset;

fn main() {
    let encoded_dataset = lzss_encode_dataset(&mut "abcdedede".chars(), 15, 4).collect::<Vec<_>>();
    println!("{:#?}", encoded_dataset);
}

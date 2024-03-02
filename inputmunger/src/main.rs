
fn bytes_to_decimals(bytes: &[u8]) -> String {
    // Asserting that the length of the byte array is divisible by 4
    assert_eq!(
        bytes.len() % 4,
        0,
        "Length of the byte array must be divisible by 4"
    );

    // Create a mutable vector to store the decimals
    let mut decimals = Vec::new();

    // Iterate over the bytes in chunks of 4
    for chunk in bytes.chunks(4) {
        // Interpret the chunk of bytes as a big endian u32 integer
        let decimal = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        decimals.push(decimal.to_string()); // Convert the integer to a string and push it to the vector
    }

    // Join the decimals into a single string separated by spaces
    decimals.join(" ")
}

fn main() {
    let bytes = hex::decode(include_bytes!("./input.txt")).unwrap();
    let res = bytes_to_decimals(&bytes);
    println!("{}", res);
}

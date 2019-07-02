use necromancer::*;
use std::fs::File;
use std::io::{BufReader, Read};

mod necromancer;

fn main() -> std::io::Result<()> {
    let file = File::open("fixtures/6.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut encrypted_base_64 = String::new();
    buf_reader.read_to_string(&mut encrypted_base_64)?;
    println!("{:?}", base_64_to_hex(&encrypted_base_64));

    let mut encrypted = base_64_to_hex(&encrypted_base_64);

    for key_size in 2..40 {
        let bytes = encrypted.clone().into_bytes();
        let mut chunked = bytes.chunks(key_size);
        let a = chunked.next().unwrap();
        let b = chunked.next().unwrap();
        println!(
            "Key size: {:?}\t|Score:\t{:?}",
            key_size,
            hamming_distance(a, b) as f32 / key_size as f32
        );
    }

    // Looks like the key size is probably 6
    let key_size = 6;

    let mut key_bytes: [Vec<u8>; 6] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    for six_bytes in encrypted.clone().into_bytes().chunks(6) {
        for (i, byte) in six_bytes.iter().cloned().enumerate() {
            key_bytes[i].push(byte);
        }
    }

    for bytes in &key_bytes {
        println!("{:?}", unmask_xor(&u8s_to_hex_string(&bytes)).2);
    }

    // Seems like this is the key?
    // 555555514444 in hex
    let key: Vec<u8> = vec![85, 85, 85, 81, 68, 68];

    let mask: Vec<u8> = key.iter().cloned().cycle().take(encrypted.len()).collect();

    println!(
        "{:?}",
        String::from_utf8(xor(&encrypted.clone().into_bytes(), &mask)).unwrap()
    );

    Ok(())
}

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

    let encrypted = base_64_to_hex(&encrypted_base_64);

    for key_size in 2..40 {
        let bytes = encrypted.clone().into_bytes();
        let mut chunked = bytes.chunks(key_size);
        let a = chunked.next().unwrap();
        let b = chunked.next().unwrap();
        let c = chunked.next().unwrap();
        let d = chunked.next().unwrap();
        println!(
            "Key size: {:?}\t|Score:\t{:?}",
            key_size,
            hamming_distance(a, b) as f32 / key_size as f32
                + hamming_distance(b, c) as f32 / key_size as f32
                + hamming_distance(c, d) as f32 / key_size as f32
                + hamming_distance(a, c) as f32 / key_size as f32
                + hamming_distance(b, d) as f32 / key_size as f32
                + hamming_distance(a, d) as f32 / key_size as f32
        );
    }

    let key_size = 4;

    let mut key_bytes: Vec<Vec<u8>> = vec![];

    for _ in 0..key_size {
        key_bytes.push(Vec::new());
    }

    for chunks in encrypted.clone().into_bytes().chunks(key_size) {
        for (i, byte) in chunks.iter().cloned().enumerate() {
            key_bytes[i].push(byte);
        }
    }

    let mut key: Vec<u8> = Vec::new();

    for bytes in &key_bytes {
        let answer = unmask_xor(&u8s_to_hex_string(&bytes));
        println!("Unencrypted: {:?}", answer.0);
        println!("English score: {:?}", answer.1);
        key.push(answer.2);
    }

    println!("KEY: {:?}", u8s_to_hex_string(&key));

    let mask: Vec<u8> = key.iter().cloned().cycle().take(encrypted.len()).collect();

    println!(
        "{:?}",
        String::from_utf8(xor(&encrypted.clone().into_bytes(), &mask)).unwrap()
    );

    Ok(())
}

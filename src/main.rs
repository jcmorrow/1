use necromancer::*;
use std::fs::File;
use std::io::{BufReader, Read};

mod necromancer;

fn main() -> std::io::Result<()> {
    let file = File::open("fixtures/6.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut encrypted_base_64 = String::new();
    buf_reader.read_to_string(&mut encrypted_base_64)?;
    println!("Encrypted and in base-64: {:?}\n", encrypted_base_64);

    let encrypted = base_64_to_hex(&encrypted_base_64);

    println!("Still encrypted, but now in hex: {:?}", encrypted);

    let bytes = hex_string_to_bytes(&encrypted);

    println!("That same thing in bytes: {:?}", bytes);

    // for candidate_key_length in 2..41 {
    //     let mut hamming_distances: Vec<u32> = Vec::new();
    //     let chunks = bytes.chunks(candidate_key_length);
    //     let in_chunks: Vec<&[u8]> = chunks.clone().collect();

    //     for (i, byte) in chunks.clone().enumerate() {
    //         for j in i + 1..chunks.len() - 1 {
    //             if let Some(next_byte) = in_chunks.get(j) {
    //                 hamming_distances.push(hamming_distance(byte, next_byte));
    //             }
    //         }
    //     }
    //     // average_hamming_distance /= candidate_key_length as f32;
    //     let cumulative_hamming_distance: u32 = hamming_distances.iter().sum();
    //     let average_hamming_distance = cumulative_hamming_distance as f32
    //         / hamming_distances.len() as f32
    //         / candidate_key_length as f32;
    //     println!(
    //         "Key length\t|Hamming Distance\t|\n{:?}\t\t|{:?}\t\t|",
    //         candidate_key_length, average_hamming_distance
    //     );
    // }

    let key_length = 29;

    let mut partial_texts: Vec<Vec<u8>> = Vec::new();

    for _ in 0..29 {
        partial_texts.push(Vec::new());
    }

    for chunk in bytes.chunks(key_length) {
        for (i, byte) in chunk.iter().enumerate() {
            partial_texts[i].push(*byte);
        }
    }

    let mut full_key: Vec<u8> = Vec::new();

    for partial_text in partial_texts {
        let (_decrypted_hex, _english_score, key) =
            deencrypt_single_byte_xor(&bytes_to_hex_string(&partial_text));
        full_key.push(key);
    }
    let full_key_plain = bytes_to_string(&full_key);
    let full_key_hex = bytes_to_hex_string(&full_key);
    println!("Key: {}", full_key_plain);
    println!(
        "Decrypted: {}",
        hex_to_string(&repeating_key_xor(
            &hex_to_string(&encrypted),
            &full_key_plain
        ))
    );

    Ok(())
}

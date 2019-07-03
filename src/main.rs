use necromancer::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

mod necromancer;

fn main() -> std::io::Result<()> {
    let file = File::open("fixtures/8.txt").unwrap();
    let buf = BufReader::new(file);

    let mut most_repeats_text = String::new();
    let mut most_repeats = 0;
    for line in buf.lines() {
        if let Ok(encrypted_hex) = line {
            let mut string_counts: HashMap<&[u8], u32> = HashMap::new();
            let bytes = hex_to_bytes(&encrypted_hex);
            for chunk in bytes.chunks(16) {
                let current_count = string_counts.entry(chunk).or_insert(0);
                *current_count += 1;
            }
            let num_repeats_in_this_text: u32 = string_counts.values().fold(0, |mut acc, entry| {
                if *entry > 1 {
                    acc += *entry;
                }
                acc
            });

            println!("{:?} | {:?}", encrypted_hex, num_repeats_in_this_text);

            if num_repeats_in_this_text > most_repeats {
                most_repeats = num_repeats_in_this_text;
                most_repeats_text = encrypted_hex;
            }
        }
    }

    println!("{:?}", most_repeats_text);

    Ok(())
}

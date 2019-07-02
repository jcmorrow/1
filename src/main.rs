use necromancer::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod necromancer;

fn main() -> std::io::Result<()> {
    let plain = "Burning 'em, if you ain't quick and nimble\n I go crazy when I hear a cymbal";
    let key = "ICE";
    println!("{:?}", repeating_key_xor(plain, key));
    Ok(())
}

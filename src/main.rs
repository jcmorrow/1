use necromancer::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod necromancer;

fn main() -> std::io::Result<()> {
    let mut best_score = 0.;
    let mut sentence: String = String::new();

    let file = File::open("fixtures/4.txt")?;

    for line in BufReader::new(file).lines() {
        let (s, score) = unmask_xor(&line?);
        if score > best_score {
            best_score = score;
            sentence = s;
        }
    }

    println!("Sentence: {:?}", sentence);
    println!("Score: {:?}", best_score);
    Ok(())
}

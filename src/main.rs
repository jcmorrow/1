use necromancer::*;

mod necromancer;

fn main() {
    // let b64: String =
    //     String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    // let normal: String = String::from("I'm killing your brain like a poisonous mushroom");
    let hex: String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.  Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let h: char = 'h';
    // let e: char = 'e';
    // let l: char = 'l';
    // assert_eq!(char_to_binary(h), vec![0, 1, 1, 0, 1, 0, 0, 0]);
    // assert_eq!(char_to_binary(e), vec![0, 1, 1, 0, 0, 1, 0, 1]);
    // assert_eq!(char_to_binary(l), vec![0, 1, 1, 0, 1, 1, 0, 0]);

    // assert_eq!(binary_to_u8(&char_to_binary(h)), 104);

    // assert_eq!(string_to_base_64(normal), b64);
    // assert_eq!(hex_to_base_64(hex), b64);

    // let string1 = "1c0111001f010100061a024b53535009181c";
    // let string2 = "686974207468652062756c6c277320657965";

    // println!("{:?}", &hex_xor(string1, string2));

    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut decrypted = String::new();
    let mut best_score = 0.0;
    // println!("{:?}", hex_to_string(encrypted));
    // println!("{:?}", hex_to_string(encrypted).into_bytes());
    for mask_char in 39..128 {
        let mask_vec = vec![mask_char; encrypted.len()];
        let mask = String::from_utf8(mask_vec.clone()).unwrap();
        let score = english_score(&hex_xor(encrypted, &string_to_hex(&mask)).into_bytes());
        if score > best_score {
            best_score = score;
            decrypted = hex_xor(encrypted, &string_to_hex(&mask));
        }
    }
    println!("Sentence: {:?}", decrypted);
    println!("Score: {:?}", best_score);
}

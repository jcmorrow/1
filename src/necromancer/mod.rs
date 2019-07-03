use openssl::symm::{decrypt, Cipher};

pub fn decrypt_aes_128_ecb(encrypted: &str, key: &str) -> String {
    let cipher = Cipher::aes_128_ecb();

    let text = decrypt(
        cipher,
        &string_to_bytes(key),
        None,
        &hex_string_to_bytes(&encrypted),
    )
    .unwrap();
    hex_to_string(&bytes_to_hex_string(&text))
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b)
        .map(|(a, b)| u32::from(hamming_distance_byte(*a, *b)))
        .sum()
}

pub fn hamming_distance_byte(a: u8, b: u8) -> u8 {
    let distance: u8 = byte_to_binary(a, 8)
        .iter()
        .zip(byte_to_binary(b, 8))
        .map(|(a, b)| a ^ b)
        .sum();
    distance
}

pub fn string_to_bytes(s: &str) -> Vec<u8> {
    String::from(s).into_bytes()
}

pub fn bytes_to_string(s: &[u8]) -> String {
    String::from_utf8(s.to_owned()).expect(&format!("Can't turn {:?} into valid string", s))
}

pub fn repeating_key_xor(s: &str, k: &str) -> String {
    let s_bytes = &String::from(s).into_bytes();
    let repeated_key: Vec<u8> = String::from(k)
        .into_bytes()
        .iter()
        .cloned()
        .cycle()
        .take(s.len())
        .collect();
    bytes_to_hex_string(&xor(s_bytes, &repeated_key))
}

pub fn deencrypt_single_byte_xor(s: &str) -> (String, f32, u8) {
    let mut decrypted = String::new();
    let mut best_score = 0.0;
    let mut key: u8 = 0;
    for candidate_key in 0..127 {
        let mask_bytes = vec![candidate_key; s.len()];
        let mask = bytes_to_string(&mask_bytes);
        let decrypted_hex = hex_xor(s, &string_to_hex(&mask));
        let decrypted_plain = hex_to_string(&decrypted_hex);
        let score = english_score(&decrypted_plain);
        if score > best_score {
            best_score = score;
            decrypted = decrypted_hex;
            key = candidate_key;
        }
    }
    (decrypted, best_score, key)
}

pub fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    String::from(s)
        .into_bytes()
        .chunks(2)
        .map(hex_to_byte)
        .collect()
}

pub fn hex_to_base_64(s: &str) -> String {
    let mut b64 = String::new();
    let mut bins: Vec<u8> = Vec::new();

    for byte in hex_string_to_bytes(s) {
        bins.append(&mut byte_to_binary(byte, 8));
    }

    for c in bins.chunks(6) {
        b64.push(BASE_64_ALPHABET[binary_to_byte(c) as usize]);
    }

    b64
}

pub fn byte_to_binary(c: u8, bits: usize) -> Vec<u8> {
    let mut xs: Vec<u8> = Vec::new();
    let mut quotient: u8 = c as u8;
    let mut remainder: u8;

    while quotient > 0 {
        remainder = quotient % 2;
        quotient /= 2;
        xs.push(remainder);
    }

    xs = pad(&xs, bits);
    xs.reverse();
    xs
}

pub fn binary_to_byte(bs: &[u8]) -> u8 {
    let mut n: u8 = 0;

    for (i, b) in bs.iter().rev().enumerate() {
        n += (2 as u8).pow(i as u32) * b;
    }

    n
}

// pad([0, 1], 2) => [0, 0, 0, 1]
pub fn pad(xs: &[u8], len: usize) -> Vec<u8> {
    let mut xs = xs.to_vec();
    while xs.len() < len {
        xs.push(0);
    }
    xs
}

#[allow(dead_code)]
pub fn string_to_base_64(s: String) -> String {
    let mut b64 = String::new();
    let mut bins: Vec<u8> = Vec::new();

    for c in s.chars() {
        bins.append(&mut byte_to_binary(c as u8, 8));
    }
    for c in bins.chunks(6) {
        b64.push(BASE_64_ALPHABET[binary_to_byte(c) as usize]);
    }

    b64
}

// Input: "ABCD"
// Output: "001083"
pub fn base_64_to_hex(s: &str) -> String {
    let mut hex = String::new();
    let mut bins: Vec<u8> = Vec::new();

    for c in string_to_bytes(s) {
        let index = base_64_index(c as char);
        if index > -1 {
            bins.append(&mut byte_to_binary(index as u8, 6));
        }
    }

    for c in bins.chunks(8) {
        hex.push_str(&u8_to_hex(binary_to_byte(c)));
    }

    hex
}

pub fn base_64_index(c: char) -> isize {
    for (i, ch) in BASE_64_ALPHABET.iter().enumerate() {
        if *ch == c {
            return i as isize;
        }
    }
    return -1;
}

// FF -> 255
pub fn hex_to_byte(hex: &[u8]) -> u8 {
    let base_16 = match String::from_utf8(hex.to_vec()) {
        Ok(x) => x,
        Err(_x) => panic!("Tried to convert {:?} in base-16 to base-10"),
    };
    let base_10 = match u32::from_str_radix(&base_16, 16) {
        Ok(x) => x,
        Err(_x) => panic!(
            "Tried to convert {:?} in base-16 to base-10",
            String::from_utf8(hex.to_owned())
        ),
    };
    base_10 as u8
}

// 255 => FF
pub fn u8_to_hex(b: u8) -> String {
    format!("{:0>2x}", b)
}

pub fn hex_xor(s1: &str, s2: &str) -> String {
    bytes_to_hex_string(&xor(&hex_string_to_bytes(&s1), &hex_string_to_bytes(&s2)))
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|x| x.0 ^ x.1).collect()
}

// Input: "I'm killing your brain like a poisonous mushroom"
// Output: "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
#[allow(dead_code)]
pub fn string_to_hex(s: &str) -> String {
    String::from(s)
        .into_bytes()
        .iter()
        .map(|x| u8_to_hex(*x))
        .collect()
}

#[allow(dead_code)]
pub fn hex_to_string(hex: &str) -> String {
    String::from_utf8(hex_to_bytes(hex))
        .expect("Non UTF-8 byte encounted while converting hex to UTF8")
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    String::from(hex)
        .into_bytes()
        .chunks(2)
        .map(hex_to_byte)
        .collect()
}

pub fn bytes_to_hex_string(xs: &[u8]) -> String {
    xs.iter().map(|x| u8_to_hex(*x)).collect()
}

pub fn character_frequencies(chars: &[u8]) -> [u32; 255] {
    let mut freq = [0 as u32; 255];
    if chars.is_empty() {
        return freq;
    }
    for ch in chars {
        freq[*ch as usize] += 1;
    }
    let freq_vec: Vec<u32> = freq
        .iter()
        .map(|x| (*x as f32 * 100.0 / chars.len() as f32) as u32)
        .collect();

    freq.copy_from_slice(&freq_vec[..255]);
    freq
}

// Mean-squared error
pub fn mse(a: u32, b: u32) -> u32 {
    (a as i32 - b as i32).pow(2) as u32
}

// MSE vs. english frequencies
pub fn english_score(string: &str) -> f32 {
    let error: u32 = character_frequencies(&string_to_bytes(string))
        .iter()
        .zip(character_frequencies(&String::from(WIKIPEDIA).into_bytes()).iter())
        .map(|x| mse(*x.0, *x.1 as u32))
        .sum();
    10000.0 / error as f32
}

const WIKIPEDIA: &str = "In computing, plain text is a loose term for data (e.g. file contents) that represent only characters of readable material but not its graphical representation nor other objects (floating-point numbers, images, etc.). It may also include a limited number of characters that control simple arrangement of text, such as spaces, line breaks, or tabulation characters (although tab characters can \"mean\" many different things, so are hardly \"plain\"). Plain text is different from formatted text, where style information is included; from structured text, where structural parts of the document such as paragraphs, sections, and the like are identified); and from binary files in which some portions must be interpreted as binary objects (encoded integers, real numbers, images, etc.).

The term is sometimes used quite loosely, to mean files that contain only \"readable\" content (or just files with nothing that the speaker doesn't prefer). For example, that could exclude any indication of fonts or layout (such as markup, markdown, or even tabs); characters such as curly quotes, non-breaking spaces, soft hyphens, em dashes, and/or ligatures; or other things.

In principle, plain text can be in any encoding, but occasionally the term is taken to imply ASCII. As Unicode-based encodings such as UTF-8 and UTF-16 become more common, that usage may be shrinking.

plain text is also sometimes used only to exclude \"binary\" files: those in which at least some parts of the file cannot be correctly interpreted via the character encoding in effect. For example, a file or string consisting of \"hello\" (in whatever encoding), following by 4 bytes that express a binary integer that is not just a character, is a binary file, not plain text by even the loosest common usages. Put another way, translating a plain text file to a character encoding that uses entirely different number to represent characters, does not change the meaning (so long as you know what encoding is in use), but for binary files such a conversion does change the meaning of at least some parts of the file.

Files that contain markup or other meta-data are generally considered plain-text, so long as the markup is also in directly human-readable form (as in HTML, XML, and so on (as Coombs, Renear, and DeRose argue,[1] punctuation is itself markup; and no one considers punctuation to disqualify a file from being plain text).

The use of plain text rather than binary files, enables files to survive much better \"in the wild\", in part by making them largely immune to computer architecture incompatibilities. For example, all the problems of Endianness can be avoided (with encodings such as UCS-2 rather than UTF-8, endianness matters, but uniformly for every character, rather than for potentially-unknown subsets of it). 
";

#[allow(dead_code)]
const BASE_64_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    fn test_string_to_bytes() {
        let a = "this is a test";

        assert_eq!(
            string_to_bytes(a),
            vec![116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116],
        );
    }

    #[test]
    fn test_hamming_distance() {
        let a = "this is a test";
        let b = "wokka wokka!!!";

        assert_eq!(
            hamming_distance(&string_to_bytes(a), &string_to_bytes(b)),
            37
        );
    }

    #[test]
    fn test_hamming_distance_byte() {
        let a: u8 = 1;
        let b: u8 = 2;

        assert_eq!(hamming_distance_byte(a, b), 2);
    }

    #[test]
    fn test_decrypt_single_char_xor() {
        let original = "Cooking MC\'s like a pound of bacon";
        // ASCII 88
        let key = "X";
        let encrypted_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        assert_eq!(repeating_key_xor(original, key), encrypted_hex);
        assert_eq!(88, deencrypt_single_byte_xor(encrypted_hex).2);
        assert_eq!(
            deencrypt_single_byte_xor(encrypted_hex).0,
            string_to_hex(original)
        );
    }

    #[test]
    fn test_hex_xor() {
        let input = hex_string_to_bytes("1c0111001f010100061a024b53535009181c");

        let mask = hex_string_to_bytes("686974207468652062756c6c277320657965");
        let output = hex_string_to_bytes("746865206b696420646f6e277420706c6179");

        assert_eq!(xor(&input, &mask), output);
        assert_eq!(bytes_to_string(&output), "the kid don\'t play");
    }

    #[test]
    fn test_repeating_key_xor() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        assert_eq!(repeating_key_xor(input, key), output);
    }

    #[test]
    fn test_hex_to_base_64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base_64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let base_64_with_line_break =
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hy\nb29t";
        let english = "I'm killing your brain like a poisonous mushroom";

        assert_eq!(base_64_to_hex(base_64), hex);
        assert_eq!(base_64_to_hex(base_64_with_line_break), hex);
        assert_eq!(hex_to_string(hex), english);
        assert_eq!(string_to_hex(english), hex);
        assert_eq!(hex_to_base_64(hex), base_64);
    }

    #[test]
    fn test_byte_to_binary() {
        // ASCII 88
        let input = 'X';
        let binary = vec![0, 1, 0, 1, 1, 0, 0, 0];

        assert_eq!(byte_to_binary(input as u8, 8), binary);
        assert_eq!(binary_to_byte(&binary), 88);
    }

    #[test]
    fn test_decrypt_aes_ecb() {
        let file = File::open("fixtures/7.txt").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut encrypted_base_64 = String::new();

        let decrypted_file = File::open("fixtures/7_decrypted.txt").unwrap();
        let mut decrypted_buf_reader = BufReader::new(decrypted_file);
        let mut decrypted = String::new();
        decrypted_buf_reader.read_to_string(&mut decrypted).unwrap();

        buf_reader.read_to_string(&mut encrypted_base_64).unwrap();
        let encrypted = base_64_to_hex(&encrypted_base_64);
        let bytes = hex_string_to_bytes(&encrypted);
        let key = "YELLOW SUBMARINE";

        assert_eq!(decrypted, decrypt_aes_128_ecb(&encrypted, key));
    }
}

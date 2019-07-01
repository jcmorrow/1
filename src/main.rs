const BASE_64_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn main() {
    let b64: String =
        String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    let normal: String = String::from("I'm killing your brain like a poisonous mushroom");
    let hex: String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let h: char = 'h';
    let e: char = 'e';
    let l: char = 'l';
    assert_eq!(char_to_binary(h), vec![0, 1, 1, 0, 1, 0, 0, 0]);
    assert_eq!(char_to_binary(e), vec![0, 1, 1, 0, 0, 1, 0, 1]);
    assert_eq!(char_to_binary(l), vec![0, 1, 1, 0, 1, 1, 0, 0]);

    assert_eq!(binary_to_u8(&char_to_binary(h)), 104);

    assert_eq!(string_to_base_64(normal), b64);
    assert_eq!(hex_to_base_64(hex), b64);
}

fn string_to_base_64(s: String) -> String {
    let mut b64 = String::new();
    let mut bins: Vec<u8> = Vec::new();

    for c in s.chars() {
        bins.append(&mut char_to_binary(c));
    }
    for c in bins.chunks(6) {
        b64.push(BASE_64_ALPHABET[binary_to_u8(c) as usize]);
    }

    b64
}

fn hex_to_base_64(s: String) -> String {
    let mut b64 = String::new();
    let mut bins: Vec<u8> = Vec::new();

    for c in s.into_bytes().chunks(2) {
        bins.append(&mut char_to_binary(
            u32::from_str_radix(&String::from_utf8(c.to_vec()).unwrap(), 16).unwrap() as u8 as char,
        ));
    }

    for c in bins.chunks(6) {
        b64.push(BASE_64_ALPHABET[binary_to_u8(c) as usize]);
    }

    b64
}

fn char_to_binary(c: char) -> Vec<u8> {
    let mut xs: Vec<u8> = Vec::new();
    let mut quotient: u8 = c as u8;
    let mut remainder: u8;

    while quotient > 0 {
        remainder = quotient % 2;
        quotient /= 2;
        xs.push(remainder);
    }

    xs = pad(&xs, 8);
    xs.reverse();
    xs
}

fn binary_to_u8(bs: &[u8]) -> u8 {
    let mut n: u8 = 0;

    for (i, b) in bs.iter().rev().enumerate() {
        n += (2 as u8).pow(i as u32) * b;
    }

    n
}

fn pad(xs: &[u8], len: usize) -> Vec<u8> {
    let mut xs = xs.to_vec();
    while xs.len() < len {
        xs.push(0);
    }
    xs
}

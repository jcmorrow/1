pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b)
        .map(|(a, b)| hamming_distance_byte(*a, *b))
        .sum()
}

pub fn hamming_distance_byte(a: u8, b: u8) -> u32 {
    let distance: u8 = char_to_binary(a as char, 8)
        .iter()
        .zip(char_to_binary(b as char, 8))
        .map(|(a, b)| a ^ b)
        .sum();
    distance as u32
}

// Input:
// (
//   "Burning 'em, if you ain't quick and nimble
//   I go crazy when I hear a cymbal",
//   "ICE",
// )
// Output:
// 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
// a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
pub fn repeating_key_xor(s: &str, k: &str) -> String {
    let s_bytes = &String::from(s).into_bytes();
    let repeated_key: Vec<u8> = String::from(k)
        .into_bytes()
        .iter()
        .cloned()
        .cycle()
        .take(s.len())
        .collect();
    u8s_to_hex_string(&xor(s_bytes, &repeated_key))
}

pub fn unmask_xor(s: &str) -> (String, f32, u8) {
    let mut decrypted = String::new();
    let mut best_score = 0.0;
    let mut key: u8 = 0;
    for mask_char in 40..128 {
        let mask_vec = vec![mask_char; s.len()];
        let mask = String::from_utf8(mask_vec.clone()).unwrap();
        let score = english_score(&hex_to_string(&hex_xor(s, &string_to_hex(&mask))).into_bytes());
        if score > best_score {
            println!(
                "Frequencies: {:?}",
                &character_frequencies(&decrypted.clone().into_bytes())[..]
            );
            println!(
                "Frequencies: {:?}",
                &character_frequencies(&String::from(WASHINGTON).into_bytes())[..]
            );
            best_score = score;
            decrypted = hex_xor(s, &string_to_hex(&mask));
            key = mask_char;
        }
    }
    (decrypted, best_score, key)
}

// "FFFF" -> ["FF", "FF"] -> [255, 255]
pub fn hex_string_to_u8(s: &str) -> Vec<u8> {
    String::from(s)
        .into_bytes()
        .chunks(2)
        .map(hex_to_u8)
        .collect()
}

// Input: "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
// Output: "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
// English: "I'm killing your brain like a poisonous mushroom"*/
#[allow(dead_code)]
pub fn hex_to_base_64(s: String) -> String {
    let mut b64 = String::new();
    let mut bins: Vec<u8> = Vec::new();

    for c in s.into_bytes().chunks(2) {
        let hex_string = match String::from_utf8(c.to_vec()) {
            Ok(x) => x,
            Err(x) => panic!("Tried to parse hex into utf8, got error: {:?}", x),
        };
        let n: u32 = match u32::from_str_radix(&hex_string, 16) {
            Ok(x) => x,
            Err(x) => panic!("Tried to parse hex into utf8, got error: {:?}", x),
        };

        bins.append(&mut char_to_binary(n as u8 as char, 8));
    }

    for c in bins.chunks(6) {
        b64.push(BASE_64_ALPHABET[binary_to_u8(c) as usize]);
    }

    b64
}

// 'c' -> [0,0,0,0,1,1,0,0]
pub fn char_to_binary(c: char, bits: usize) -> Vec<u8> {
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

// [0,1,1,0,0,0,1,1] -> 99
pub fn binary_to_u8(bs: &[u8]) -> u8 {
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
        bins.append(&mut char_to_binary(c, 8));
    }
    for c in bins.chunks(6) {
        b64.push(BASE_64_ALPHABET[binary_to_u8(c) as usize]);
    }

    b64
}

// Input: "ABCD"
// Output: "001083"
pub fn base_64_to_hex(s: &str) -> String {
    let mut hex = String::new();
    let mut bins: Vec<u8> = Vec::new();

    for c in String::from(s).chars() {
        bins.append(&mut char_to_binary(base_64_index(c) as u8 as char, 6));
    }

    for c in bins.chunks(8) {
        hex.push_str(&u8_to_hex(binary_to_u8(c)));
    }

    hex
}

pub fn base_64_index(c: char) -> usize {
    for (i, ch) in BASE_64_ALPHABET.iter().enumerate() {
        if *ch == c {
            return i;
        }
    }
    return 0;
}

// FF -> 255
pub fn hex_to_u8(hex: &[u8]) -> u8 {
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
    let mut xor: Vec<u8> = Vec::new();
    let hex1 = hex_string_to_u8(&s1);
    let hex2 = hex_string_to_u8(&s2);
    for hunks in hex1.iter().zip(hex2) {
        xor.push(hunks.0 ^ hunks.1);
    }
    u8s_to_hex_string(&xor)
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

// Input: "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
// Output: "I'm killing your brain like a poisonous mushroom"
#[allow(dead_code)]
pub fn hex_to_string(hex: &str) -> String {
    String::from_utf8(
        String::from(hex)
            .into_bytes()
            .chunks(2)
            .map(hex_to_u8)
            .collect(),
    )
    .unwrap()
}

pub fn u8s_to_hex_string(xs: &[u8]) -> String {
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
pub fn english_score(string: &[u8]) -> f32 {
    let error: u32 = character_frequencies(string)
        .iter()
        .zip(character_frequencies(&String::from(WASHINGTON).into_bytes()).iter())
        .map(|x| mse(*x.0, *x.1 as u32))
        .sum();
    10000.0 / error as f32
}

#[allow(dead_code)]
const WASHINGTON: &str = "On the 9th. William Findley and David Redick--deputed by the Committee of Safety (as it is designated) which met on the 2d. of this month at Parkinson Ferry arrived in Camp with the Resolutions of the said Committee; and to give information of the State of things in the four Western Counties of Pennsylvania to wit--Washington Fayette Westd. & Alligany in order to see if it would prevent the March of the Army into them.  At 10 oclock I had a meeting with these persons in presence of Govr. Howell (of New Jersey) the Secretary of the Treasury, Colo. Hamilton, & Mr.  Dandridge: Govr. Mifflin was invited to be present, but excused himself on acct. of business.  I told the Deputies that by one of the Resolutions it would appear that they were empowered to give information of the disposition & of the existing state of matters in the four Counties above men[tioned]; that I was ready to hear & would listen patiently, and with candour to what they had to say.  Mr. Findley began. He confined his information to such parts of the four Counties as he was best acquainted with; referring to Mr. Reddick for a recital of what fell within his knowledge, in the other parts of these Counties.  The substance of Mr. Findleys communications were as follows--viz.--That the People in the parts where he was best acquainted, had seen there folly; and he believed were disposed to submit to the Laws; that he thought, but could not undertake to be responsible, for the re-establishment of the public Offices for the Collection of the Taxes on distilled spirits, & Stills--intimating however, that it might be best for the present, & until the peoples minds were a little more tranquilized, to hold the Office of Inspection at Pitsburgh under the protection--or at least under the influence of the Garrison; That he thought the Distillers would either enter their stills or would put them down; That the Civil authority was beginning to recover its tone; & enumerated some instances of it; That the ignorance, & general want of information among the people far exceeded any thing he had any conception of; That it was not merely the excise law their opposition was aimed at, but to all law, & Government; and to the Officers of Government; and that the situation in which he had been, & the life he had led for sometime, was such, that rather than go through it again, he would prefer quitting this scene altogether.  Mr. Redicks information was similar to the above; except as to the three last recitals--on wch. I do not recollect that he expressed any sentiment further than that the situation of those who were not in the opposition to government whilst the frenzy was at its height, were obliged to sleep with their Arms by their bed Sides every night; not knowing but that before Morning they might have occasion to use them in defence of their persons, or their properties.  He added, that for a long time after the riots commenced, and until lately, the distrust of one another was such, that even friends were affraid to communicate their sentiments to each other; That by whispers this was brought about; and growing bolder as they became more communicative they found their strength, and that there was a general disposition not only to acquiesce under, but to support the Laws--and he gave some instances also of Magistrates enforcing them.  He said the People of those Counties believed that the opposition to the Excise law--or at least that their dereliction to it, in every other part of the U. States was similar to their own, and that no Troops could be got to March against them for the purpose of coercion; that every acct.  until very lately, of Troops marching against them was disbelieved; & supposed to be the fabricated tales of governmental men; That now they had got alarmed; That many were disposing of their property at an under rate, in order to leave the Country, and added (I think) that they wd. go to Detroit. That no person of any consequence, except one, but what had availed themselves of the proffered amnesty; That those who were still in the opposition, and obnoxious to the laws, were Men of little or no property, & cared but little where they resided; That he did not believe there was the least intention in them to oppose the Army; & that there was not three rounds of ammunition for them in all the Western Country. He (& I think Mr. Findley also) was apprehensive that the resentments of the Army might be productive of treatment to some of these people that might be attended with disagreeable consequences; & on that account seemed to deprecate the March of it: declaring however, that it was their wish, if the people did not give proofs of unequivocal submission, that it might not stop short of its object.  After hearing what both had to say, I briefly told them--That it had been the earnest wish of governmt. to bring the people of those counties to a sense of their duty, by mild, & lenient means; That for the purpose of representing to their sober reflection the fatal consequences of such conduct Commissioners had been sent amongst them that they might be warned, in time, of what must follow, if they persevered in their opposition to the laws; but that coercion wou'd not be resorted to except in the dernier resort: but, that the season of the year made it indispensible that preparation for it should keep pace with the propositions that had been made; That it was unnecessary for me to enumerate the transactions of those people (as they related to the proceedings of government) forasmuch as they knew them as well as I did; That the measure which they were not witness to the adoption of was not less painful than expensive--Was inconvenient, & distressing--in every point of view; but as I considered the support of the Laws as an object of the first magnitude, and the greatest part of the expense had already been incurred, that nothing Short of the most unequivocal proofs of absolute Submission should retard the March of the army into the Western counties, in order to convince them that the government could, & would enforce obedience to the laws--not suffering them to be insulted with impunity. Being asked again what proofs would be required, I answered, they knew as well as I did, what was due to justice & example. They understood my meaning--and asked if they might have another interview. I appointed five oclock in the After noon for it. At this second Meeting there was little more than a repeti[ti]on of what had passed in the forenoon; and it being again mentioned that all the principal characters, except one, in the Western counties who had been in the opposition, had submitted to the propositions--I was induced, seeing them in the Street the next day, to ask Mr. Redick who that one was?--telling him at the same time I required no disclosure that he did not feel himself entirely free to make. He requested a little time to think of it, and asked for another meeting--which was appointed at 5 oclock that afternoon--which took place accordingly when he said David Bradford was the person he had alluded to in his former conversations.  He requested to know if a Meeting of the people, by their deputies, would be permitted by the Army at any given point, on their March into that Country (with fresh evidence of the sincerity of their disposition to acquiesce in whatever might be required) . I replied I saw no objection to it, provided they came unarmed; but to be cautious that not a gun was fired, as there could be no answering for consequences in this case. I assured them that every possible care should be taken to keep the Troops from offering them any insult or damage and that those who always had been subordinate to the Laws, & such as had availed themselves of the amnesty, should not be injured in their persons or property; and that the treatment of the rest would depend upon their own conduct. That the Army, unless opposed, did not mean to act as executioners, or bring offenders to a Military Tribunal; but merely to aid the civil Magistrates, with whom offences would lye. Thus endd. the matter.";
const BASE_64_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

extern crate regex;
use regex::Regex;

// The starting code of lowercase ASCII. We use this to determine the "opposite" alpha character
const ASCII_LOWER_START: u8 = 97;

fn cipher_sub(c: char) -> char {
    if char::is_alphabetic(c) {
        ((ASCII_LOWER_START + 25) - ((c as u8) - ASCII_LOWER_START)) as char
    } else {
        c
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let re = Regex::new(r"[^A-Za-z1-9]").unwrap();

    re.replace_all(&plain, "")
        .to_ascii_lowercase() // convert all upper to lower
        .chars() // create iterator of characters
        .map(cipher_sub) // do conversion
        .collect::<Vec<char>>()
        .chunks(5) // chunks the characters into sets of 5...
        .map(|c| c.iter().collect::<String>()) // .. what are then collected as a single string
        .collect::<Vec<String>>() // then collect as a vector of strings...
        .join(" ") // and join them all with a space
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .replace(" ", "")
        .chars()
        .map(cipher_sub)
        .collect::<String>()
}

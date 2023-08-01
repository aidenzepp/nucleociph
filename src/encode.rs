//! Holds the logic for encoding a string of text.


// Returns the individual bits of a `char` as a `String`.
fn bits(character: char) -> String {
    format!("{:08b}", character as u8)
}

// Returns the bit-pair associated with a given index `i`.
fn pair(bits: &String, i: usize) -> &str {
    &bits[i * 2..(i + 1) * 2]
}

// Returns the encoded character associated with the bit-pair.
fn code(pair: &str) -> Option<char> {
    match pair {
        "00" => Some('A'),
        "01" => Some('T'),
        "10" => Some('C'),
        "11" => Some('G'),
        _    => None,
    }
}

// Processes and encodes the character.
fn process(character: char) -> impl Iterator<Item = char> {
    let bits: String = bits(character);

    (0..4).filter_map(move |i| code(pair(&bits, i)))
}

/// Returns the phrase provided into an encoded cipher of ATGC characters.
pub fn encode(phrase: String) -> String {
    phrase.chars().flat_map(process).collect()
}

#[test]
fn works() {
    let phrase: String = "Hello!".to_string();
    let cipher: String = "TACATCTTTCGATCGATCGGACAT".to_string();
    
    assert_eq!(encode(phrase), cipher);
}

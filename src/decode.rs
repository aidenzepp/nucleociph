//! Holds the logic for decoding a string of text.

// Returns the decoded bit-pair associated with the character.
fn code(character: char) -> Option<&'static str> {
    match character {
        'A' => Some("00"),
        'T' => Some("01"),
        'C' => Some("10"),
        'G' => Some("11"),
        _   => None,
    }
}

// Returns the `char` formed from the bits provided.
fn form(bits: &str) -> char {
    u8::from_str_radix(bits, 2).unwrap_or_default() as char
}

// Convert the chunks of binary representation back to characters.
fn process(bits: String) -> String {
    bits.as_bytes()
        .chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or_default())
        .map(form)
        .collect()
}

/// Decodes a cipher of ATCG characters into the phrase it represents.
///
/// Each ATCG character in the cipher is treated as a 2-bit pair, with 'A' representing "00",
/// 'T' representing "01", 'C' representing "10", and 'G' representing "11".
///
/// # Examples
///
/// ```
/// use nucleociph::decode;
///
/// let cipher: String = "TACATCTTTCGATCGATCGGACAT".to_string();
/// let phrase: String = "Hello!".to_string();
/// 
/// assert_eq!(decode(cipher), phrase);
/// ```
///
/// # Arguments
///
/// * `cipher` - A string composed of 'A', 'T', 'C', 'G' characters representing the encoded phrase.
///
/// # Returns
///
/// Returns the decoded string, or an empty string if the cipher cannot be decoded.
pub fn decode(cipher: String) -> String {
    let binary: String = cipher.chars().filter_map(code).collect();

    process(binary)
}

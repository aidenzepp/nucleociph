/// Represents the phrase of the cipher.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct Phrase {
    /// Represents the individual words of the phrase.
    words: Vec<String>,
}

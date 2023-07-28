/// Represents the phrase of the cipher.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct Phrase {
    /// Represents the individual words of the phrase.
    words: Vec<String>,
}

impl Phrase {
    /// Returns a new [`Phrase`].
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    /// Returns the [`String`]s stored.
    pub fn words(&self) -> Vec<String> {
        self.words.clone()
    }

    /// Returns the [`String`] stored at the specified index.
    pub fn get(&self, index: usize) -> Option<&String> {
        self.words.get(index)
    }

    /// Sets the word at the specified index, if found.
    pub fn set(&mut self, index: usize, word: String) {
        if let Some(prev) = self.words.get_mut(index) {
            *prev = word;
        }
    }
}

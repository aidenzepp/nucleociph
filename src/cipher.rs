// types...
use crate::nucleo::Nucleo;

/// Represents the cipher of the phrase.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct Cipher {
    /// Represents the nucleo-encoding of the phrase.
    words: Vec<Nucleo>,
}

impl Cipher {
    /// Returns a new [`Phrase`].
    pub fn new(words: Vec<Nucleo>) -> Self {
        Self { words }
    }

    /// Returns the [`Nucleo`]s stored.
    pub fn words(&self) -> Vec<Nucleo> {
        self.words.clone()
    }

    /// Returns the [`Nucleo`] stored at the specified index.
    pub fn get(&self, index: usize) -> Option<&Nucleo> {
        self.words.get(index)
    }

    /// Sets the word at the specified index, if found.
    pub fn set(&mut self, index: usize, word: Nucleo) {
        if let Some(prev) = self.words.get_mut(index) {
            *prev = word;
        }
    }
}

// types...
use crate::nucleo::Nucleo;

/// Represents the cipher of the phrase.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct Cipher {
    /// Represents the nucleo-encoding of the phrase.
    words: Vec<Nucleo>,
}

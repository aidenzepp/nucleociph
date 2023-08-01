//! A library for encoding phrases into nucleotide characters, and vice versa.

// private imports...
mod decode;
mod encode;

// public re-exports...
pub use decode::decode;
pub use encode::encode;

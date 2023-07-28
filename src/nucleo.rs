/// Represents the nucleotide characters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Nucleo {
    #[default]
    A = 0b00,
    T = 0b01,
    C = 0b10,
    G = 0b11,
}

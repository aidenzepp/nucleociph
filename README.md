
# Nucleociph
`nucleociph` is a simple library for encoding and decoding strings into and from a DNA-based cipher.

## Overview
In this library, each character in a string is treated as a sequence of 8 bits. These bits are grouped into pairs and encoded as 'A', 'T', 'G', or 'C'. 'A' represents the bit pair "00", 'T' represents "01", 'C' represents "10", and 'G' represents "11". 

You can use this library to:
1. Encode a string into a DNA-based cipher.
2. Decode a DNA-based cipher back into a string.

## Usage
Add `nucleociph` to your `Cargo.toml`:
```toml
[dependencies]
nucleociph = "1.0.1"
```

## Examples
```rust
use nucleociph::{decode, encode};

let phrase: String = "Hello World!".to_string();
let cipher: String = encode(phrase.clone());

assert_eq!(phrase, decode(cipher));
```

## License
`nucleociph` is distributed under the terms of the MIT license.

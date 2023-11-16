<img src="https://github.com/tpisto/bitread/assets/226244/93251fe8-f790-4c57-84d0-58ebc14e1254" width="100px">

# bitread
The bitread library in Rust is designed to efficiently convert small binary data into Rust structs. It requires users to specify the bit count for each field and supports both Lsb0 and Msb0 formats. The library is streamlined, offering only "bits" and "map" attributes. The core concept behind bitread is:

- Reliability: It's built to work effectively.
- Simplicity: Utilizing macro magic, it's easy to adapt for specific requirements.
- Lightweight: It's a concise declarative macro built upon [bitvec](https://github.com/ferrilab/bitvec).

This approach ensures bitread is both user-friendly and versatile, catering to a wide range of binary data handling needs in Rust programming.

!Please note that the library currently does not support endianess; it defaults to using the system's local endianess. This is an important consideration when working with data that may have specific endian requirements.

Example:
```rust
use bitread::prelude::*;

#[derive(BitRead, Debug, PartialEq)]
#[bitrw(endian = "little", bit_order = "lsb")]
pub struct PositionWithInactivityTimer {
    #[bitrw(bits = 1)]
    last_fix_failed: bool,

    #[bitrw(
        bits = 23,
        map = "|x: i32| { x as f64 * (180.0 / ((1 << 23) as f64)) }"
    )]
    latitude_degrees: f64,
}

fn main() {
    println!("Hello, world!");

      let data = vec![
          0xE8, // Last fix failed (0), 23 bit latitude (remaining bits)
          0x25, // 23 bit latitude (continued)
          0xF4, // 23 bit latitude (continued)
      ];

      let position_with_inactivity_timer =
          PositionWithInactivityTimer::read_from(data.as_slice()).unwrap();

      assert_eq!(
          position_with_inactivity_timer,
          PositionWithInactivityTimer {
              last_fix_failed: false,
              latitude_degrees: -8.33338737487793,
          }
      );
}
```
## Other libraries

If you need write capabilities too and more features, you should check following libs:
* https://github.com/sharksforarms/deku
* https://github.com/jam1garner/binrw

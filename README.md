<img src="https://github.com/tpisto/bitread/assets/226244/93251fe8-f790-4c57-84d0-58ebc14e1254" width="100px">

# bitread
Rust library to simply convert small binary data to Rust structs. This expect you to put bit count to every field. Lsb0 and Msb0 supported.
Only "bits" and "map" attributes available. Idea of this library is that:

- It works
- It is really simple macro magic, so you can quickly adapt it to your specific needs
- This is only small declarative macro on top of [bitvec](https://github.com/ferrilab/bitvec)

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

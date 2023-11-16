pub mod prelude;
// re-export of bitvec
pub mod bitvec {
    pub use bitvec::prelude::*;
    pub use bitvec::view::BitView;
}

pub use bitread_lib::*;
pub use bitread_macro::*;

#[cfg(test)]
mod tests {
    use super::prelude::*;

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

        #[bitrw(
            bits = 24,
            map = "|x: i32| { x as f64 * (360.0 / ((1 << 24) as f64)) }"
        )]
        longitude_degrees: f64,

        #[bitrw(bits = 1)]
        in_trip: bool,

        #[bitrw(bits = 7)]
        timestamp: u8,

        #[bitrw(bits = 1)]
        battery_critical: bool,

        #[bitrw(bits = 1)]
        inactivity_indicator_alarm: bool,

        #[bitrw(bits = 14, map = "|x: u16| { x as u32 * 2 }")]
        inactivity_timer_minutes: u32,

        #[bitrw(bits = 8, map = "|x: u8| { 3.5 + x as f64 * 0.032 }")]
        battery_voltage_volts: f64,

        #[bitrw(bits = 3, map = "|x: u8| { x as u32 * 45 }")]
        heading_degrees: u32,

        #[bitrw(bits = 5, map = "|x: u8| { x as u32 * 5 }")]
        speed_kmh: u32,
    }

    // Uplink Port 33: Position w/ Inactivity Timer
    #[test]
    fn test_parse_position_with_inactivity_timer() {
        let data = vec![
            0xE8, // Last fix failed (0), 23 bit latitude (remaining bits)
            0x25, // 23 bit latitude (continued)
            0xF4, // 23 bit latitude (continued)
            0x9B, // 24 bit longitude
            0x9E, // 24 bit longitude (continued)
            0x87, // 24 bit longitude (continued), In trip (1), Timestamp (remaining bits)
            0x2B, // Timestamp (continued), Battery critical (0), Inactivity indicator alarm (1)
            0x6A, // Inactivity timer (lower bits)
            0x99, // Inactivity timer (upper bits)
            0x2A, // Battery voltage
            0xAB, // Heading, Speed
        ];

        let position_with_inactivity_timer =
            PositionWithInactivityTimer::read_from(data.as_slice()).unwrap();

        assert_eq!(
            position_with_inactivity_timer,
            PositionWithInactivityTimer {
                last_fix_failed: false,
                latitude_degrees: -8.33338737487793,
                longitude_degrees: -169.28500413894653,
                in_trip: true,
                timestamp: 21,
                battery_critical: false,
                inactivity_indicator_alarm: true,
                inactivity_timer_minutes: 19636,
                battery_voltage_volts: 4.844,
                heading_degrees: 135,
                speed_kmh: 105,
            }
        );
    }
}

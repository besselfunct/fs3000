#![deny(unsafe_code)]
#![no_std]

use embedded_hal::blocking::i2c;
use interp::interp;

#[derive(Debug)]
pub struct FS3000<I2C> {
    // The concrete I2C device implementation
    i2c: I2C,
    // We don't need a device address because all FS3000 chips are at
    // hex 0x28
    address: DeviceAddr,
    // There are two variants of the FS3000 chip with different velocity measurement ranges
    subtype: ChipType,
}

impl<I2C, E> FS3000<I2C>
where
    I2C: i2c::Read<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2C, address: DeviceAddr, subtype: ChipType) -> Result<Self, E> {
        Ok(Self {
            i2c,
            address,
            subtype,
        })
    }

    pub fn get_raw_velocity(&mut self) -> Result<RawData, E> {
        let mut buffer = [0u8; 5];
        self.i2c.read(self.address as u8, &mut buffer)?;
        let data = RawData {
            checksum: buffer[0],
            data_high: buffer[1],
            data_low: buffer[2],
            generic_checksum_1: buffer[3],
            generic_checksum_2: buffer[4],
        };
        Ok(data)
    }

    pub fn get_counts(&mut self) -> u16 {
        let data = self.get_raw_velocity();
        get_counts(data.unwrap())
    }

    pub fn debug_values(&mut self) -> [u8; 5] {
        let data = self.get_raw_velocity();
        let result = data.unwrap();
        [
            result.checksum,
            result.data_high,
            result.data_low,
            result.generic_checksum_1,
            result.generic_checksum_2,
        ]
    }
    #[allow(unused)]
    fn calculate_checksum(&mut self, rawdata: RawData) -> bool {
        let sum = rawdata.data_high
            + rawdata.data_low
            + rawdata.generic_checksum_1
            + rawdata.generic_checksum_2;
        let checksum_result = u8::wrapping_add(sum, rawdata.checksum);
        if checksum_result == 0 {
            true
        } else {
            false
        }
    }
    #[allow(unused)]
    pub fn get_measurement(&mut self) -> f32 {
        let counts = self.get_counts();
        match &self.subtype {
            ChipType::Type1005 => interp(&COUNTS_1005, &MPS_1005, counts as f32),
            ChipType::Type1015 => interp(&COUNTS_1015, &MPS_1015, counts as f32),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DeviceAddr {
    Default = 0x28,
}

#[derive(Debug)]
pub enum ChipType {
    Type1005,
    Type1015,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawData {
    pub checksum: u8,
    pub data_high: u8,
    pub data_low: u8,
    pub generic_checksum_1: u8,
    pub generic_checksum_2: u8,
}

pub fn get_counts(rawdata: RawData) -> u16 {
    let result = u16::from_be_bytes([rawdata.data_high, rawdata.data_low]);
    result
}

const COUNTS_1005: [f32; 9] = [
    409.0, 915.0, 1522.0, 2066.0, 2523.0, 2908.0, 3256.0, 3572.0, 3686.0,
];
const MPS_1005: [f32; 9] = [0.0, 1.07, 2.01, 3.00, 3.97, 4.96, 5.98, 6.99, 7.23];
const COUNTS_1015: [f32; 13] = [
    409.0, 1203.0, 1597.0, 1908.0, 2187.0, 2400.0, 2629.0, 2801.0, 3006.0, 3178.0, 3309.0, 3563.0,
    3686.0,
];
const MPS_1015: [f32; 13] = [
    0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 13.0, 15.0,
];

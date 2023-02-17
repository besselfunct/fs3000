#![deny(unsafe_code)]
#![no_std]

use embedded_hal::blocking::i2c;

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
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E> + i2c::Read<Error = E>,
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
        let result = get_counts(data.unwrap());
        result
    }

    pub fn debug_values(&mut self) -> [u8; 2] {
        let data = self.get_raw_velocity();
        let result = data.unwrap();
        let values = [result.data_high, result.data_low];
        values
    }
    fn calculate_checksum(&mut self, rawdata: RawData) -> bool {
        let sum = rawdata.data_high
            + rawdata.data_low
            + rawdata.generic_checksum_1
            + rawdata.generic_checksum_2;
        let checksum_result = sum + rawdata.checksum;
        match checksum_result {
            0x00u8 => true,
            _ => false,
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
    checksum: u8,
    data_high: u8,
    data_low: u8,
    generic_checksum_1: u8,
    generic_checksum_2: u8,
}

pub fn get_counts(rawdata: RawData) -> u16 {
    let result = u16::from_be_bytes([rawdata.data_high, rawdata.data_low]);
    result
}

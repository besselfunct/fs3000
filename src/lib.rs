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
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub fn new(i2c: I2C, address: DeviceAddr, subtype: ChipType) -> Result<Self, E> {
        Ok(Self {
            i2c,
            address,
            subtype,
        })
    }
}

#[derive(Debug)]
pub enum DeviceAddr {
    Default = 0x28,
}

#[derive(Debug)]
pub enum ChipType {
    Type1005,
    Type1015,
}

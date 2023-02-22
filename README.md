# fs3000
Basic I2C driver for the Renesas FS3000-1005 and FS3000-1015 Air Velocity Sensors
---
## ! This crate is a WIP. Reading of sensors is supported, but the checksum is not calculated for the resulting data !

Currently the following features are implemented:
- Reading bytes from the sensor
- Converting bytes to counts
- Calculating the checksum
- Converting counts to m/s via interpolation

TODO:
- Add checksum calculation to `get_measurement` function
- Add custom error type and error handling if checksum calculation fails

Example use:
```rust
use fs3000::*;

// Change ChipType based on the range of the sensor you're using
let mut sensor = FS3000::new(i2c, DeviceAddr::Default, ChipType::Type1005)?;

loop {
  let measurement = sensor.get_measurement();
  println!("Air Velocity is: {:?} m/s", measurement);
  // Add some delay function here.
  // The response time of the sensor is 125 ms
}
```

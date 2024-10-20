//! MCP4725 Driver
//!
//! Provides a unit tested driver to access the MCP4725 digital to analog
//! converter ICs.
//!
//! Focuses on:
//!
//! - Providing reliable data.
//! - A safter interface to an i2c DAC.
//! - No infinite loops.
//!
//!```rust,ignore
//!
//! use dac_lib_mcp4725::MCP4725;
//!
//! /*--SNIP--*/
//!
//! fn main() -> ! {
//!     /*--SNIP--*/
//!
//!     //Device Specific I2C pins()
//!     let scl = p.PB6;
//!     let sda = p.PB7;
//! }
//! ```

#![cfg_attr(not(test), no_std)]


use embedded_hal::i2c::{I2c, ErrorKind};


mod registers;
#[allow(unused_imports)]
pub use crate::registers::{
    CommandBM,
};

/// Sensor Address, multiple depending on setup
pub const SENSOR_ADDR: u8 = 0x63;

pub const MAX_ATTEMPTS: usize = 3;

pub struct MCP4725<I2C>{
    i2c: I2C,
    address: u8,
}

impl <I2C: I2c> MCP4725<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address}
    }

    pub fn read

    pub fn fast_write_dac(&mut self, value: u16) -> Result<(), I2C::Error> {

        Ok(())
    }

    pub fn write_dac(&mut self, value: u16) -> Result<(), I2C::Error> {

        Ok(())
    }

    pub fn write_dac_eeprom(&mut self, value: u16) -> Result<(), I2C::Error> {
        Ok(())
    }
}


// Tests
#[cfg(test)]
mod dac_test {
    use embedded_hal::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c::{
        Mock as I2cMock,
        Transaction as I2cTransaction,
    };

    use super::*;

    //Check that the testing macros are functional
    #[test]
    fn self_test(){
        assert!(true);
    }

    //Check that Mocking using the embedded_hal_mock crate works
    #[test]
    fn mocking_i2c() {
        let expectations = [
            I2cTransaction::write(SENSOR_ADDR, vec![1, 2]),
            I2cTransaction::read(SENSOR_ADDR, vec![3, 4]),
        ];

        let mut i2c = I2cMock::new(&expectations);
        let mut buf = vec![0u8, 2];

        i2c.write(SENSOR_ADDR, &vec![1, 2]).unwrap();
        i2c.read(SENSOR_ADDR, &mut buf).unwrap();

        assert_eq!(buf, vec![3, 4]);

        i2c.done();
    }


}

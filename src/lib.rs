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


//use embedded_hal::i2c::{I2c, ErrorKind};
use embedded_hal::i2c::I2c;


mod registers;
#[allow(unused_imports)]
pub use crate::registers::{
    CommandBM,
    FPowerModes,
    PowerModes,
};


/// The default address
pub const DEFAULT_ADDR: u8 = 0x62;

/// The address for jumpered units
pub const JUMPER_ADDR: u8 = 0x63;

pub const MAX_ATTEMPTS: usize = 3;

pub struct DacRead{
    register: u16,
    eeprom: u16,
}

pub struct MCP4725<I2C>{
    i2c: I2C,
    address: u8,
}

impl <I2C: I2c> MCP4725<I2C> {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address}
    }

    pub fn read_dac(&mut self) -> Result<DacRead, I2C::Error> {
        let mut bytes: [u8; 5] = [0, 0, 0, 0, 0];

        self.i2c.read(
            self.address,
            &mut bytes
        )?;
        
        //break down into Register data and EEPROM data.`9
        let dac_read: DacRead = DacRead{
            register: ((bytes[1] as u16)<<4)|((bytes[2] as u16)>>4),
            eeprom: ((bytes[3] as u16)<<8)|((bytes[4] as u16)<<0),
        }; 

        return Ok(dac_read);
    }

    pub fn fast_write_dac(&mut self, value: u16) -> Result<(), I2C::Error> {
        let mut bytes: [u8; 2] = [0, 0];
        bytes[0] = (FPowerModes::Normal as u8) | ((value>>8) as u8);
        bytes[1] = (0x0F & value) as u8;

        self.i2c.write(
            self.address,
            &bytes,
        )?;

        Ok(())
    }

    pub fn write_dac(&mut self, value: u16) -> Result<(), I2C::Error> {
        let mut bytes: [u8; 3] = [0, 0, 0];
        bytes[0] |= (CommandBM::WriteDACReg as u8) | (PowerModes::Normal as u8);  
        bytes[1] |= ((value >> 4) & 0x00FF) as u8;
        bytes[2] |= ((value << 4) & 0x00F0) as u8; 

        self.i2c.write(
            self.address,
            &bytes,
        )?;

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
            I2cTransaction::write(DEFAULT_ADDR, vec![1, 2]),
            I2cTransaction::read(DEFAULT_ADDR, vec![3, 4]),
        ];

        let mut i2c = I2cMock::new(&expectations);
        let mut buf = vec![0u8, 2];

        i2c.write(DEFAULT_ADDR, &vec![1, 2]).unwrap();
        i2c.read(DEFAULT_ADDR, &mut buf).unwrap();

        assert_eq!(buf, vec![3, 4]);

        i2c.done();
    }

    #[test]
    fn read_dac() {

        //We indicate normal mode, with completed EEPROM status.
        //Then we output the 12bits of data from the DAC register.(byte 3/4).
        //Finally EEPROM data is output in bytes (5/6)
        let expectations = [
            I2cTransaction::read(
                JUMPER_ADDR,
                vec![
                (PowerModes::Normal as u8)|0x80, 
                0xFF,
                0xFF,
                (FPowerModes::Normal as u8)|0x0F,
                0xFF
                ]
            ),
        ];

        let mut i2c = I2cMock::new(&expectations);

        let mut dac_0 = MCP4725::new(&mut i2c, DEFAULT_ADDR);

        //change address.
        dac_0.address = JUMPER_ADDR;

        //check read function.
        let ret = dac_0.read_dac();
        assert!(ret.is_ok());

        let data: DacRead = ret.unwrap();
        assert_eq!(data.register, 0xFFF);
        assert_eq!(data.eeprom, 0xFFF);

        i2c.done();
    }

    #[test]
    fn write_fast_mode() {

        //The two MSB in the first byte are zero for fast mode.
        //The next two bits are for the Powerdown selection
        //The rest of the bits are for the DAC register
        let expectations = [
            I2cTransaction::write(
                DEFAULT_ADDR,
                vec!((FPowerModes::Normal as u8)| 0x0A , 0xA),
            )
        ];


        let mut i2c = I2cMock::new(&expectations);
        let mut dac_0 = MCP4725::new(&mut i2c, DEFAULT_ADDR);
        
        let ret = dac_0.fast_write_dac(0xAAA);
        assert!(ret.is_ok());

        i2c.done();
    }


    #[test]
    fn write_dac_register() {

        let expectations = [
            I2cTransaction::write(
                DEFAULT_ADDR,
                vec!(
                    (CommandBM::WriteDACReg as u8)|(PowerModes::Normal as u8),
                    0xFF,
                    0xF0
                ),
            )
        ];


        let mut i2c = I2cMock::new(&expectations);
        let mut dac_0 = MCP4725::new(&mut i2c, DEFAULT_ADDR);
        
        let ret = dac_0.write_dac(0x0FFF);
        assert!(ret.is_ok());

        i2c.done();
    }
}

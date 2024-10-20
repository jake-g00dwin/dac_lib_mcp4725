//! This file contains the enums representing the availble I2C registers and
//! commands for the MCP4725 IC.


#[repr(u8)]
#[allow(dead_code)]
pub enum CommandBM{
    FastWrite = 0x3F, // 00x* , we only care that the first two msb are zero
    WriteDACReg = 0x5F, // 010
    WriteDACRegEEPROM = 0x7F, // Writes DAC and EEPROM
}



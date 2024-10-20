//! This file contains the enums representing the availble I2C registers and
//! commands for the MCP4725 IC.


#[repr(u8)]
#[allow(dead_code)]
pub enum CommandBM{
    FastWrite = 0x3F, // 00x* , we only care that the first two msb are zero
    WriteDACReg = 0x5F, // 010
    WriteDACRegEEPROM = 0x7F, // Writes DAC and EEPROM
}

/// FPowerModes are for when writing using Fast mode.
/// It affects bits 4 and 5
#[repr(u8)]
#[allow(dead_code)]
pub enum FPowerModes{
    Normal = (0<<5)|(0<<4), // PD1 = 0 PD0 = 0
    PD1K = (0<<5)|(1<<4),
    PD100K = (1<<5)|(0<<4),
    PD500K = (1<<5)|(1<<4),
}



/// PowerModes are for when writing using normal mode.
/// It affects bits 1 and 2
#[repr(u8)]
#[allow(dead_code)]
pub enum PowerModes {
    Normal = (0<<2)|(0<<1), // PD1 = 0 PD0 = 0
    PD1K = (0<<2)|(1<<1),
    PD100K = (1<<2)|(0<<1),
    PD500K = (1<<2)|(1<<1),
}


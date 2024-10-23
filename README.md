# dac_lib_mcp4725
A rust library for the MCP4725 DAC module

## Description

The MCP4725 is a 12bit Digital to Analog converter with built in EEPROM 
memory.

**I2C Speeds:**

- Standard 100 kbps
- Fast 400 kbps
- High-Speed 3.4 Mbps

**Connections:**

OUT:: Analog Output voltage
GND:: Ground
SCL:: I2C Clock
SDA:: I2C Data
VCC:: Supply voltage 2.7v to 5.5v
GND:: Ground

## Usage

### Adding to project

There are two ways to add the crate/repo to your project. You can use the 
github URL or you can add it via the normal crates name(TBD).


**Cargo.toml**
```toml
dac_lib_mcp4725 = {git = 'https://github.com/jake-g00dwin/dac_lib_mcp4725'}
```



### Running Tests

To run the tests for the project after downloading or cloning the repo

```sh
cargo test
```

## Roadmap

- [X] Define all Bitmasks for registers.
- [ ] Add Example usage documentation.
- [ ] 

## Contributing

If you want to contribute to it feel free to open up a pull-request or make a 
new github issue.



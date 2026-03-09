#![no_std]

use embedded_hal_async::i2c::I2c;

pub mod registers;

use registers::*;

/// Possible device addresses for the TCA9535.
///
/// Represented as: `<A2><A1><A0>` where each `A2`,`A1`,`A0` correspond to the state of the address
/// pin: either `L` (Low) or `H` (High).
#[derive(Clone, Copy, Debug)]
pub enum DeviceAddress {
    LLL,
    LLH,
    LHL,
    LHH,
    HLL,
    HLH,
    HHL,
    HHH,
}

impl From<DeviceAddress> for u8 {
    fn from(value: DeviceAddress) -> Self {
        match value {
            DeviceAddress::LLL => 0x20,
            DeviceAddress::LLH => 0x21,
            DeviceAddress::LHL => 0x22,
            DeviceAddress::LHH => 0x23,
            DeviceAddress::HLL => 0x23,
            DeviceAddress::HLH => 0x25,
            DeviceAddress::HHL => 0x26,
            DeviceAddress::HHH => 0x27,
        }
    }
}

#[allow(dead_code)]
enum Register {
    InputPort0,             // read
    InputPort1,             // read
    OutputPort0,            // read-write
    OutputPort1,            // read-write
    PolarityInversionPort0, // read-write
    PolarityInversionPort1, // read-write
    ConfigurationPort0,     // read-write
    ConfigurationPort1,     // read-write
}
impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        match value {
            Register::InputPort0 => 0x00,
            Register::InputPort1 => 0x01,
            Register::OutputPort0 => 0x02,
            Register::OutputPort1 => 0x03,
            Register::PolarityInversionPort0 => 0x04,
            Register::PolarityInversionPort1 => 0x05,
            Register::ConfigurationPort0 => 0x06,
            Register::ConfigurationPort1 => 0x07,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RegisterPair {
    Input,             // read
    Ouptput,           // read-write
    PolarityInversion, // read-write
    Configuration,     // read-write
}
impl From<RegisterPair> for u8 {
    fn from(value: RegisterPair) -> Self {
        match value {
            RegisterPair::Input => 0x00,
            RegisterPair::Ouptput => 0x02,
            RegisterPair::PolarityInversion => 0x04,
            RegisterPair::Configuration => 0x06,
        }
    }
}

pub struct Tca9535<I2C: I2c> {
    i2c: I2C,
    address: DeviceAddress,
}

impl<I2C: I2c> Tca9535<I2C> {
    pub fn new(i2c: I2C, address: DeviceAddress) -> Self {
        Self { i2c, address }
    }

    #[allow(dead_code)]
    async fn read(&mut self, register: Register) -> Result<u8, I2C::Error> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(self.address.into(), &[register.into()], &mut buffer)
            .await?;
        Ok(buffer[0])
    }

    #[allow(dead_code)]
    async fn write(&mut self, register: Register, data: u8) -> Result<(), I2C::Error> {
        self.i2c
            .write(self.address.into(), &[register.into(), data])
            .await
    }

    async fn read_pair(&mut self, register_pair: RegisterPair) -> Result<u16, I2C::Error> {
        let mut buffer0 = [0u8; 1];
        self.i2c
            .write_read(
                self.address.into(),
                &[u8::from(register_pair)],
                &mut buffer0,
            )
            .await?;
        let mut buffer1 = [0u8; 1];
        self.i2c
            .write_read(
                self.address.into(),
                &[u8::from(register_pair) + 1],
                &mut buffer1,
            )
            .await?;

        Ok(u16::from_le_bytes([buffer0[0], buffer1[0]]))
    }

    async fn write_pair(
        &mut self,
        register_pair: RegisterPair,
        data: u16,
    ) -> Result<(), I2C::Error> {
        let bytes: [u8; 2] = data.to_be_bytes();
        self.i2c
            .write(self.address.into(), &[u8::from(register_pair), bytes[0]])
            .await?;

        self.i2c
            .write(
                self.address.into(),
                &[u8::from(register_pair) + 1, bytes[1]],
            )
            .await
    }

    /// Invert how the input will be interpreded.
    /// When a bit is set this means the corresponding pin is inverted, so a high logic level will
    /// be reported as 0 in [`Self::read_input()`].
    pub async fn set_polarity_inversion(&mut self, polarity: Polarity) -> Result<(), I2C::Error> {
        self.write_pair(RegisterPair::PolarityInversion, polarity.0)
            .await
    }

    /// Read the status of the polarity inversion.
    pub async fn read_polarity_inversion(&mut self) -> Result<Polarity, I2C::Error> {
        Ok(Polarity(
            self.read_pair(RegisterPair::PolarityInversion).await?,
        ))
    }

    /// Set the configuration of the pin, a bit set to 1 mean the corresponding pin is set as input.
    /// 0 means output.
    pub async fn set_configuration(
        &mut self,
        configuration: Configuration,
    ) -> Result<(), I2C::Error> {
        self.write_pair(RegisterPair::Configuration, configuration.0)
            .await
    }

    /// Read the current configuration of the pins.
    pub async fn read_configuration(&mut self) -> Result<Configuration, I2C::Error> {
        Ok(Configuration(
            self.read_pair(RegisterPair::Configuration).await?,
        ))
    }

    /// Set the output status of the pins.
    /// A bit set to 1 means the corresponding pin output is set to high.
    pub async fn set_output(&mut self, output: Output) -> Result<(), I2C::Error> {
        self.write_pair(RegisterPair::Ouptput, output.0).await
    }

    /// Read the current status of the pin output.
    pub async fn read_output(&mut self) -> Result<Output, I2C::Error> {
        Ok(Output(self.read_pair(RegisterPair::Ouptput).await?))
    }

    /// Read the input status of the pins.
    /// Each bit correspond to a pin, a bit set to 1 will mean the corresponding pin is at a
    /// high logic level, a bit set to 0 will mean the pin is at a low logic level.
    ///
    /// If the corresponding polarity inversion bit (see [`Self::set_polarity_inversion()`])
    /// is set to 1, the meaning is inverted: a bit at 0 will mean high logic level.
    pub async fn read_input(&mut self) -> Result<Input, I2C::Error> {
        Ok(Input(self.read_pair(RegisterPair::Input).await?))
    }
}

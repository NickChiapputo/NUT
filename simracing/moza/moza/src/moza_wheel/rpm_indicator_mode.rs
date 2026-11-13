#[derive(Debug)]
#[repr(u8)]
pub enum RPMIndicatorMode {
    Rpm = 1,
    Off = 2,
    On = 3
}

impl TryFrom<u8> for RPMIndicatorMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == (RPMIndicatorMode::Rpm as u8) => Ok(RPMIndicatorMode::Rpm),
            x if x == (RPMIndicatorMode::Off as u8) => Ok(RPMIndicatorMode::Off),
            x if x == (RPMIndicatorMode::On as u8) => Ok(RPMIndicatorMode::On),
            _ => Err(())
        }
    }
}

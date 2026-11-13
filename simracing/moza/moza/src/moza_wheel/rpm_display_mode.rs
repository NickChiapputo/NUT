#[derive(Debug)]
#[repr(u8)]
pub enum RPMDisplayMode {
    /// Each LED gets its own color defined.
    IndividualColors = 1,
    /// ALL LEDs are the same color as the furthest right LED that is turned on.
    SingleColor = 2,
}

impl TryFrom<u8> for RPMDisplayMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == (RPMDisplayMode::IndividualColors as u8) => Ok(RPMDisplayMode::IndividualColors),
            x if x == (RPMDisplayMode::SingleColor as u8) => Ok(RPMDisplayMode::SingleColor),
            _ => Err(())
        }
    }
}

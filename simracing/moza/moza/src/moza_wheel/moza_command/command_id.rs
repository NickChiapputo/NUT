pub const INVALID_GROUP_ID: u8 = 255;

pub enum MozaCommandID {
    WheelBrightness,
    WheelRPMTimings,
    WheelRPMIndicatorMode,
    WheelSetRPMDisplayMode,
    WheelGetRPMDisplayMode,
    WheelRPMBlinkColor1,
    WheelRPMBlinkColor2,
    WheelRPMBlinkColor3,
    WheelRPMBlinkColor4,
    WheelRPMBlinkColor5,
    WheelRPMBlinkColor6,
    WheelRPMBlinkColor7,
    WheelRPMBlinkColor8,
    WheelRPMBlinkColor9,
    WheelRPMBlinkColor10,
    WheelRPMBrightness,
    WheelRPMColor1,
    WheelRPMColor2,
    WheelRPMColor3,
    WheelRPMColor4,
    WheelRPMColor5,
    WheelRPMColor6,
    WheelRPMColor7,
    WheelRPMColor8,
    WheelRPMColor9,
    WheelRPMColor10,
    WheelButtonColor1,
    WheelButtonColor2,
    WheelButtonColor3,
    WheelButtonColor4,
    WheelButtonColor5,
    WheelButtonColor6,
    WheelButtonColor7,
    WheelButtonColor8,
    WheelButtonColor9,
    WheelButtonColor10,
    WheelRPMValue1,
    WheelRPMValue2,
    WheelRPMValue3,
    WheelRPMValue4,
    WheelRPMValue5,
    WheelRPMValue6,
    WheelRPMValue7,
    WheelRPMValue8,
    WheelRPMValue9,
    WheelRPMValue10,
    WheelSendTelemetry,
}

impl MozaCommandID {
    pub fn index_to_rpm_blink_color(index:usize) -> Self {
        match index {
            0 => MozaCommandID::WheelRPMBlinkColor1,
            1 => MozaCommandID::WheelRPMBlinkColor2,
            2 => MozaCommandID::WheelRPMBlinkColor3,
            3 => MozaCommandID::WheelRPMBlinkColor4,
            4 => MozaCommandID::WheelRPMBlinkColor5,
            5 => MozaCommandID::WheelRPMBlinkColor6,
            6 => MozaCommandID::WheelRPMBlinkColor7,
            7 => MozaCommandID::WheelRPMBlinkColor8,
            8 => MozaCommandID::WheelRPMBlinkColor9,
            _ => MozaCommandID::WheelRPMBlinkColor10,
        }
    }

    pub fn index_to_rpm_value(index:usize) -> Self {
        match index {
            0 => MozaCommandID::WheelRPMValue1,
            1 => MozaCommandID::WheelRPMValue2,
            2 => MozaCommandID::WheelRPMValue3,
            3 => MozaCommandID::WheelRPMValue4,
            4 => MozaCommandID::WheelRPMValue5,
            5 => MozaCommandID::WheelRPMValue6,
            6 => MozaCommandID::WheelRPMValue7,
            7 => MozaCommandID::WheelRPMValue8,
            8 => MozaCommandID::WheelRPMValue9,
            _ => MozaCommandID::WheelRPMValue10,
        }
    }
}

/// Convert a [MozaCommandID] enum value into a tuple containing the command ID list,
/// number of bytes in the payload, read group, and write group.
///
/// If the command is not available for reading, the read group is set to [INVALID_GROUP_ID].
///
/// If the command is not available for writing, the write group is set to [INVALID_GROUP_ID].
impl From<MozaCommandID> for (Vec<u8>, usize, u8, u8) {
    fn from(value: MozaCommandID) -> Self {
        use MozaCommandID::*;
        match value {
            //                           command_id     bytes read_group        write_group
            WheelBrightness =>          (vec![1],           1, 64,                  63),
            WheelRPMTimings =>          (vec![2],          10, 64,                  63),
            WheelRPMIndicatorMode =>    (vec![4],           1, 64,                  63),
            WheelSetRPMDisplayMode =>   (vec![7],           1, INVALID_GROUP_ID,    63),
            WheelGetRPMDisplayMode =>   (vec![8],           1, 64,                  INVALID_GROUP_ID),
            WheelRPMBlinkColor1 =>      (vec![15, 0],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor2 =>      (vec![15, 1],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor3 =>      (vec![15, 2],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor4 =>      (vec![15, 3],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor5 =>      (vec![15, 4],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor6 =>      (vec![15, 5],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor7 =>      (vec![15, 6],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor8 =>      (vec![15, 7],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor9 =>      (vec![15, 8],       3, INVALID_GROUP_ID,    63),
            WheelRPMBlinkColor10 =>     (vec![15, 9],       3, INVALID_GROUP_ID,    63),
            WheelRPMBrightness =>       (vec![20, 0],       1, 64,                  63),
            WheelRPMColor1 =>           (vec![21, 0, 0],    3, 64,                  63),
            WheelRPMColor2 =>           (vec![21, 0, 1],    3, 64,                  63),
            WheelRPMColor3 =>           (vec![21, 0, 2],    3, 64,                  63),
            WheelRPMColor4 =>           (vec![21, 0, 3],    3, 64,                  63),
            WheelRPMColor5 =>           (vec![21, 0, 4],    3, 64,                  63),
            WheelRPMColor6 =>           (vec![21, 0, 5],    3, 64,                  63),
            WheelRPMColor7 =>           (vec![21, 0, 6],    3, 64,                  63),
            WheelRPMColor8 =>           (vec![21, 0, 7],    3, 64,                  63),
            WheelRPMColor9 =>           (vec![21, 0, 8],    3, 64,                  63),
            WheelRPMColor10 =>          (vec![21, 0, 9],    3, 64,                  63),
            WheelButtonColor1 =>        (vec![21, 1, 0],    3, 64,                  63),
            WheelButtonColor2 =>        (vec![21, 1, 1],    3, 64,                  63),
            WheelButtonColor3 =>        (vec![21, 1, 2],    3, 64,                  63),
            WheelButtonColor4 =>        (vec![21, 1, 3],    3, 64,                  63),
            WheelButtonColor5 =>        (vec![21, 1, 4],    3, 64,                  63),
            WheelButtonColor6 =>        (vec![21, 1, 5],    3, 64,                  63),
            WheelButtonColor7 =>        (vec![21, 1, 6],    3, 64,                  63),
            WheelButtonColor8 =>        (vec![21, 1, 7],    3, 64,                  63),
            WheelButtonColor9 =>        (vec![21, 1, 8],    3, 64,                  63),
            WheelButtonColor10 =>       (vec![21, 1, 9],    3, 64,                  63),
            WheelRPMValue1 =>           (vec![24,0],        2, 64,                  63),
            WheelRPMValue2 =>           (vec![24,1],        2, 64,                  63),
            WheelRPMValue3 =>           (vec![24,2],        2, 64,                  63),
            WheelRPMValue4 =>           (vec![24,3],        2, 64,                  63),
            WheelRPMValue5 =>           (vec![24,4],        2, 64,                  63),
            WheelRPMValue6 =>           (vec![24,5],        2, 64,                  63),
            WheelRPMValue7 =>           (vec![24,6],        2, 64,                  63),
            WheelRPMValue8 =>           (vec![24,7],        2, 64,                  63),
            WheelRPMValue9 =>           (vec![24,8],        2, 64,                  63),
            WheelRPMValue10 =>          (vec![24,9],        2, 64,                  63),
            WheelSendTelemetry =>       (vec![253, 222],    4, INVALID_GROUP_ID,    65),
        }
    }
}

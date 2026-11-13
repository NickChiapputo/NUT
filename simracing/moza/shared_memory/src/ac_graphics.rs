/// Status of the instance
pub enum Status {
    Off = 0,
    Replay,
    Live,
    Pause
}

pub enum SessionType {
    Unknown = -1,
    Practice = 0,
    Qualify,
    Race,
    Hotlap,
    TimeAttack,
    Drift,
    Drag
}

pub enum Flag {
    None,
    Blue,
    Yellow,
    Black,
    White,
    Checkered,
    Penalty
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct PenaltyRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct StatusRaw {
    pub data: i32,
}

impl From<StatusRaw> for Status {
    fn from(value: StatusRaw) -> Self {
        match value.data {
            0 => Self::Off,
            1 => Self::Replay,
            2 => Self::Live,
            _ => Self::Pause
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Replay => write!(f, "Replay"),
            Self::Live => write!(f, "Live"),
            Self::Pause => write!(f, "Pause"),
        }
    }
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct SessionTypeRaw {
    pub data: i32,
}

impl From<SessionTypeRaw> for SessionType {
    fn from(value: SessionTypeRaw) -> Self {
        match value.data {
            0 => Self::Practice,
            1 => Self::Qualify,
            2 => Self::Race,
            3 => Self::Hotlap,
            4 => Self::TimeAttack,
            5 => Self::Drift,
            6 => Self::Drag,
            _ => Self::Unknown
        }
    }
}

impl std::fmt::Display for SessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Practice => write!(f, "Practice"),
            Self::Qualify => write!(f, "Qualify"),
            Self::Race => write!(f, "Race"),
            Self::Hotlap => write!(f, "Hotlap"),
            Self::TimeAttack => write!(f, "TimeAttack"),
            Self::Drift => write!(f, "Drift"),
            Self::Drag => write!(f, "Drag"),
        }
    }
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct FlagTypeRaw {
    pub data: i32,
}

impl From<FlagTypeRaw> for Flag {
    fn from(value: FlagTypeRaw) -> Self {
        match value.data {
            1 => Self::Blue,
            2 => Self::Yellow,
            3 => Self::Black,
            4 => Self::White,
            5 => Self::Checkered,
            6 => Self::Penalty,
            _ => Self::None,
        }
    }
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Blue => write!(f, "Blue"),
            Self::Yellow => write!(f, "Yellow"),
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White"),
            Self::Checkered => write!(f, "Checkered"),
            Self::Penalty => write!(f, "Penalty"),
        }
    }
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct TrackGripStatusRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct RainIntensityRaw {
    pub data: i32,
}

/// Data updated at each graphical step.
///
/// They mostly refer to player’s car except for carCoordinates and carID,
/// which refer to the cars currently on track.
#[repr(C, packed(4))]
#[derive(Clone, Debug)]
pub struct SPageFileGraphic {
    /// Current step index
    pub packet_id: i32,
    /// See enums ACC_STATUS
    pub status: StatusRaw,
    /// See enums ACC_SESSION_TYPE
    pub session: SessionTypeRaw,
    /// Current lap time in wide character
    pub current_time: [u16; 15],
    /// Last lap time in wide character
    pub last_time: [u16; 15],
    /// Best lap time in wide character
    pub best_time: [u16; 15],
    /// Last split time in wide character
    pub split: [u16; 15],
    /// No of completed laps
    pub completed_laps: i32,
    /// Current player position
    pub position: i32,
    /// Current lap time in milliseconds
    pub i_current_time: i32,
    /// Last lap time in milliseconds
    pub i_last_time: i32,
    /// Best lap time in milliseconds
    pub i_best_time: i32,
    /// Session time left
    pub session_time_left: f32,
    /// Distance travelled in the current stint
    pub distance_traveled: f32,
    /// Car is pitting
    pub is_in_pit: i32,
    /// Current track sector
    pub current_sector_index: i32,
    /// Last sector time in milliseconds
    pub last_sector_time: i32,
    /// Number of completed laps
    pub number_of_laps: i32,
    /// Tyre compound used
    pub tyre_compound: [u16; 33],
    /// Not used in ACC
    pub replay_time_multiplier: f32,
    /// Car position on track spline (0.0 start to 1.0 finish)
    pub normalized_car_position: f32,
    /// Number of cars on track
    pub active_cars: i32,
    /// Coordinates of cars on track
    pub car_coordinates: [[f32; 3]; 60],
    /// Car IDs of cars on track
    pub car_id: [i32; 60],
    /// Player Car ID
    pub player_car_id: i32,
    /// Penalty time to wait
    pub penalty_time: f32,
    /// See enums [Flag]
    pub flag: FlagTypeRaw,
    /// See enums ACC_PENALTY_TYPE
    pub penalty: PenaltyRaw,
    /// Ideal line on
    pub ideal_line_on: i32,
    /// Car is in pit lane
    pub is_in_pit_lane: i32,
    /// Ideal line friction coefficient
    pub surface_grip: f32,
    /// Mandatory pit is completed
    pub mandatory_pit_done: i32,
    /// Wind speed in m/s
    pub wind_speed: f32,
    /// wind direction in radians
    pub wind_direction: f32,
    /// Car is working on setup
    pub is_setup_menu_visible: i32,
    /// current car main display index, see Appendix 1
    pub main_display_index: i32,
    /// current car secondary display index
    pub secondary_display_index: i32,
    /// Traction control level
    pub tc: i32,
    /// Traction control cut level
    pub tc_cut: i32,
    /// Current engine map
    pub engine_map: i32,
    /// ABS level
    pub abs: i32,
    /// Average fuel consumed per lap in liters
    pub fuel_used_per_lap: f32,
    /// Rain lights on
    pub rain_lights: i32,
    /// Flashing lights on
    pub flashing_lights: i32,
    /// Current lights stage
    pub lights_stage: i32,
    /// Exhaust temperature
    pub exhaust_temperature: f32,
    /// Current wiper stage
    pub wiper_lv: i32,
    /// Time the driver is allowed to drive/race (ms)
    pub driver_stint_total_time_left: i32,
    /// Time the driver is allowed to drive/stint (ms)
    pub driver_stint_time_left: i32,
    /// Are rain tyres equipped
    pub rain_tyres: i32,
    /// todo: what is this
    pub session_index: i32,
    /// Used fuel since last time refueling
    pub used_fuel: f32,
    /// Delta time in wide character
    pub delta_lap_time: [u16; 15],
    /// Delta time time in milliseconds
    pub i_delta_lap_time: i32,
    /// Estimated lap time in milliseconds
    pub estimated_lap_time: [u16; 15],
    /// Estimated lap time in wide character
    pub i_estimated_lap_time: i32,
    /// Delta positive (1) or negative (0)
    pub is_delta_positive: i32,
    /// Last split time in milliseconds
    pub i_split: i32,
    /// Check if Lap is valid for timing
    pub is_valid_lap: i32,
    /// Laps possible with current fuel level
    pub fuel_estimated_laps: f32,
    /// Status of track
    pub track_status: [u16; 33],
    /// Mandatory pitstops the player still has to do
    pub missing_mandatory_pits: i32,
    /// Time of day in seconds
    pub clock: f32,
    /// Is Blinker left on
    pub direction_lights_left: i32,
    /// Is Blinker right on
    pub direction_lights_right: i32,
    /// Yellow Flag is out?
    pub global_yellow: i32,
    /// Yellow Flag in Sector 1 is out?
    pub global_yellow1: i32,
    /// Yellow Flag in Sector 2 is out?
    pub global_yellow2: i32,
    /// Yellow Flag in Sector 3 is out?
    pub global_yellow3: i32,
    /// White Flag is out?
    pub global_white: i32,
    /// Green Flag is out?
    pub global_green: i32,
    /// Checkered Flag is out?
    pub global_chequered: i32,
    /// Red Flag is out?
    pub global_red: i32,
    /// Number of tyre set on the MFD
    pub mfd_tyre_set: i32,
    /// How much fuel to add on the MFD
    pub mfd_fuel_to_add: f32,
    /// Tyre pressure left front on the MFD
    pub mfd_tyre_pressure_lf: f32,
    /// Tyre pressure right front on the MFD
    pub mfd_tyre_pressure_rf: f32,
    /// Tyre pressure left rear on the MFD
    pub mfd_tyre_pressure_lr: f32,
    /// Tyre pressure right rear on the MFD
    pub mfd_tyre_pressure_rr: f32,
    /// See enums ACC_TRACK_GRIP_STATUS
    pub track_grip_status: TrackGripStatusRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity: RainIntensityRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity_in_10m: RainIntensityRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity_in_30m: RainIntensityRaw,
    /// Tyre Set currently in use
    pub current_tyre_set: i32,
    /// Next Tyre set per strategy
    pub strategy_tyre_set: i32,
    /// Distance in ms to car in front
    pub gap_ahead: i32,
    /// Distance in ms to car behind
    pub gap_behind: i32,
}

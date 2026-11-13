const STRING_LENGTH_MAX: usize = 64;
const STORED_PARTICIPANTS_MAX: usize = 64;
const TYRE_MAX: usize = 4;
const VEC_MAX: usize = 3;
const TYRE_COMPOUND_NAME_LENGTH_MAX: usize = 40;

#[repr(C)]
pub enum GameState {
    Exited = 0,
    FrontEnd,
    InGamePlaying,
    InGamePaused,
    InGameIMenuTimeTicking,
    InGameRestarting,
    InGameReplay,
    FrontEndReplay,
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(f, "{}",
            match self {
                Self::Exited => "Exited",
                Self::FrontEnd => "FrontEnd",
                Self::InGamePlaying => "InGamePlaying",
                Self::InGamePaused => "InGamePaused",
                Self::InGameIMenuTimeTicking => "InGameIMenuTimeTicking",
                Self::InGameRestarting => "InGameRestarting",
                Self::InGameReplay => "InGameReplay",
                Self::FrontEndReplay => "FrontEndReplay",
            }
        ) {
            Ok(()) => Ok(()),
            Err(_) => Err(std::fmt::Error)
        }
    }
}

impl From<u32> for GameState {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::FrontEnd,
            2 => Self::InGamePlaying,
            3 => Self::InGamePaused,
            4 => Self::InGameIMenuTimeTicking,
            5 => Self::InGameRestarting,
            6 => Self::InGameReplay,
            7 => Self::FrontEndReplay,
            _ => Self::Exited,
        }
    }
}

#[derive(Copy, Clone)]
pub enum SessionState {
    SessionInvalid = 0,
    SessionPractice,
    SessionTest,
    SessionQualify,
    SessionFormationLap,
    SessionRace,
    SessionTimeAttack,
}

impl From<u32> for SessionState {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::SessionPractice,
            2 => Self::SessionTest,
            3 => Self::SessionQualify,
            4 => Self::SessionFormationLap,
            5 => Self::SessionRace,
            6 => Self::SessionTimeAttack,
            _ => Self::SessionInvalid,
        }
    }
}

impl PartialEq for SessionState {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }

    fn ne(&self, other: &Self) -> bool {
        *self as u32 != *other as u32
    }
}

impl std::fmt::Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(f, "{}",
            match self {
                Self::SessionPractice => "SessionPractice",
                Self::SessionTest => "SessionTest",
                Self::SessionQualify => "SessionQualify",
                Self::SessionFormationLap => "SessionFormationLap",
                Self::SessionRace => "SessionRace",
                Self::SessionTimeAttack => "SessionTimeAttack",
                Self::SessionInvalid => "SessionInvalid",
            }
        ) {
            Ok(()) => Ok(()),
            Err(_) => Err(std::fmt::Error)
        }
    }
}

#[derive(Clone)]
#[repr(C, packed(4))]
pub struct ParticipantInfoSharedMemory {
    // pub is_active: i32, // bool // this is WRONG
    pub is_active: u8,
    pub name: [u8; STRING_LENGTH_MAX], // need string max + 4?y
    pub world_position: [f32; VEC_MAX],
    pub current_lap_distance: f32,
    pub race_position: u32,
    pub laps_completed: u32,
    pub current_lap: u32,
    pub current_sector: u32,
}

impl Default for ParticipantInfoSharedMemory {
    fn default() -> Self {
        ParticipantInfoSharedMemory {
            is_active: 0,
            name: ['\0' as u8; STRING_LENGTH_MAX],
            world_position: [0_f32; VEC_MAX],
            current_lap_distance: 0_f32,
            race_position: 0,
            laps_completed: 0,
            current_lap: 0,
            current_sector: 0
        }
    }
}

pub struct ParticipantInfo {
    pub is_active: bool,
    pub name: String,
    pub world_position: [f32; VEC_MAX],
    pub current_lap_distance: f32,
    pub race_position: u32,
    pub laps_completed: u32,
    pub current_lap: u32,
    pub current_sector: u32,
}

impl From<&ParticipantInfoSharedMemory> for ParticipantInfo {
    fn from(value: &ParticipantInfoSharedMemory) -> Self {
        ParticipantInfo {
            is_active: value.is_active != 0,
            name: std::ffi::CStr::from_bytes_until_nul(&value.name)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            world_position: value.world_position,
            current_lap_distance: value.current_lap_distance,
            race_position: value.race_position,
            laps_completed: value.laps_completed,
            current_lap: value.current_lap,
            current_sector: value.current_sector,
        }
    }
}

/// Project Cars 2 Shared Memory MApping
#[repr(C, packed(4))]
pub struct ProjectCars2 {
    // Version number
    pub version: u32,
    pub build_version_number: u32,

    // Game states
    pub game_state: u32,
    pub session_state: u32,
    pub race_state: u32,

    // Participant info
    pub viewed_participant_index: i32,
    pub num_participants: i32,
    pub participant_info: [ParticipantInfoSharedMemory; STORED_PARTICIPANTS_MAX],

    // Unfiltered input
    pub unfiltered_throttle: f32,
    pub unfiltered_brake: f32,
    pub unfiltered_steering: f32,
    pub unfiltered_clutch: f32,

    // Vehicle information
    pub car_name: [u8; STRING_LENGTH_MAX],
    pub car_class_name: [u8; STRING_LENGTH_MAX],

    // Event information
    pub laps_in_event: u32,
    pub track_location: [u8; STRING_LENGTH_MAX],
    pub track_variation: [u8; STRING_LENGTH_MAX],
    pub track_length: f32,

    // Timings
    pub num_sectors: i32,
    // pub lap_invalidated: i32, // bool
    pub lap_invalidated: u8,
    pub best_lap_time: f32,
    pub last_lap_time: f32,
    pub current_time: f32,
    pub split_time_ahead: f32,
    pub split_time_behind: f32,
    pub split_time: f32,
    pub event_time_remaining: f32,
    pub personal_fastest_lap_time: f32,
    pub world_fastest_lap_time: f32,
    pub current_sector1_time: f32,
    pub current_sector2_time: f32,
    pub current_sector3_time: f32,
    pub fastest_sector1_time: f32,
    pub fastest_sector2_time: f32,
    pub fastest_sector3_time: f32,
    pub personal_fastest_sector1_time: f32,
    pub personal_fastest_sector2_time: f32,
    pub personal_fastest_sector3_time: f32,
    pub world_fastest_sector1_time: f32,
    pub world_fastest_sector2_time: f32,
    pub world_fastest_sector3_time: f32,

    // Flags
    pub highest_flag_colour: u32,
    pub highest_flag_reason: u32,

    // Pit info
    pub pit_mode: u32,
    pub pit_schedule: u32,

    // Car state
    pub car_flags: u32,
    pub oil_temp_celsius: f32,
    pub oil_pressure_k_pa: f32,
    pub water_temp_celsius: f32,
    pub water_pressure_k_pa: f32,
    pub fuel_pressure_k_pa: f32,
    pub fuel_level: f32,
    pub fuel_capacity: f32,
    pub speed: f32,
    pub rpm: f32,
    pub max_rpm: f32,
    pub brake: f32,
    pub throttle: f32,
    pub clutch: f32,
    pub steering: f32,
    pub gear: i32,
    pub num_gears: i32,
    pub odometer_km: f32,
    // pub anti_lock_active: i32, // bool
    pub anti_lock_active: u8,
    pub last_opponent_collision_index: i32,
    pub last_opponent_collision_magnitude: f32,
    // pub boost_active: i32, // bool
    pub boost_active: u8,
    pub boost_amount: f32,

    // Motion and device related
    pub orientation: [f32; VEC_MAX],
    pub local_velocity: [f32; VEC_MAX],
    pub world_velocity: [f32; VEC_MAX],
    pub angular_velocity: [f32; VEC_MAX],
    pub local_acceleration: [f32; VEC_MAX],
    pub world_acceleration: [f32; VEC_MAX],
    pub extents_centre: [f32; VEC_MAX],

    // Wheels/tires
    pub tyre_flags: [u32; TYRE_MAX],
    pub terrain: [u32; TYRE_MAX],
    pub tyre_y: [f32; TYRE_MAX],
    pub tyre_rps: [f32; TYRE_MAX],
    pub tyre_slip_speed: [f32; TYRE_MAX],
    pub tyre_temp: [f32; TYRE_MAX],
    pub _deprecated_tire_grip: [f32; TYRE_MAX],
    pub tyre_height_above_ground: [f32; TYRE_MAX],
    pub _deprecated_tire_lateral_stiffness: [f32; TYRE_MAX],
    pub tyre_wear: [f32; TYRE_MAX],
    pub brake_damage: [f32; TYRE_MAX],
    pub suspension_damage: [f32; TYRE_MAX],
    pub brake_temp_celsius: [f32; TYRE_MAX],
    pub tyre_tread_temp: [f32; TYRE_MAX],
    pub tyre_layer_temp: [f32; TYRE_MAX],
    pub tyre_carcass_temp: [f32; TYRE_MAX],
    pub tyre_rim_temp: [f32; TYRE_MAX],
    pub tyre_internal_air_temp: [f32; TYRE_MAX],

    // Car damage
    pub crash_state: u32,
    pub aero_damage: f32,
    pub engine_damage: f32,

    // Weather
    pub ambient_temperature: f32,
    pub track_temperature: f32,
    pub rain_density: f32,
    pub wind_speed: f32,
    pub wind_direction_x: f32,
    pub wind_direction_y: f32,
    pub cloud_brightness: f32,

    // PCars2 additions start, version 8

    /// Sequence number to help slightly with data integrity needs
    pub sequence_number: u32,

    // Additional car data
    pub wheel_local_position_y: [f32; TYRE_MAX],
    pub suspension_travel: [f32; TYRE_MAX],
    pub suspension_velocity: [f32; TYRE_MAX],
    pub air_pressure: [f32; TYRE_MAX],
    pub engine_speed: f32,
    pub engine_torque: f32,
    pub wings: [f32; 2],
    pub hand_brake: f32,

    // Additional race data
    pub current_sector1_times: [f32; STORED_PARTICIPANTS_MAX],
    pub current_sector2_times: [f32; STORED_PARTICIPANTS_MAX],
    pub current_sector3_times: [f32; STORED_PARTICIPANTS_MAX],
    pub fastest_sector1_times: [f32; STORED_PARTICIPANTS_MAX],
    pub fastest_sector2_times: [f32; STORED_PARTICIPANTS_MAX],
    pub fastest_sector3_times: [f32; STORED_PARTICIPANTS_MAX],
    pub fastest_lap_times: [f32; STORED_PARTICIPANTS_MAX],
    pub last_lap_times: [f32; STORED_PARTICIPANTS_MAX],
    // pub laps_invalidated: [i32; STORED_PARTICIPANTS_MAX], // bool
    pub laps_invalidated: [u8; STORED_PARTICIPANTS_MAX], // bool
    pub race_states: [u32; STORED_PARTICIPANTS_MAX],
    pub pit_modes: [u32; STORED_PARTICIPANTS_MAX],
    pub orientations: [f32; STORED_PARTICIPANTS_MAX * VEC_MAX],
    pub speeds: [f32; STORED_PARTICIPANTS_MAX],
    pub car_names: [u8; STORED_PARTICIPANTS_MAX * STRING_LENGTH_MAX],
    pub car_class_names: [u8; STORED_PARTICIPANTS_MAX * STRING_LENGTH_MAX],

    // ???
    pub enforced_pit_stop_lap: i32,
    pub translated_track_location: [u8; STRING_LENGTH_MAX],
    pub translated_track_variation: [u8; STRING_LENGTH_MAX],
    pub brake_bias: f32,
    pub turbo_boost_pressure: f32,
    pub tyre_compound: [u8; TYRE_MAX * TYRE_COMPOUND_NAME_LENGTH_MAX],
    pub pit_schedules: [u32; STORED_PARTICIPANTS_MAX],
    pub highest_flag_colours: [u32; STORED_PARTICIPANTS_MAX],
    pub highest_flag_reasons: [u32; STORED_PARTICIPANTS_MAX],
    pub nationalities: [u32; STORED_PARTICIPANTS_MAX],
    pub snow_density: f32,

    // AMS2 additions (v10...)

    // Session info
    pub session_duration: f32,
    pub session_additional_laps: i32,

    // Tires
    pub tyre_temp_left: [f32; TYRE_MAX],
    pub tyre_temp_center: [f32; TYRE_MAX],
    pub tyre_temp_right: [f32; TYRE_MAX],

    // DRS
    pub drs_state: u32,

    // Suspension
    pub ride_height: [f32; TYRE_MAX],

    // Input
    pub joy_pad0: u32,
    pub d_pad: u32,

    // ABS/TC
    pub anti_lock_setting: i32,
    pub traction_control_setting: i32,

    // ERS
    pub ers_deployment_mode: u32,
    pub ers_auto_mode_enabled: i32, // bool

    // Clutch state and damage
    pub clutch_temp: f32,
    pub clutch_wear: f32,
    pub clutch_overheated: i32, // bool
    pub clutch_slipping: i32, // bool

    // Yellow flag
    pub yellow_flag_state: i32,

    //
    pub session_is_private: i32, // bool
    pub launch_stage: i32,
}
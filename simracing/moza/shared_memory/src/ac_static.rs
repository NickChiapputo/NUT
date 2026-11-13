/// Data that never changes during a session.
#[repr(C, packed(4))]
#[derive(Clone, Debug)]
pub struct SPageFileStatic {
    /// Shared memory version
    pub sm_version: [u16; 15],
    /// Assetto Corsa version
    pub ac_version: [u16; 15],
    /// Number of sessions
    pub number_of_sessions: i32,
    /// Number of cars
    pub num_cars: i32,
    /// Player car model see Appendix 2
    pub car_model: [u16; 33],
    /// Track name
    pub track: [u16; 33],
    /// Player name
    pub player_name: [u16; 33],
    /// Player surname
    pub player_surname: [u16; 33],
    /// Player nickname
    pub player_nick: [u16; 33],
    /// Number of sectors
    pub sector_count: i32,
    /// Not shown in ACC
    pub max_torque: f32,
    /// Not shown in ACC
    pub max_power: f32,
    /// Maximum rpm
    pub max_rpm: i32,
    /// Maximum fuel tank capacity
    pub max_fuel: f32,
    /// Not shown in ACC
    pub suspension_max_travel: [f32; 4],
    /// Not shown in ACC
    pub tyre_radius: [f32; 4],
    /// Maximum turbo boost
    pub max_turbo_boost: f32,
    /// unused
    pub deprecated_1: f32,
    /// unused
    pub deprecated_2: f32,
    /// Penalties enabled
    pub penalties_enabled: i32,
    /// Fuel consumption rate
    pub aid_fuel_rate: f32,
    /// Tire wear rate
    pub aid_tire_rate: f32,
    /// Mechanical damage rate
    pub aid_mechanical_damage: f32,
    /// Not allowed in Blancpain endurance series
    pub aid_allow_tyre_blankets: f32,
    /// Stability control used
    pub aid_stability: f32,
    /// Auto clutch used
    pub aid_auto_clutch: i32,
    /// Always true in ACC
    pub aid_auto_blip: i32,
    /// Not used in ACC
    pub has_drs: i32,
    /// Not used in ACC
    pub has_ers: i32,
    /// Not used in ACC
    pub has_kers: i32,
    /// Not used in ACC
    pub kers_max_j: f32,
    /// Not used in ACC
    pub engine_brake_settings_count: i32,
    /// Not used in ACC
    pub ers_power_controller_count: i32,
    /// Not used in ACC
    pub track_spline_length: f32,
    /// Not used in ACC
    pub track_configuration: [u16; 33],
    /// Not used in ACC
    pub ers_max_j: f32,
    /// Not used in ACC
    pub is_timed_race: i32,
    /// Not used in ACC
    pub has_extra_lap: i32,
    /// Not used in ACC
    pub car_skin: [u16; 33],
    /// Not used in ACC
    pub reversed_grid_positions: i32,
    /// Pit window opening time
    pub pit_window_start: i32,
    /// Pit windows closing time
    pub pit_window_end: i32,


    // Not sure if anything below this is valid. Doesn't match the ACSharedDocumentation file


    /// If is a multiplayer session
    pub is_online: i32,
    /// Name of the dry tyres
    pub dry_tyres_name: [u16; 33],
    /// Name of the wet tyres
    pub wet_tyres_name: [u16; 33],
}

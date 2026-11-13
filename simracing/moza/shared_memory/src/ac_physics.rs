/// Data changes at each graphic step.
///
/// All data refers to the player’s car.
#[repr(C, packed(4))]
#[derive(Clone, Debug)]
pub struct SPageFilePhysics {
    /// Current step index
    pub packet_id: i32,
    /// Gas pedal input value (from -0 to 1.0)
    pub gas: f32,
    /// Brake pedal input value (from -0 to 1.0)
    pub brake: f32,
    /// Amount of fuel remaining in kg
    pub fuel: f32,
    /// Current gear
    pub gear: i32,
    /// Engine revolutions per minute
    pub rpm: i32,
    /// Steering input value (from -1.0 to 1.0)
    pub steer_angle: f32,
    /// Car speed in km/h
    pub speed_kmh: f32,
    /// Car velocity vector in global coordinates
    pub velocity: [f32; 3],
    /// Car acceleration vector in global coordinates
    pub acc_g: [f32; 3],
    /// Tyre slip for each tyre [FL, FR, RL, RR]
    pub wheel_slip: [f32; 4],
    /// Wheel load for each tyre [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub wheel_load: [f32; 4],
    /// Tyre pressure [FL, FR, RL, RR]
    pub wheels_pressure: [f32; 4],
    /// Wheel angular speed in rad/s [FL, FR, RL, RR]
    pub wheel_angular_speed: [f32; 4],
    /// Tyre wear [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_wear: [f32; 4],
    /// Dirt accumulated on tyre surface [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_dirty_level: [f32; 4],
    /// Tyre rubber core temperature [FL, FR, RL, RR]
    pub tyre_core_temperature: [f32; 4],
    /// Wheels camber in radians [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub camber_rad: [f32; 4],
    /// Suspension travel [FL, FR, RL, RR]
    pub suspension_travel: [f32; 4],
    /// DRS on *(NOT SENT BY SIM)*
    pub drs: f32,
    /// TC in action
    pub tc: f32,
    /// Car yaw orientation
    pub heading: f32,
    /// Car pitch orientation
    pub pitch: f32,
    /// Car roll orientation
    pub roll: f32,
    /// Centre of gravity height *(NOT SENT BY SIM)*
    pub cg_height: f32,
    /// Car damage: front 0, rear 1, left 2, right 3, centre 4
    pub car_damage: [f32; 5],
    /// Number of tyres out of track *(NOT SENT BY SIM)*
    pub number_of_tyres_out: i32,
    /// Pit limiter is on
    pub pit_limiter_on: i32,
    /// ABS in action
    pub abs: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_charge: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_input: f32,
    /// Automatic transmission on
    pub auto_shifter_on: i32,
    /// Ride height: 0 front, 1 rear *(NOT SENT BY SIM)*
    pub ride_height: [f32; 2],
    /// Car turbo level
    pub turbo_boost: f32,
    /// Car ballast in kg / Not implemented *(NOT SENT BY SIM)*
    pub ballast: f32,
    /// Air density *(NOT SENT BY SIM)*
    pub air_density: f32,
    /// Air temperature
    pub air_temp: f32,
    /// Road temperature
    pub road_temp: f32,
    /// Car angular velocity vector in local coordinates
    pub local_angular_vel: [f32; 3],
    /// Force feedback signal
    pub final_ff: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub performance_meter: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub engine_brake: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_recovery_level: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_power_level: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_heat_charging: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_is_charging: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_current_kj: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub drs_available: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub drs_enabled: i32,
    /// Brake discs temperatures
    pub brake_temp: [f32; 4],
    /// Clutch pedal input value (from -0 to 1.0)
    pub clutch: f32,
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_i: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_m: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_o: [f32; 4],
    /// Car is controlled by the AI
    pub is_ai_controlled: i32,
    /// Tyre contact point global coordinates [FL, FR, RL, RR]
    pub tyre_contact_point: [[f32; 3]; 4],
    /// Tyre contact normal [FL, FR, RL, RR] [x,y,z]
    pub tyre_contact_normal: [[f32; 3]; 4],
    /// Tyre contact heading [FL, FR, RL, RR] [x,y,z]
    pub tyre_contact_heading: [[f32; 3]; 4],
    /// Front brake bias, see Appendix 4
    pub brake_bias: f32,
    /// Car velocity vector in local coordinates
    pub local_velocity: [f32; 3],


    // Not sure if anything below this is valid. Doesn't match the ACSharedDocumentation file


    /// Not used in ACC *(NOT SENT BY SIM)*
    pub p2p_activations: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub p2p_status: i32,
    /// Maximum engine rpm *(NOT SENT BY SIM)*
    pub current_max_rpm: i32,
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub mz: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub fx: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub fy: [f32; 4],
    /// Tyre slip ratio [FL, FR, RL, RR] in radians
    pub slip_ratio: [f32; 4],
    /// Tyre slip angle [FL, FR, RL, RR]
    pub slip_angle: [f32; 4],
    /// TC in action *(NOT SENT BY SIM)*
    pub tc_in_action: i32,
    /// ABS in action *(NOT SENT BY SIM)*
    pub abs_in_action: i32,
    /// Suspensions damage levels [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub suspension_damage: [f32; 4],
    /// Tyres core temperatures [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_temp: [f32; 4],
    /// Water Temperature
    pub water_temp: f32,
    /// Brake pressure [FL, FR, RL, RR] see Appendix 2
    pub brake_pressure: [f32; 4],
    /// Brake pad compund front
    pub front_brake_compound: i32,
    /// Brake pad compund rear
    pub rear_brake_compound: i32,
    /// Brake pad wear [FL, FR, RL, RR]
    pub pad_life: [f32; 4],
    /// Brake disk wear [FL, FR, RL, RR]
    pub disc_life: [f32; 4],
    /// Ignition switch set to on?
    pub ignition_on: i32,
    /// Starter Switch set to on?
    pub starter_engine_on: i32,
    /// Engine running?
    pub is_engine_running: i32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub kerb_vibration: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub slip_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub g_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub abs_vibrations: f32,
}

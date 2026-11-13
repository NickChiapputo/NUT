use crate::PCars2Data;
use crate::pcars2;

#[unsafe(no_mangle)]
pub extern "C" fn start_pcars_library() -> *mut PCars2Data {
    match PCars2Data::new() {
        Ok(data) => Box::into_raw(Box::new(data)),
        Err(_) => std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_pcars2_data(data: *mut PCars2Data) {
    if data.is_null() {
        return;
    }

    unsafe{
        let _ = Box::from_raw(data);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_is_in_game(data: *mut PCars2Data) -> u32 {
    if unsafe {&*data}.is_in_game() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_game_state(data: *mut PCars2Data) -> pcars2::GameState {
    unsafe {&*data}.game_state()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_unfiltered_throttle(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.unfiltered_throttle()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_throttle(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.throttle()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_unfiltered_brake(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.unfiltered_brake()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_unfiltered_steering(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.unfiltered_steering()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_unfiltered_clutch(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.unfiltered_clutch()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_active_participant_current_lap_distance(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.get_active_participant().current_lap_distance
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_track_length(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.track_length()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_current_lap_time_s(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.current_lap_time()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_last_lap_time_s(data: *mut PCars2Data) -> f32 {
    unsafe {&*data}.last_lap_time()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_active_participant_current_lap(data: *mut PCars2Data) -> u32 {
    unsafe {&*data}.get_active_participant().current_lap
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pcars2_current_lap_is_valid(data: *mut PCars2Data) -> i32 {
    if unsafe {&*data}.is_lap_valid() { 1 } else { 0 }
}

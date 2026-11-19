/// Estimates the time gap between two cars.
///
/// `lap_a` / `lap_b`: zero-based current lap numbers
/// `location_a` / `location_b`: track locations in [0.0, track_length_m)
/// `speed_a` / `speed_b`: speed in meters per second
/// `track_length_m`: track length in meters
///
/// Returns:
///   positive => A is ahead of B
///   negative => B is ahead of A
///
/// The estimate uses the speed of the car behind to convert distance to time.
pub fn calculate(
    lap_a: u32,
    location_a: f32,
    speed_a: f32,
    lap_b: u32,
    location_b: f32,
    speed_b: f32,
    track_length_m: f32,
) -> Option<f32> {
    if !location_a.is_finite()
        || !location_b.is_finite()
        || !speed_a.is_finite()
        || !speed_b.is_finite()
        || !track_length_m.is_finite()
        || track_length_m <= 0.0
        || !(0.0..track_length_m).contains(&location_a)
        || !(0.0..track_length_m).contains(&location_b)
        || speed_a <= 0.0
        || speed_b <= 0.0
    {
        return None;
    }

    let location_a_normalized = location_a / track_length_m;
    let location_b_normalized = location_b / track_length_m;

    // Unwrapped progress in laps. Lap counters must use the same convention.
    let progress_a = lap_a as f32 + location_a_normalized;
    let progress_b = lap_b as f32 + location_b_normalized;
    let distance_gap_m = (progress_a - progress_b) * track_length_m;

    // Use the trailing car's speed to estimate the time needed to cover the gap.
    let trailing_speed = if distance_gap_m >= 0.0 {
        speed_b
    } else {
        speed_a
    };

    Some(distance_gap_m / trailing_speed)
}
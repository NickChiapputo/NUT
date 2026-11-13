use moza;
use shared_memory::{self, PCars2Data};
use overlay::OverlayResult;

use std::io::Write;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;

const PORT: &str = "/dev/ttyACM0";

fn main() -> OverlayResult {
    let kill_signal = setup_sigint_handler();

    let mut wheel = moza::MozaWheel::from_port(PORT);
    wheel.set_rpm_indicator_mode(moza::RPMIndicatorMode::Rpm);
    wheel.set_rpm_display_mode(moza::RPMDisplayMode::SingleColor);
    wheel.set_rpm_thresholds(&[
        80.0,
        83.0,
        86.0,
        89.0,
        91.0,
        92.0,
        93.0,
        94.0,
        96.0,
        97.0
    ]);
    wheel.set_rpm_brightness(6);

    // println!("RPM LED Colors: {:?}", wheel.get_rpm_colors().expect("Failed to get RPM LED colors"));
    // println!("Button Colors:  {:?}", wheel.get_button_colors().expect("Failed to get button colors"));
    wheel.set_blinking_rpm_color(&[0, 0, 255]);
    // wheel.set_rpm_leds_from_rpm(10, 10);
    // let rpms: Vec<u16> = wheel.get_rpms().expect("Failed to get RPM values")
    //     .iter()
    //     .map(|rpm_at_i| (rpm_at_i[0] as u16) << 8 | (rpm_at_i[1] as u16))
    //     .collect();
    // println!("RPMs: {:?}", rpms);
    // println!("RPM Timings: {:?}", wheel.get_rpm_timings().expect("Failed to get RPM timings"));



    let is_ac = false;

    if is_ac {
        eprintln!("Telemetry is not initialized; exiting.");
        return Ok(());
    }


    if !is_ac {
        println!("Waiting for shared memory to be available...");
        let pcars_data: PCars2Data = load_pcars_data_blocking(&kill_signal)
            .expect("Failed to load PCars data.");

        // let pcars_data = shared_memory::PCars2Data::new().expect("Failed to load pcars data");
        println!("  pcars2 data: {}", pcars_data.version());
        overlay::run(pcars_data, wheel)

        /*
        while !kill_signal.load(Ordering::SeqCst) {
            if pcars_data.is_in_game() {
                let lap = Lap::from_data(&pcars_data);

                if pcars_data.session_state() != session_state {
                    session_state = pcars_data.session_state();
                    println!("Starting {}", session_state);
                }

                print!("{}[2K", 27 as char);
                println!(
                    "RPM: {:>04}/{:>04}",
                    pcars_data.rpm() as i32,
                    pcars_data.max_rpm() as i32
                );

                print!("{}[2K", 27 as char);
                println!("Current Lap: {}", lap);

                print!("{}[2A", 27 as char);

                if lap.time < current_lap.time {
                    // starting a new lap
                    current_lap.sector_3_time = pcars_data.sector_3_time();
                    current_lap.time = pcars_data.last_lap_time();
                    println!("Lap {}: {}", session.len()+1, current_lap);
                    session.push(current_lap);
                }

                current_lap = Lap::from(lap);

                let rpm = match pcars_data.is_pit_limited() {
                    true => pcars_data.max_rpm(),
                    false => pcars_data.rpm()
                };
                wheel.set_rpm_leds_from_rpm(rpm as i32, pcars_data.max_rpm() as i32);
            } else {
                session.clear();
                print!("{}[2K", 27 as char);
                println!("Game state: {}, sequence_number: {}", pcars_data.game_state(), pcars_data.sequence_number());
                print!("{}[1A", 27 as char);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        */
    } else {
        let data = shared_memory::ACData::new().expect("Failed to load AC shared data");
        if !data.is_valid() {
            eprintln!("Invalid data. Exiting :(");
            return Ok(());
        }

        while !kill_signal.load(Ordering::SeqCst) {
            if !data.is_valid() {
                break;
            }


            print!(
                "[{}::{}] {}, RPM: {:>04}/{:>04}, abs {}, yellow? {:?}\r",
                data.get_status(),
                data.get_session_type(),
                data.get_flag(),
                data.get_rpms(),
                data.get_max_rpm(),
                data.abs(),
                data.is_yellow(),
            );
            std::io::stdout().flush().unwrap();
            let rpm = match data.is_pit_limited() {
                true => data.get_max_rpm(),
                false => data.get_rpms()
            };
            wheel.set_rpm_leds_from_rpm(rpm, data.get_max_rpm());

            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        eprintln!("PCars2 Telemetry is not initialized; exiting.");
        return Ok(());
    }

    // println!("\nfinished...");
}

fn load_pcars_data_blocking(kill_signal: &Arc<AtomicBool>) -> Result<PCars2Data, ()> {
    while !kill_signal.load(Ordering::SeqCst) {
        if let Ok(pcars_data) = PCars2Data::new() {
            return Ok(pcars_data);
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Err(())
}

fn setup_sigint_handler() -> Arc<AtomicBool> {
    let kill_signal = Arc::new(AtomicBool::new(false));
    let signal_clone = kill_signal.clone();

    ctrlc::set_handler(move || {
        signal_clone.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler.");

    kill_signal
}

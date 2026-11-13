use serialport::SerialPort;
use std::io::{Write, Read};
use moza_command::{MozaCommand, MozaCommandID};

mod moza_command;
mod rpm_indicator_mode;
mod rpm_display_mode;

pub use rpm_indicator_mode::RPMIndicatorMode;
pub use rpm_display_mode::RPMDisplayMode;


const BAUDRATE: u32 = 115_200;
const MOZA_DEVICE_ID: u8 = 0x17;
const MAX_BRIGHTNESS: u8 = 15;
const MOZA_NUM_LEDS: usize = 10;
const TELEMETRY_BLINK_BIT_INDEX: usize = 15;
const MAX_READ_RESPONSE_RETRIES: usize = 3;

pub struct MozaWheel {
    serial_port: String,
    rpm_thresholds: [f32; MOZA_NUM_LEDS],
    blink_on_max_rpm: bool,
}

impl MozaWheel {
    pub fn from_port(serial_port: &str) -> Self {
        MozaWheel{
            serial_port: serial_port.to_string(),
            rpm_thresholds: [0.0; MOZA_NUM_LEDS],
            blink_on_max_rpm: true,
        }
    }

    pub fn set_rpm_indicator_mode(&self, mode: RPMIndicatorMode) {
        self.open_port_and_send_message(
            &MozaCommand::generate_with_payload(
                MOZA_DEVICE_ID,
                MozaCommandID::WheelRPMIndicatorMode,
                &[mode as u8]
            ),
            false
        );
    }

    pub fn set_rpm_display_mode(&self, mode: RPMDisplayMode) {
        self.open_port_and_send_message(
            &MozaCommand::generate_with_payload(
                MOZA_DEVICE_ID,
                MozaCommandID::WheelSetRPMDisplayMode,
                &[mode as u8]
            ),
            false
        );
    }

    pub fn set_rpm_brightness(&self, brightness: u8) {
        self.open_port_and_send_message(
            &MozaCommand::generate_with_payload(
                MOZA_DEVICE_ID,
                MozaCommandID::WheelRPMBrightness,
                &[u8::min(brightness, MAX_BRIGHTNESS)]
            ),
            false
        );
    }

    pub fn set_rpm_leds(&self, led_statuses: &[bool; MOZA_NUM_LEDS]) {
        self.open_port_and_send_message(
            &self.generate_telemetry_command_with_leds(led_statuses),
            false
        );
    }

    pub fn set_rpm_led_at_index(&self, index: usize) {
        assert!(index < MOZA_NUM_LEDS);
        let mut command = MozaCommand::generate(
            MOZA_DEVICE_ID,
            MozaCommandID::WheelSendTelemetry
        );
        command.set_payload_bit(index, true);

        self.open_port_and_send_message(&command, false);
    }

    pub fn set_rpm_leds_from_rpm(&self, rpm: i32, max_rpm: i32) {
        let rpm_percent = 100.0 * (rpm as f32 / max_rpm as f32);
        let mut led_statuses = [false; MOZA_NUM_LEDS];
        for (index, led_status) in led_statuses.iter_mut().enumerate().take(MOZA_NUM_LEDS) {
            *led_status = rpm_percent >= self.rpm_thresholds[index];
        }

        let mut command = self.generate_telemetry_command_with_leds(&led_statuses);
        let shift_indicator_blink = self.blink_on_max_rpm && led_statuses.iter().all(|&x| x);
        command.set_payload_bit(TELEMETRY_BLINK_BIT_INDEX, shift_indicator_blink);
        self.open_port_and_send_message(&command, false);
    }

    pub fn set_rpm_thresholds(&mut self, thresholds: &[f32; MOZA_NUM_LEDS]) {
        self.rpm_thresholds = *thresholds;
    }

    pub fn set_blinking_rpm_color_at(&self, index:usize, color:&[u8; 3]) {
        self.open_port_and_send_message(
            &MozaCommand::generate_with_payload(
                MOZA_DEVICE_ID,
                MozaCommandID::index_to_rpm_blink_color(index),
                color
            ),
            false
        );
    }

    pub fn set_blinking_rpm_color(&self, color:&[u8; 3]) {
        let _ = (0..10).map(|index| {
            self.set_blinking_rpm_color_at(index, color)
        });
    }

    pub fn get_rpm_indicator_mode(&self) -> Result<RPMIndicatorMode, ()> {
        self.get_response_for_command(MozaCommandID::WheelRPMIndicatorMode)?[0].try_into()
    }

    pub fn get_rpm_display_mode(&self) -> Result<RPMDisplayMode, ()> {
        self.get_response_for_command(MozaCommandID::WheelGetRPMDisplayMode)?[0].try_into()
    }

    pub fn get_rpm_color_at(&self, index:usize) -> Result<Vec<u8>, ()> {
        let command_id = match index {
            0 => MozaCommandID::WheelRPMColor1,
            1 => MozaCommandID::WheelRPMColor2,
            2 => MozaCommandID::WheelRPMColor3,
            3 => MozaCommandID::WheelRPMColor4,
            4 => MozaCommandID::WheelRPMColor5,
            5 => MozaCommandID::WheelRPMColor6,
            6 => MozaCommandID::WheelRPMColor7,
            7 => MozaCommandID::WheelRPMColor8,
            8 => MozaCommandID::WheelRPMColor9,
            _ => MozaCommandID::WheelRPMColor10,
        };
        Ok(self.get_response_for_command(command_id)?[..=2].to_vec())
    }

    pub fn get_rpm_colors(&self) -> Result<Vec<Vec<u8>>, ()> {
        (0..10).map(|index| {
            Ok(self.get_rpm_color_at(index))?
        }).collect()
    }

    pub fn get_button_color_at(&self, index:usize) -> Result<Vec<u8>, ()> {
        let command_id = match index {
            0 => MozaCommandID::WheelButtonColor1,
            1 => MozaCommandID::WheelButtonColor2,
            2 => MozaCommandID::WheelButtonColor3,
            3 => MozaCommandID::WheelButtonColor4,
            4 => MozaCommandID::WheelButtonColor5,
            5 => MozaCommandID::WheelButtonColor6,
            6 => MozaCommandID::WheelButtonColor7,
            7 => MozaCommandID::WheelButtonColor8,
            8 => MozaCommandID::WheelButtonColor9,
            _ => MozaCommandID::WheelButtonColor10,
        };
        Ok(self.get_response_for_command(command_id)?[..=2].to_vec())
    }

    pub fn get_button_colors(&self) -> Result<Vec<Vec<u8>>, ()> {
        (0..10).map(|index| {
            Ok(self.get_button_color_at(index))?
        }).collect()
    }

    pub fn get_rpms(&self) -> Result<Vec<Vec<u8>>, ()> {
        (0..10).map(|index| {
            Ok(self.get_rpm_at_index(index))?
        }).collect()
    }

    pub fn get_rpm_at_index(&self, index:usize) -> Result<Vec<u8>, ()> {
        self.get_response_for_command(MozaCommandID::index_to_rpm_value(index))
    }

    pub fn get_rpm_timings(&self) -> Result<Vec<u8>, ()> {
        self.get_response_for_command(MozaCommandID::WheelRPMTimings)
    }
}

impl MozaWheel {
    fn open_port(&self) -> serialport::TTYPort {
        let mut port = serialport::TTYPort::open(
            &serialport::new(&self.serial_port, BAUDRATE)
        ).unwrap_or_else(|_| panic!("Failed to open port {}", &self.serial_port));
        port.set_exclusive(false).unwrap();
        port.set_timeout(std::time::Duration::from_millis(500)).unwrap();
        port.set_data_bits(serialport::DataBits::Eight).unwrap();
        port.set_parity(serialport::Parity::None).unwrap();
        port.set_stop_bits(serialport::StopBits::One).unwrap();
        port.clear(serialport::ClearBuffer::All).expect("Failed to clear serial port buffer");
        port
    }

    fn open_port_and_send_message(&self, command: &moza_command::MozaCommand, read_command: bool) {
        self.open_port()
            .write_all(&command.generate_message(read_command))
            .expect("Failed to write command!");
    }

    fn generate_telemetry_command_with_leds(&self, led_statuses: &[bool; MOZA_NUM_LEDS]) -> MozaCommand {
        let mut command = MozaCommand::generate(
            MOZA_DEVICE_ID,
            MozaCommandID::WheelSendTelemetry
        );
        for (bit, &status) in led_statuses.iter().enumerate() {
            command.set_payload_bit(bit, status);
        }

        command
    }

    fn get_response_for_command(&self, command_id:MozaCommandID) -> Result<Vec<u8>, ()> {
        let command = MozaCommand::generate(MOZA_DEVICE_ID, command_id);
        let mut retries = 0;
        while retries < MAX_READ_RESPONSE_RETRIES {
            self.open_port_and_send_message(&command, true);
            match read_full_message(&mut self.open_port(), &command, &0.5) {
                Ok(result) => return Ok(result),
                Err(_) => retries += 1
            }
        }

        Err(())
    }
}

fn read_full_message(
    port:&mut serialport::TTYPort,
    command:&MozaCommand,
    timeout:&f32
) -> Result<Vec<u8>, ()> {
    let start = std::time::Instant::now();
    let start_byte = MozaCommand::start_byte();

    while !has_time_elapsed(&start, timeout) {
        wait_for_byte(port, &start_byte, timeout)?;

        let mut response = vec![0; command.length() - 1];
        if port.read_exact(&mut response).is_err() {
            println!("[read_full_message] Failed to read response command");
            return Err(());
        };
        response.reverse();
        response.push(start_byte);
        response.reverse();

        if command.is_valid_response(&response) {
            return Ok(command.get_response_payload(&response));
        }
    }

    println!("[read_full_message] Did not get response in time");

    Err(())
}

fn wait_for_byte(port:&mut serialport::TTYPort, byte:&u8, timeout:&f32) -> Result<(), ()> {
    let mut single_char_buf: Vec<u8> = vec![0_u8];
    let start = std::time::Instant::now();
    while !has_time_elapsed(&start, timeout) {
        if port.read_exact(&mut single_char_buf).is_err() {
            println!("[wait_for_byte] Failed to read next byte");
            return Err(());
        };
        if single_char_buf[0] == *byte {
            break;
        }
    }

    Ok(())
}

fn has_time_elapsed(start:&std::time::Instant, timeout:&f32) -> bool {
    (std::time::Instant::now() - *start) > std::time::Duration::from_secs_f32(*timeout)
}

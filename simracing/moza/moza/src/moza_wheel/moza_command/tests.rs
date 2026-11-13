use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_wheel_rpm_indicator_mode_write_command(mode: u8) -> Vec<u8> {
        let command: MozaCommand = MozaCommand::generate_with_payload(
            MOZA_DEVICE_ID,
            MozaCommandID::WheelRPMIndicatorMode,
            &vec![mode]
        );
        command.generate_message(false)
    }

    fn generate_wheel_rpm_brightness_write_command(brightness: u8) -> Vec<u8> {
        let command: MozaCommand = MozaCommand::generate_with_payload(
            MOZA_DEVICE_ID,
            MozaCommandID::WheelRPMBrightness,
            &vec![brightness]
        );
        command.generate_message(false)
    }

    fn generate_wheel_telemetry_write_command(led_statuses: [bool; 12]) -> Vec<u8> {
        let mut command: MozaCommand = MozaCommand::generate(
            MOZA_DEVICE_ID,
            MozaCommandID::WheelSendTelemetry
        );
        for (idx, status) in led_statuses.iter().enumerate() {
            command.set_payload_bit(idx, *status);
        }

        command.generate_message(false)
    }

    #[test]
    fn test_rpm_indicator_mode_message_length() {
        let message = generate_wheel_rpm_indicator_mode_write_command(1);
        let (command_id, payload_values_length, _, _) = MozaCommandID::WheelRPMIndicatorMode.into();
        let golden_message_length = MOZA_PROTOCOL_HEADER_LENGTH + command_id.len() + payload_values_length + 1;
        assert_eq!(message.len(), golden_message_length);
    }

    #[test]
    fn test_rpm_indicator_mode_message_header() {
        let message = generate_wheel_rpm_indicator_mode_write_command(1);
        let (command_id, payload_values_length, _, write_group) = MozaCommandID::WheelRPMIndicatorMode.into();

        assert_eq!(message[0], MOZA_START_BYTE);
        assert_eq!(message[1], (payload_values_length + command_id.len()) as u8);
        assert_eq!(message[2], write_group);
        assert_eq!(message[3], MOZA_DEVICE_ID);
    }

    #[test]
    fn test_rpm_indicator_mode_message_payload() {
        let payload = vec![1_u8];
        let message = generate_wheel_rpm_indicator_mode_write_command(1);
        let (command_id, payload_values_length, _, _) = MozaCommandID::WheelRPMIndicatorMode.into();

        let range_of_interest = MOZA_PROTOCOL_HEADER_LENGTH..(MOZA_PROTOCOL_HEADER_LENGTH + payload_values_length);
        let message_payload = message[range_of_interest].to_vec();

        for (&command_id_i, &message_i) in command_id.iter().zip(message_payload.iter()) {
            assert!(command_id_i == message_i);
        }

        for (&payload_i, &message_i) in payload.iter().zip(message_payload[command_id.len()..].iter()) {
            assert!(payload_i == message_i);
        }
    }

    #[test]
    fn test_rpm_indicator_mode_message_checksum() {
        let message = generate_wheel_rpm_indicator_mode_write_command(1);
        let checuksum = message[message.len() - 1];
        let golden_checksum: u8 = 0xe8;
        assert_eq!(checuksum, golden_checksum);
    }

    #[test]
    fn test_rpm_brightness_message_checksum() {
        let message = generate_wheel_rpm_brightness_write_command(255);
        let checuksum = message[message.len() - 1];
        let golden_checksum: u8 = 0xf7;
        assert_eq!(checuksum, golden_checksum);
    }


    #[test]
    fn test_telemetry_message_length() {
        let message = generate_wheel_telemetry_write_command([
            false, false, true, false, false, false,
            false, false, false, false, false, false
        ]);
        let (command_id, payload_values_length, _, _) = MozaCommandID::WheelSendTelemetry.into();
        let golden_message_length = MOZA_PROTOCOL_HEADER_LENGTH + command_id.len() + payload_values_length + 1;
        assert_eq!(message.len(), golden_message_length);
    }

    #[test]
    fn test_telemetry_message_header() {
        let message = generate_wheel_telemetry_write_command([
            false, false, true, false, false, false,
            false, false, false, false, false, false
        ]);
        let (command_id, payload_values_length, _, write_group) = MozaCommandID::WheelSendTelemetry.into();

        assert_eq!(message[0], MOZA_START_BYTE);
        assert_eq!(message[1], (payload_values_length + command_id.len()) as u8);
        assert_eq!(message[2], write_group);
        assert_eq!(message[3], MOZA_DEVICE_ID);
    }

    #[test]
    fn test_telemetry_message_payload() {
        let message = generate_wheel_telemetry_write_command([
            false, false, true, false, false, false,
            false, false, false, false, false, false
        ]);
        let (command_id, payload_values_length, _, _) = MozaCommandID::WheelSendTelemetry.into();

        let range_of_interest = MOZA_PROTOCOL_HEADER_LENGTH..(MOZA_PROTOCOL_HEADER_LENGTH + payload_values_length);
        let message_payload = message[range_of_interest].to_vec();

        for (&command_id_i, &message_i) in command_id.iter().zip(message_payload.iter()) {
            assert_eq!(command_id_i, message_i);
        }

        let golden_payload: Vec<u8> = vec![0x00, 0x00, 0x00, 0x04]; // big-endian (reversed bytes)
        for (&payload_i, &message_i) in golden_payload.iter().zip(message_payload[command_id.len()..].iter()) {
            assert_eq!(payload_i, message_i);
        }
    }
}
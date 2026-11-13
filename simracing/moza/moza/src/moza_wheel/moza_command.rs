#[cfg(test)]
mod tests;
#[cfg(test)]
use super::MOZA_DEVICE_ID;

mod command_id;

pub use command_id::MozaCommandID;

const MOZA_MAGIC_NUMBER: u8 = 0x0d;
const MOZA_START_BYTE: u8 = 0x7e;
const MOZA_PROTOCOL_HEADER_LENGTH: usize = 4;

pub struct MozaCommand {
    device_id: u8,
    command_id: Vec<u8>,
    payload_values: Vec<u8>,
    read_group: u8,
    write_group: u8,
}

impl MozaCommand {
    pub fn generate(
        device_id: u8,
        command_id: MozaCommandID,
    ) -> Self {
        let (command_id, payload_len, read_group, write_group) = command_id.into();
        MozaCommand { device_id, command_id, payload_values: vec![0_u8; payload_len], read_group, write_group }
    }

    pub fn generate_with_payload(
        device_id: u8,
        command_id: MozaCommandID,
        payload: &[u8]
    ) -> Self {
        let mut command = MozaCommand::generate(device_id, command_id);
        command.set_payload(payload);
        command
    }

    pub fn set_payload(&mut self, payload: &[u8]) {
        assert_eq!(payload.len(), self.payload_values.len());
        self.payload_values = payload.to_vec();
    }

    pub fn _set_payload_byte(&mut self, index: usize, value: u8) {
        assert!(index < self.payload_values.len());
        self.payload_values[index] = value;
    }

    pub fn set_payload_bit(&mut self, bit_index: usize, set: bool) {
        assert!(bit_index < self.payload_values.len() * 8);
        let byte_index: usize = bit_index / 8;
        let bit_index = bit_index % 8;
        if set {
            set_bit(&mut self.payload_values[byte_index], bit_index);
        } else {
            unset_bit(&mut self.payload_values[byte_index], bit_index);
        }
    }

    pub fn generate_message(&self, read_nwrite: bool) -> Vec<u8> {
        let message_length: usize = self.length();
        let mut message: Vec<u8> = vec![0; message_length];

        message[0..MOZA_PROTOCOL_HEADER_LENGTH].copy_from_slice(&self.generate_header(read_nwrite));
        message[MOZA_PROTOCOL_HEADER_LENGTH..(MOZA_PROTOCOL_HEADER_LENGTH + self.command_id.len())]
            .copy_from_slice(&self.command_id[..]);

        let payload_offset: usize = MOZA_PROTOCOL_HEADER_LENGTH + self.command_id.len();
        let mut payload_values = self.payload_values.clone();
        payload_values.reverse();
        message[payload_offset..(payload_offset + self.payload_values.len())].copy_from_slice(&payload_values[..]);
        message[MOZA_PROTOCOL_HEADER_LENGTH + self.payload_length()] = MozaCommand::calculate_checksum(&message);

        message
    }

    pub fn length(&self) -> usize {
        MOZA_PROTOCOL_HEADER_LENGTH + self.command_id.len() + self.payload_values.len() + 1
    }

    pub fn is_valid_response(&self, response:&[u8]) -> bool {
        let valid_header = response.len() == self.length() &&
            response[0] == MozaCommand::start_byte() &&
            response[1] == (self.payload_length()) as u8 &&
            response[2] == self.read_group | 0x80 &&
            response[3] == nibble_swap(&self.device_id);
        if !valid_header {
            return false;
        }

        response[response.len() - 1] ==
            MozaCommand::calculate_checksum(&response[..response.len() - 1])
    }

    pub fn get_response_payload(&self, response:&[u8]) -> Vec<u8> {
        response[(MOZA_PROTOCOL_HEADER_LENGTH+self.command_id.len())..(response.len() - 1)].to_vec()
    }

    fn payload_length(&self) -> usize {
        self.payload_values.len() + self.command_id.len()
    }

    fn generate_header(&self, read_nwrite: bool) -> [u8; MOZA_PROTOCOL_HEADER_LENGTH] {
        [
            MozaCommand::start_byte(),
            (self.payload_values.len() + self.command_id.len()) as u8,
            if read_nwrite { self.read_group } else { self.write_group },
            self.device_id
        ]
    }
}

impl MozaCommand {
    pub fn start_byte() -> u8 {
        MOZA_START_BYTE
    }

    pub fn calculate_checksum(message: &[u8]) -> u8 {
        let checksum: u32 = message.iter()
            .map(|&d| d as u32)
            .sum::<u32>()
            + (MOZA_MAGIC_NUMBER as u32);
        (checksum % 256) as u8
    }
}

fn set_bit(byte: &mut u8, index: usize) {
    *byte |= 1 << index;
}

fn unset_bit(byte: &mut u8, index: usize) {
    *byte &= !(1 << index);
}

fn _toggle_bit(byte: &mut u8, index: usize) {
    *byte ^= 1 << index;
}

fn nibble_swap(byte: &u8) -> u8 {
    ((byte & 0x0F) << 4) | ((byte >> 4) & 0x0F)
}

use libc::{MAP_SHARED, O_RDONLY, PROT_READ, c_char, c_void, mmap, munmap, shm_open, size_t};
use std::ffi::{CString};

mod ac_static;
mod ac_graphics;
mod ac_physics;
mod pcars2;
mod c_interface;

pub use pcars2::SessionState as PCars2SessionState;
pub use pcars2::ParticipantInfo;

#[derive(Clone, Copy)]
enum SharedMemoryFile {
    AcStatic,
    AcPhysics,
    AcGraphics,
    Pcars2,
}

impl SharedMemoryFile {
    fn get_file_size(file:Self) -> size_t {
        match file {
            Self::AcStatic => 2048,
            Self::AcPhysics => 2048,
            Self::AcGraphics => 2048,
            Self::Pcars2 => 102288,
        }
    }
}

impl From<SharedMemoryFile> for String {
    fn from(value: SharedMemoryFile) -> Self {
        match value {
            SharedMemoryFile::AcStatic => "acpmf_static",
            SharedMemoryFile::AcGraphics => "acpmf_graphics",
            SharedMemoryFile::AcPhysics => "acpmf_physics",
            SharedMemoryFile::Pcars2 => "$pcars2$",
        }.to_string()
    }
}

impl From<SharedMemoryFile> for *const c_char {
    fn from(value: SharedMemoryFile) -> Self {
        let c_string = std::ffi::CString::new(String::from(value)).unwrap();
        c_string.as_ptr()
    }
}

pub struct SharedMemory<T> {
    data: *mut T,
    shared_memory_file_descriptor: i32,
    shared_memory_file_type: SharedMemoryFile,
}

pub struct PCars2Data {
    data: SharedMemory<pcars2::ProjectCars2>
}

impl PCars2Data {
    pub fn new() -> Result<Self, ()> {
        Ok(PCars2Data {
            data: match open_shared_memory::<pcars2::ProjectCars2>(SharedMemoryFile::Pcars2) {
                Ok(data) => data,
                Err(_) => return Err(())
            }
        })
    }

    pub fn version(&self) -> u32 {
        self.data.access().version
    }

    pub fn unfiltered_throttle(&self) -> f32 {
        self.data.access().unfiltered_throttle
    }

    pub fn unfiltered_brake(&self) -> f32 {
        self.data.access().unfiltered_brake
    }

    pub fn unfiltered_steering(&self) -> f32 {
        self.data.access().unfiltered_steering
    }

    pub fn unfiltered_clutch(&self) -> f32 {
        self.data.access().unfiltered_clutch
    }

    pub fn throttle(&self) -> f32 {
        self.data.access().throttle
    }

    pub fn num_participants(&self) -> usize {
        self.data.access().num_participants.max(0) as usize
    }

    pub fn get_participants(&self) -> Vec<pcars2::ParticipantInfo> {
        if self.num_participants() == 0 {
            Vec::new()
        } else {
            self.data.access().participant_info[..self.num_participants()]
                .iter()
                .map(|val| val.into())
                .collect::<Vec<pcars2::ParticipantInfo>>()
        }
    }

    pub fn get_event_time_remaining(&self) -> f32 {
        self.data.access().event_time_remaining
    }

    pub fn get_last_lap(&self, index: usize) -> Option<f32> {
        let time = self.data.access().last_lap_times[index];
        if time < 0.0_f32 {
            None
        } else {
            Some(time)
        }
    }

    pub fn get_best_lap(&self, index: usize) -> Option<f32> {
        let time = self.data.access().fastest_lap_times[index];
        if time < 0.0_f32 {
            None
        } else {
            Some(time)
        }
    }

    pub fn get_speed(&self, index: usize) -> f32 {
        self.data.access().speeds[index]
    }

    pub fn num_sectors(&self) -> i32 {
        self.data.access().num_sectors
    }

    pub fn is_pit_limited(&self) -> bool {
        self.data.access().car_flags & (1 << 3) != 0
    }

    pub fn rpm(&self) -> f32 {
        self.data.access().rpm
    }

    pub fn max_rpm(&self) -> f32 {
        self.data.access().max_rpm
    }

    pub fn sector_1_time(&self) -> f32 {
        self.data.access().current_sector1_time
    }

    pub fn sector_2_time(&self) -> f32 {
        self.data.access().current_sector2_time
    }

    pub fn sector_3_time(&self) -> f32 {
        self.data.access().current_sector3_time
    }

    pub fn current_lap_time(&self) -> f32 {
        self.data.access().current_time
    }

    pub fn current_sector(&self) -> u32 {
        let index = self.data.access().viewed_participant_index as usize;
        self.get_participants()[index].current_sector
    }

    pub fn best_lap_time(&self) -> f32 {
        self.data.access().best_lap_time
    }

    pub fn best_sector1_time(&self) -> f32 {
        self.data.access().fastest_sector1_time
    }

    pub fn best_sector2_time(&self) -> f32 {
        self.data.access().fastest_sector2_time
    }

    pub fn best_sector3_time(&self) -> f32 {
        self.data.access().fastest_sector3_time
    }

    pub fn last_lap_time(&self) -> f32 {
        self.data.access().last_lap_time
    }

    pub fn is_lap_valid(&self) -> bool {
        !(self.data.access().lap_invalidated == 1)
    }

    pub fn world_best_sector1_time(&self) -> f32 {
        self.data.access().world_fastest_sector1_time
    }

    pub fn world_best_sector2_time(&self) -> f32 {
        self.data.access().world_fastest_sector2_time
    }

    pub fn world_best_sector3_time(&self) -> f32 {
        self.data.access().world_fastest_sector3_time
    }

    pub fn world_best_lap_time(&self) -> f32 {
        self.data.access().world_fastest_lap_time
    }

    pub fn personal_best_sector1_time(&self) -> f32 {
        self.data.access().personal_fastest_sector1_time
    }

    pub fn personal_best_sector2_time(&self) -> f32 {
        self.data.access().personal_fastest_sector2_time
    }

    pub fn personal_best_sector3_time(&self) -> f32 {
        self.data.access().personal_fastest_sector3_time
    }

    pub fn personal_best_lap_time(&self) -> f32 {
        self.data.access().personal_fastest_lap_time
    }

    pub fn game_state(&self) -> pcars2::GameState {
        self.data.access().game_state.into()
    }

    pub fn is_in_game(&self) -> bool {
        match self.game_state() {
            pcars2::GameState::InGameIMenuTimeTicking |
            pcars2::GameState::InGamePaused |
            pcars2::GameState::InGamePlaying |
            pcars2::GameState::InGameReplay |
            pcars2::GameState::InGameRestarting => true,
            _ => false
        }
    }

    pub fn session_state(&self) -> pcars2::SessionState {
        self.data.access().session_state.into()
    }

    pub fn session_is_valid(&self) -> bool {
        self.session_state() != pcars2::SessionState::SessionInvalid
    }

    pub fn session_is_test(&self) -> bool {
        self.session_state() == pcars2::SessionState::SessionTest
    }

    pub fn session_is_practice(&self) -> bool {
        self.session_state() != pcars2::SessionState::SessionPractice
    }

    pub fn session_is_qualifying(&self) -> bool {
        self.session_state() != pcars2::SessionState::SessionQualify
    }

    pub fn session_is_race(&self) -> bool {
        self.session_state() != pcars2::SessionState::SessionRace
    }

    pub fn session_is_time_attack(&self) -> bool {
        self.session_state() != pcars2::SessionState::SessionTimeAttack
    }

    pub fn session_is_formation_lap(&self) -> bool {
        self.session_state() != pcars2::SessionState::SessionFormationLap
    }

    pub fn get_active_participant(&self) -> ParticipantInfo {
        let mut active_participant = pcars2::ParticipantInfoSharedMemory::default();
        for participant in &self.data.access().participant_info {
            if participant.is_active != 0 {
                active_participant = participant.clone();
                break;
            }
        }

        (&active_participant).into()
    }

    pub fn track_length(&self) -> f32 {
        self.data.access().track_length
    }

    pub fn sequence_number(&self) -> u32 {
        self.data.access().sequence_number
    }

    pub fn get_viewed_participant_index(&self) -> usize {
        self.data.access().viewed_participant_index as usize
    }
}

pub struct ACData {
    static_data: SharedMemory<ac_static::SPageFileStatic>,
    physics_data: SharedMemory<ac_physics::SPageFilePhysics>,
    graphics_data: SharedMemory<ac_graphics::SPageFileGraphic>,
}

impl<T> SharedMemory<T> {
    pub fn is_valid(&self) -> bool {
        self.shared_memory_file_descriptor != -1
    }

    pub fn access(&self) -> &T {
        unsafe {
            self.data.as_ref().unwrap()
        }
    }
}

impl<T> Drop for SharedMemory<T> {
    fn drop(&mut self) {
        unsafe {
            munmap(self.data as *mut c_void, SharedMemoryFile::get_file_size(self.shared_memory_file_type));
        }
    }
}

impl<T> From<SharedMemoryFile> for SharedMemory<T> {
    fn from(value: SharedMemoryFile) -> Self {
        let (shared_memory_file_descriptor, data) = unsafe {
            let fd = shm_open(
                value.into(),
                O_RDONLY,
                0o666
            );

            let address = match fd {
                -1 => std::ptr::null_mut(),
                _ => mmap(
                        std::ptr::null_mut(),
                        SharedMemoryFile::get_file_size(value),
                        PROT_READ,
                        MAP_SHARED,
                        fd,
                        0
                )
            };

            (fd, address as *mut T)
        };

        SharedMemory {data, shared_memory_file_descriptor, shared_memory_file_type: value}
    }
}

fn open_shared_memory<T>(memory_type: SharedMemoryFile) -> Result<SharedMemory<T>, u8> {
    let (shared_memory_file_descriptor, data) = unsafe {
        let fd = shm_open(
            CString::new(String::from(memory_type)).unwrap().as_ptr(),
            O_RDONLY,
            0o666
        );

        if fd == -1 {
            return Err(1);
        }

        let address = mmap(
            std::ptr::null_mut(),
            SharedMemoryFile::get_file_size(memory_type),
            PROT_READ,
            MAP_SHARED,
            fd,
            0
        );

        (fd, address as *mut T)
    };

    Ok(SharedMemory {data, shared_memory_file_descriptor, shared_memory_file_type: memory_type})
}

impl ACData {
    /// Attempt to connect with shared data from an AC-compatible shared memory game.
    ///
    /// # Returns
    /// [Result]:
    /// - [Ok]: If all three shared memory files exist and can be opened, return a newly constructed [ACData] instance.
    /// - [Err]: Tuple of flags corresponding to the shared memory files.
    ///   Flag is set to true if shared memory failed to load.
    pub fn new() -> Result<Self, (bool, bool, bool)> {
        Ok(ACData {
            static_data: match open_shared_memory::<ac_static::SPageFileStatic>(SharedMemoryFile::AcStatic) {
                Ok(data) => data,
                Err(_) => return Err((true, true, true))
            },
            physics_data: match open_shared_memory::<ac_physics::SPageFilePhysics>(SharedMemoryFile::AcPhysics) {
                Ok(data) => data,
                Err(_) => return Err((false, true, true))
            },
            graphics_data: match open_shared_memory::<ac_graphics::SPageFileGraphic>(SharedMemoryFile::AcGraphics) {
                Ok(data) => data,
                Err(_) => return Err((false, false, true))
            }
        })
    }

    pub fn get_rpms(&self) -> i32 {
        self.physics_data.access().rpm
    }

    pub fn get_max_rpm(&self) -> i32 {
        self.physics_data.access().current_max_rpm
    }

    pub fn is_pit_limited(&self) -> bool {
        self.physics_data.access().pit_limiter_on == 1
    }

    pub fn is_valid(&self) -> bool {
        self.static_data.is_valid() && self.physics_data.is_valid()
            && self.graphics_data.is_valid()
    }

    pub fn get_status(&self) -> ac_graphics::Status {
        self.graphics_data.access().status.into()
    }

    pub fn get_session_type(&self) -> ac_graphics::SessionType {
        self.graphics_data.access().session.into()
    }

    pub fn get_flag(&self) -> ac_graphics::Flag {
        self.graphics_data.access().flag.into()
    }

    pub fn abs(&self) -> i32 {
        self.graphics_data.access().abs
    }

    pub fn is_yellow(&self) -> Vec<bool> {
        return vec![
            self.graphics_data.access().global_yellow == 1,
            self.graphics_data.access().global_yellow1 == 1,
            self.graphics_data.access().global_yellow2 == 1,
            self.graphics_data.access().global_yellow3 == 1
        ]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

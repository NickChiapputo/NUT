use shared_memory::{self, PCars2Data, PCars2SessionState};

use iced::widget::{Text, column, container, row, text};
use iced::{application, time, window, Color, Element, Subscription, Theme, Length};

use std::sync::{Arc, Mutex};

mod relative_gap_calculator;

pub fn run(
    data: PCars2Data,
    wheel: moza::MozaWheel
) -> iced::Result {
    let overlay = Overlay::new(
        Arc::new(Mutex::new(data)),
        Arc::new(Mutex::new(wheel))
    );

    application(
        "AMS2 RPM Overlay",
        Overlay::update,
        Overlay::view,
    )
    .subscription(Overlay::subscription)
    .style(Overlay::style)
    .window(window::Settings {
        size: iced::Size::new(400.0, 1000.0),
        decorations: false,
        transparent: true,
        resizable: false,
        level: window::Level::AlwaysOnTop,
        ..Default::default()
    })
    .run_with(|| (overlay, iced::Task::none()))
}

pub type OverlayResult = iced::Result;

pub struct Overlay {
    data: Arc<Mutex<PCars2Data>>,
    wheel: Arc<Mutex<moza::MozaWheel>>,
    rpm: f32,
    max_rpm: f32,
    participants: Vec<DisplayedParticipantInfo>,
    event_time_remaining: f32,
    last_state: PCars2SessionState,
    fastest_session_lap: f32,
    lap_data: Vec<Lap>
}

#[derive(Clone, Copy, Default)]
struct Lap {
    lap_number: u32,
    time: f32,
    sector1_time: f32,
    sector2_time: f32,
    sector3_time: f32,
    valid: bool,
    world_best_lap: bool,
    world_best_sector1: bool,
    world_best_sector2: bool,
    world_best_sector3: bool,
    personal_best_lap: bool,
    personal_best_sector1: bool,
    personal_best_sector2: bool,
    personal_best_sector3: bool,
    session_best_lap: bool,
    session_best_sector1: bool,
    session_best_sector2: bool,
    session_best_sector3: bool,
}

#[derive(Clone)]
struct DisplayedParticipantInfo {
    name: String,
    index: usize,
    overall_position: u32,
    lap: u32,
    lap_distance: f32,
    speed: f32,
    last_lap_opt: Option<f32>,
    best_lap_opt: Option<f32>,
    relative_time: f32,
    relative_laps: u32,
    gap_time: f32,
    gap_laps: u32,
    best_lap_gap: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SelfUpdates,
    OtherDriverUpdates
}

const USER_COLOR: Color = Color {
    r: 0.505,
    g: 0.631,
    b: 0.757,
    a: 1.0,
};

const INVALID_LAP_COLOR: Color = Color { // #bf616a
    r: 0.749,
    g: 0.380,
    b: 0.416,
    a: 1.0,
};

const FASTEST_LAP_COLOR: Color = Color {  // #b48ead
    r: 0.706,
    g: 0.557,
    b: 0.678,
    a: 1.0,
};

const PERSONAL_BEST_LAP_COLOR: Color = Color { // #a3be8c
    r: 0.639,
    g: 0.745,
    b: 0.549,
    a: 1.0,
};

const SESSION_BEST_LAP_COLOR: Color = Color { // #8fbcbb
    r: 0.561,
    g: 0.737,
    b: 0.733,
    a: 1.0,
};

const FOREGROUND_COLOR: Color = Color::WHITE;

impl Overlay {
    pub fn new(
        data: Arc<Mutex<PCars2Data>>,
        wheel: Arc<Mutex<moza::MozaWheel>>
    ) -> Self {
        Self {
            data,
            wheel,
            rpm: 0.0,
            max_rpm: 1.0,
            participants: Vec::new(),
            event_time_remaining: 0.0,
            last_state: PCars2SessionState::SessionInvalid,
            fastest_session_lap: -1.0,
            lap_data: Vec::new(),
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::SelfUpdates => {
                let data = self.data.lock().expect("telemetry mutex poisoned");
                let wheel = self.wheel.lock().expect("wheel mutex poisoned");

                if self.last_state != data.session_state() {
                    self.fastest_session_lap = -1.0;
                    self.last_state = data.session_state();
                    self.lap_data.clear();
                }

                self.rpm = data.rpm();
                self.max_rpm = data.max_rpm().max(1.0);
                let rpm = match data.is_pit_limited() {
                    true => data.max_rpm(),
                    false => data.rpm()
                };
                wheel.set_rpm_leds_from_rpm(rpm as i32, data.max_rpm() as i32);

                let active_participant = data.get_active_participant();
                let lap_num = active_participant.current_lap;
                let current_sector = active_participant.current_sector;
                let sector_1_time = data.sector_1_time();
                let sector_2_time = if current_sector > 0 { data.sector_2_time() } else { 0.0 };
                let sector_3_time = if current_sector > 1 { data.sector_3_time() } else { 0.0 };
                let current_lap_time = data.current_lap_time();
                let lap_valid = data.is_lap_valid();

                let num_laps_in_list = self.lap_data.len();

                let updating_existing_lap = self.lap_data.len() > 0 &&
                    self.lap_data[num_laps_in_list - 1].time <= current_lap_time;
                if updating_existing_lap {
                    self.lap_data[num_laps_in_list - 1].time = current_lap_time;
                    self.lap_data[num_laps_in_list - 1].sector1_time = sector_1_time;
                    self.lap_data[num_laps_in_list - 1].sector2_time = sector_2_time;
                    self.lap_data[num_laps_in_list - 1].sector3_time = sector_3_time;
                    self.lap_data[num_laps_in_list - 1].valid = lap_valid;
                } else {
                    if num_laps_in_list > 0 {
                        // need to update the last sector 3 time recording since the last tick doesn't get it before
                        // we get to the next lap. The previous lap sector 3 time will stay populated until sector 3
                        // is started again.
                        self.lap_data[num_laps_in_list - 1].sector3_time = data.sector_3_time();
                        self.lap_data[num_laps_in_list - 1].time = data.last_lap_time();

                        // Update flags for all previous laps.
                        for lap in &mut self.lap_data {
                            set_best_flags(lap,
                                data.world_best_lap_time(),
                                data.world_best_sector1_time(),
                                data.world_best_sector2_time(),
                                data.world_best_sector3_time(),
                                data.personal_best_lap_time(),
                                data.personal_best_sector1_time(),
                                data.personal_best_sector2_time(),
                                data.personal_best_sector3_time(),
                                data.best_lap_time(),
                                data.best_sector1_time(),
                                data.best_sector2_time(),
                                data.best_sector3_time()
                            );
                        }
                    }

                    self.lap_data.push(Lap {
                        lap_number: lap_num,
                        time: current_lap_time,
                        sector1_time: sector_1_time,
                        sector2_time: sector_2_time,
                        sector3_time: sector_3_time,
                        valid: lap_valid,
                        ..Default::default()
                    });
                }

                iced::Task::none()
            }
            Message::OtherDriverUpdates => {
                let data = self.data.lock().expect("telemetry mutex poisoned");
                if self.last_state != data.session_state() {
                    self.fastest_session_lap = -1.0;
                    self.last_state = data.session_state();
                    self.lap_data.clear();
                }
                self.event_time_remaining = data.get_event_time_remaining();
                self.participants = data.get_participants()
                    .iter()
                    .enumerate()
                    .filter_map(|(index, info)|
                        if info.name == "Safety Car" {
                            None
                        } else {
                            Some(DisplayedParticipantInfo {
                                name: info.name.clone(),
                                index: index,
                                overall_position: info.race_position,
                                lap: info.current_lap,
                                lap_distance: info.current_lap_distance,
                                speed: data.get_speed(index),
                                last_lap_opt: data.get_last_lap(index),
                                best_lap_opt: data.get_best_lap(index),
                                relative_time: 0.0, // and below to be calculated next
                                relative_laps: 0,
                                gap_time: 0.0,
                                gap_laps: 0,
                                best_lap_gap: 0.0
                            })
                        }
                    )
                    .collect();

                self.participants.sort_by(|a, b|
                    a.overall_position.cmp(&b.overall_position));

                if self.participants.len() > 0 {
                    check_best_lap(&mut self.fastest_session_lap, &self.participants[0]);
                }

                for i in 1..self.participants.len() {
                    self.participants[i].relative_time = match relative_gap_seconds(
                        &self.participants[i - 1],
                        &self.participants[i],
                        data.track_length()
                    ) {
                        Some(rel) => rel,
                        None => 0.0
                    };
                    self.participants[i].relative_laps = if self.participants[i - 1].lap > self.participants[i].lap {
                        self.participants[i - 1].lap - self.participants[i].lap
                    } else {
                        0
                    };

                    self.participants[i].gap_time = match relative_gap_seconds(
                        &self.participants[0],
                        &self.participants[i],
                        data.track_length()
                    ) {
                        Some(rel) => rel,
                        None => 0.0
                    };
                    self.participants[i].gap_laps = if self.participants[0].lap > self.participants[i].lap {
                        self.participants[0].lap - self.participants[i].lap
                    } else {
                        0
                    };

                    if self.participants[i].best_lap_opt.is_some() {
                        let this_best_lap = self.participants[i].best_lap_opt.unwrap();
                        self.participants[i].best_lap_gap =
                            if self.participants[0].best_lap_opt.is_some() {
                                self.participants[0].best_lap_opt.unwrap() - this_best_lap
                            } else {
                                0.0
                            };
                        check_best_lap(&mut self.fastest_session_lap, &self.participants[i]);
                    }
                }

                iced::Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let data = self.data.lock().expect("telemetry mutex poisoned");

        match data.session_state() {
            shared_memory::PCars2SessionState::SessionInvalid => {
                self.build_empty_view(data)
            },
            shared_memory::PCars2SessionState::SessionPractice |
            shared_memory::PCars2SessionState::SessionQualify => {
                self.build_qualify_view(data)
            },
            shared_memory::PCars2SessionState::SessionFormationLap |
            shared_memory::PCars2SessionState::SessionRace => {
                self.build_race_view(data)
            },
            shared_memory::PCars2SessionState::SessionTest |
            shared_memory::PCars2SessionState::SessionTimeAttack => {
                self.build_time_attack_view(data)
            },
        }
    }

    fn build_empty_view(&self, _data:std::sync::MutexGuard<'_, PCars2Data>) -> Element<'_, Message> {
        container(column![]).into()
    }

    fn build_qualify_view(&self, data:std::sync::MutexGuard<'_, PCars2Data>) -> Element<'_, Message> {
        let lap_rows = self.participants
            .iter()
            .enumerate()
            .map(|(position, info)| {
                let (color, best_lap_color, last_lap_color) = get_text_colors(
                    data.get_viewed_participant_index() == info.index,
                    info,
                    self.fastest_session_lap
                );

                row![
                    generate_text_block(position+1, 1, color, 12),
                    generate_text_block(info.name.clone(), 6, color, 12),
                    generate_text_block(generate_gap_string(0, info.best_lap_gap), 2, color, 12),
                    generate_text_block(format_duration_from_f32(info.last_lap_opt, 3), 2, last_lap_color, 12),
                    generate_text_block(format_duration_from_f32(info.best_lap_opt, 3), 2, best_lap_color, 12),
                ].spacing(4).into()
            });

        container(
            column![
                text(format_duration_from_f32(Some(self.event_time_remaining), 0)).size(16),
                column![
                    row![
                        generate_text_block("P",    1, FOREGROUND_COLOR, 16),
                        generate_text_block("Name", 7, FOREGROUND_COLOR, 16),
                        generate_text_block("Gap",  2, FOREGROUND_COLOR, 16),
                        generate_text_block("Last", 2, FOREGROUND_COLOR, 16),
                        generate_text_block("Best", 2, FOREGROUND_COLOR, 16),
                    ].spacing(4)
                ],
                column(lap_rows).spacing(4)
            ]
            .spacing(8),
        )
        .padding(12)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(Color::from_rgba(0.02, 0.02, 0.02, 0.72).into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            }
        })
        .into()
    }

    fn build_time_attack_view(&self, data:std::sync::MutexGuard<'_, PCars2Data>) -> Element<'_, Message> {
        let lap_rows = self.lap_data
            .iter()
            .map(|info| {
                let lap_time_color = get_time_attack_time_color(info.valid, info.world_best_lap,
                    info.personal_best_lap, info.session_best_lap);
                let sector1_color = get_time_attack_time_color(info.valid, info.world_best_sector1,
                    info.personal_best_sector1, info.session_best_sector1);
                let sector2_color = get_time_attack_time_color(info.valid, info.world_best_sector2,
                    info.personal_best_sector2, info.session_best_sector2);
                let sector3_color = get_time_attack_time_color(info.valid, info.world_best_sector3,
                    info.personal_best_sector3, info.session_best_sector3);
                row![
                    generate_text_block(info.lap_number, 1, FOREGROUND_COLOR, 12),
                    generate_text_block(format_duration_from_f32(Some(info.time), 3), 2, lap_time_color, 12),
                    generate_text_block(format_duration_from_f32(Some(info.sector1_time), 3), 2, sector1_color, 12),
                    generate_text_block(format_duration_from_f32(Some(info.sector2_time), 3), 2, sector2_color, 12),
                    generate_text_block(format_duration_from_f32(Some(info.sector3_time), 3), 2, sector3_color, 12),
                ].spacing(4).into()
            });

        container(
            column![
                text(format_duration_from_f32(Some(self.event_time_remaining), 0)).size(16),
                column![
                    row![
                        generate_text_block("L",    1, FOREGROUND_COLOR, 16),
                        generate_text_block("Time", 2, FOREGROUND_COLOR, 16),
                        generate_text_block("S1",   2, FOREGROUND_COLOR, 16),
                        generate_text_block("S2",   2, FOREGROUND_COLOR, 16),
                        generate_text_block("S3",   2, FOREGROUND_COLOR, 16),
                    ].spacing(4)
                ],
                column![
                    row![
                        generate_text_block("World", 1, FOREGROUND_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.world_best_lap_time()), 3), 2, FASTEST_LAP_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.world_best_sector1_time()), 3), 2, FASTEST_LAP_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.world_best_sector2_time()), 3), 2, FASTEST_LAP_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.world_best_sector3_time()), 3), 2, FASTEST_LAP_COLOR, 12),
                    ]
                ],
                column![
                    row![
                        generate_text_block("Personal", 1, FOREGROUND_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.personal_best_lap_time()), 3), 2, PERSONAL_BEST_LAP_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.personal_best_sector1_time()), 3), 2, PERSONAL_BEST_LAP_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.personal_best_sector2_time()), 3), 2, PERSONAL_BEST_LAP_COLOR, 12),
                        generate_text_block(format_duration_from_f32(Some(data.personal_best_sector3_time()), 3), 2, PERSONAL_BEST_LAP_COLOR, 12),
                    ]
                ],
                column(lap_rows).spacing(4)
            ]
            .spacing(8),
        )
        .padding(12)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(Color::from_rgba(0.02, 0.02, 0.02, 0.72).into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            }
        })
        .into()
    }

    fn build_race_view(&self, data:std::sync::MutexGuard<'_, PCars2Data>) -> Element<'_, Message> {
        let lap_rows = self.participants
            .iter()
            .enumerate()
            .map(|(position, info)| {
                let (color, best_lap_color, last_lap_color) = get_text_colors(
                    data.get_viewed_participant_index() == info.index,
                    info,
                    self.fastest_session_lap
                );

                row![
                    generate_text_block(position+1, 1, color, 12),
                    generate_text_block(info.name.clone(), 6, color, 12),
                    generate_text_block(generate_gap_string(info.relative_laps, info.relative_time), 2, color, 12),
                    generate_text_block(generate_gap_string(info.gap_laps, info.gap_time), 2, color, 12),
                    generate_text_block(format_duration_from_f32(info.last_lap_opt, 3), 2, last_lap_color, 12),
                    generate_text_block(format_duration_from_f32(info.best_lap_opt, 3), 2, best_lap_color, 12),
                ].spacing(4).into()
            });

        container(
            column![
                text(format_duration_from_f32(Some(self.event_time_remaining), 0)).size(16),
                column![
                    row![
                        generate_text_block("P",        1, FOREGROUND_COLOR, 16),
                        generate_text_block("Name",     7, FOREGROUND_COLOR, 16),
                        generate_text_block("Interval", 2, FOREGROUND_COLOR, 16),
                        generate_text_block("Gap",      2, FOREGROUND_COLOR, 16),
                        generate_text_block("Last",     2, FOREGROUND_COLOR, 16),
                        generate_text_block("Best",     2, FOREGROUND_COLOR, 16),
                    ].spacing(4)
                ],
                column(lap_rows).spacing(4)
            ]
            .spacing(8),
        )
        .padding(12)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(Color::from_rgba(0.02, 0.02, 0.02, 0.72).into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            }
        })
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            time::every(std::time::Duration::from_millis(500)).map(|_| Message::OtherDriverUpdates),
            time::every(std::time::Duration::from_millis(50)).map(|_| Message::SelfUpdates)
        ])
    }

    fn style(&self, theme: &Theme) -> iced::daemon::Appearance {
        iced::daemon::Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}

fn check_best_lap(best_lap: &mut f32, participant: &DisplayedParticipantInfo) {
    if participant.best_lap_opt.is_some() {
        let this_best_lap = participant.best_lap_opt.unwrap();
        let faster = *best_lap < 0.0 ||
            this_best_lap < *best_lap;
        if faster {
            *best_lap = this_best_lap;
        }
    }
}

fn generate_gap_string(
    lap_difference: u32,
    time_difference: f32
) -> String {
    if lap_difference > 0 {
        String::from(format!("{} lap{}", lap_difference, if lap_difference > 1 {"s"} else {""}))
    } else {
        format_duration_from_f32(if time_difference == 0.0 { None } else { Some(time_difference) }, 3)
    }
}

fn generate_text_block<'a>(
    position:impl text::IntoFragment<'a>,
    width:u16,
    color:Color,
    text_size:u16
) -> Text<'a> {
    text(position)
        .width(Length::FillPortion(width))
        .color(color)
        .size(text_size)
}

fn format_duration_from_f32(
    seconds: Option<f32>,
    ms_digits: usize
) -> String {
    if seconds.is_none() {
        return format!("---")
    } else {
        let is_negative = seconds.unwrap() < 0.0;

        let mut d = std::time::Duration::from_secs_f32(f32::abs(seconds.unwrap()));
        let hours = d.as_secs() / 3600;

        d = d - std::time::Duration::from_secs_f32((hours * 3600) as f32);
        let minutes = d.as_secs() / 60;
        let seconds = d.as_secs() % 60;
        let milliseconds = d.subsec_millis();

        let sign_string = if is_negative { format!("-") } else { format!("") };

        let hours_string: String = if hours > 0 {
            String::from(format!("{sign_string}{hours}:"))
        } else {
            String::from(format!("{sign_string}"))
        };

        let minutes_string: String = if minutes > 0 || hours > 0 {
            if hours > 0 {
                String::from(format!("{hours_string}{minutes:02}:"))
            } else {
                String::from(format!("{hours_string}{minutes}:"))
            }
        } else {
            String::from(format!(""))
        };

        let seconds_string: String = if minutes > 0 {
            String::from(format!("{minutes_string}{seconds:02}"))
        } else {
            String::from(format!("{seconds}"))
        };

        let ms_string: String = if ms_digits > 0 {
            String::from(format!("{seconds_string}.{milliseconds:0width$}", width=ms_digits))
        } else {
            seconds_string
        };
        return ms_string;
    }
}

fn relative_gap_seconds(
    participant_a: &DisplayedParticipantInfo,
    participant_b: &DisplayedParticipantInfo,
    track_length: f32
) -> Option<f32> {
    relative_gap_calculator::calculate(
        participant_a.lap,
        participant_a.lap_distance,
        participant_a.speed,
        participant_b.lap,
        participant_b.lap_distance,
        participant_b.speed,
        track_length
    )
}

fn get_time_attack_time_color(valid: bool, world_best: bool, personal_best: bool, session_best: bool) -> Color {
    if !valid {
        INVALID_LAP_COLOR
    } else if world_best {
        FASTEST_LAP_COLOR
    } else if personal_best {
        PERSONAL_BEST_LAP_COLOR
    } else if session_best {
        SESSION_BEST_LAP_COLOR
    } else {
        FOREGROUND_COLOR
    }
}

fn set_best_flags(
    lap: &mut Lap,
    world_best_lap: f32,
    world_best_sector1: f32,
    world_best_sector2: f32,
    world_best_sector3: f32,
    personal_best_lap: f32,
    personal_best_sector1: f32,
    personal_best_sector2: f32,
    personal_best_sector3: f32,
    session_best_lap: f32,
    session_best_sector1: f32,
    session_best_sector2: f32,
    session_best_sector3: f32,
) {
    // Set session best flags
    (*lap).session_best_lap = lap.valid && lap.time == session_best_lap;
    (*lap).session_best_sector1 = lap.valid && lap.sector1_time == session_best_sector1;
    (*lap).session_best_sector2 = lap.valid && lap.sector2_time == session_best_sector2;
    (*lap).session_best_sector3 = lap.valid && lap.sector3_time == session_best_sector3;

    // Set world best flags
    (*lap).world_best_lap = lap.valid && lap.time == world_best_lap;
    (*lap).world_best_sector1 = lap.valid && lap.sector1_time == world_best_sector1;
    (*lap).world_best_sector2 = lap.valid && lap.sector2_time == world_best_sector2;
    (*lap).world_best_sector3 = lap.valid && lap.sector3_time == world_best_sector3;

    // Set personal best flags
    (*lap).personal_best_lap = lap.valid && lap.time == personal_best_lap;
    (*lap).personal_best_sector1 = lap.valid && lap.sector1_time == personal_best_sector1;
    (*lap).personal_best_sector2 = lap.valid && lap.sector2_time == personal_best_sector2;
    (*lap).personal_best_sector3 = lap.valid && lap.sector3_time == personal_best_sector3;
}

/// Calculate foreground text colors from best and last laps compared to fastest in the session.
///
/// # Arguments
/// - `viewed_participant_index`    Currently viewed participant
/// - `info`                        Participant info containing best and last lap [Option]s.
/// - `fastest_session_lap`         Fastest overall lap in the session (in seconds).
///
/// # Returns
/// Tuple containing the main, best lap, and last lap text colors.
fn get_text_colors(
    is_viewed_participant: bool,
    info: &DisplayedParticipantInfo,
    fastest_session_lap: f32
) -> (Color, Color, Color) {
    let best_lap_exists = info.best_lap_opt.is_some();
    let last_lap_exists = info.last_lap_opt.is_some();
    let best_lap_is_fastest_lap = if best_lap_exists { fastest_session_lap == info.best_lap_opt.unwrap() }
        else { false };
    let last_lap_is_fastest_lap = if last_lap_exists { fastest_session_lap == info.last_lap_opt.unwrap() }
        else { false };

    let main_color = if is_viewed_participant { USER_COLOR } else { FOREGROUND_COLOR };

    let best_lap_color =
        if best_lap_exists && best_lap_is_fastest_lap { FASTEST_LAP_COLOR }
        else if is_viewed_participant { USER_COLOR }
        else { FOREGROUND_COLOR };

    let last_lap_color =
        if best_lap_exists && last_lap_exists && last_lap_is_fastest_lap { FASTEST_LAP_COLOR }
        else if is_viewed_participant { USER_COLOR }
        else { FOREGROUND_COLOR };

    (main_color, best_lap_color, last_lap_color)
}

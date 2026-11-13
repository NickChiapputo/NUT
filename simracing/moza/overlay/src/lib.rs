use shared_memory::{self, PCars2Data};

use iced::widget::{column, row, container, progress_bar, text};
use iced::{application, time, window, Color, Element, Subscription, Theme, Length};

use std::sync::{Arc, Mutex};

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
}

#[derive(Clone)]
struct DisplayedParticipantInfo {
    name: String,
    // index: usize,
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
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SelfUpdates,
    OtherDriverUpdates
}

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
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::SelfUpdates => {
                let data = self.data.lock().expect("telemetry mutex poisoned");
                let wheel = self.wheel.lock().expect("wheel mutex poisoned");

                self.rpm = data.rpm();
                self.max_rpm = data.max_rpm().max(1.0);
                let rpm = match data.is_pit_limited() {
                    true => data.max_rpm(),
                    false => data.rpm()
                };
                wheel.set_rpm_leds_from_rpm(rpm as i32, data.max_rpm() as i32);

                iced::Task::none()
            }
            Message::OtherDriverUpdates => {
                let data = self.data.lock().expect("telemetry mutex poisoned");

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
                                // index: index,
                                overall_position: info.race_position,
                                lap: info.current_lap,
                                lap_distance: info.current_lap_distance,
                                speed: data.get_speed(index),
                                last_lap_opt: data.get_last_lap(index),
                                best_lap_opt: data.get_best_lap(index),
                                relative_time: 0.0, // to be calculated next
                                relative_laps: 0,
                                gap_time: 0.0, // to be calculated next
                                gap_laps: 0
                            })
                        }
                    )
                    .collect();

                self.participants.sort_by(|a, b|
                    a.overall_position.cmp(&b.overall_position));

                for i in 1..self.participants.len() {
                    self.participants[i].relative_time = match relative_gap_seconds(
                        self.participants[i - 1].lap,
                        self.participants[i - 1].lap_distance,
                        self.participants[i - 1].speed,
                        self.participants[i].lap,
                        self.participants[i].lap_distance,
                        self.participants[i].speed,
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
                        self.participants[0].lap,
                        self.participants[0].lap_distance,
                        self.participants[0].speed,
                        self.participants[i].lap,
                        self.participants[i].lap_distance,
                        self.participants[i].speed,
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
                }

                iced::Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let fraction = (self.rpm / self.max_rpm).clamp(0.0, 1.0);
        let lap_rows = self.participants
            .iter()
            .enumerate()
            .map(|(position, info)| {
                row![
                    text(position+1)
                        .width(Length::FillPortion(1))
                        .color(Color::WHITE)
                        .size(12),
                    text(info.name.clone())
                        .width(Length::FillPortion(6))
                        .color(Color::WHITE)
                        .size(12),
                    text(if info.relative_laps > 0 {
                            String::from(format!("{} lap{}", info.relative_laps, if info.relative_laps > 1 {"s"} else {""}))
                        } else {
                            format_duration_from_f32(
                                if info.relative_time == 0.0 { None }
                                else { Some(info.relative_time) }, 3)
                        })
                        .width(Length::FillPortion(2))
                        .color(Color::WHITE)
                        .size(12),
                    text(if info.gap_laps > 0 {
                            String::from(format!("{} lap{}", info.gap_laps, if info.gap_laps > 1 {"s"} else {""}))
                        } else {
                            format_duration_from_f32(
                                if info.gap_time == 0.0 { None }
                                else { Some(info.gap_time) }, 3)
                        })
                        .width(Length::FillPortion(2))
                        .color(Color::WHITE)
                        .size(12),
                    text(format_duration_from_f32(info.last_lap_opt, 3))
                        .width(Length::FillPortion(2))
                        .color(Color::WHITE)
                        .size(12),
                    text(format_duration_from_f32(info.best_lap_opt, 3))
                        .width(Length::FillPortion(2))
                        .color(Color::WHITE)
                        .size(12),
                ].spacing(4).into()
            });

        container(
            column![
                column![
                    text(format_duration_from_f32(Some(self.event_time_remaining), 0)),
                    text("RPM").size(16),
                    progress_bar(0.0..=1.0, fraction),
                    text(format!("{:.0} / {:.0} RPM", self.rpm, self.max_rpm)).size(22),
                ],
                column![
                    row![
                        text("P")
                            .width(Length::FillPortion(1))
                            .color(Color::WHITE)
                            .size(16),
                        text("Name")
                            .width(Length::FillPortion(7))
                            .color(Color::WHITE)
                            .size(16),
                        text("Interval")
                            .width(Length::FillPortion(2))
                            .color(Color::WHITE)
                            .size(16),
                        text("Gap")
                            .width(Length::FillPortion(2))
                            .color(Color::WHITE)
                            .size(16),
                        text("Last")
                            .width(Length::FillPortion(2))
                            .color(Color::WHITE)
                            .size(16),
                        text("Best")
                            .width(Length::FillPortion(2))
                            .color(Color::WHITE)
                            .size(16),
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
fn relative_gap_seconds(
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

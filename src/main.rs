use std::num::ParseIntError;
use iced::{
    Element,
    Sandbox,
    Settings,
    Alignment,
    widget::{row, slider, TextInput},
};

pub fn main() -> iced::Result {
    Aquarium::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    BaseChanged(Result<u32, ParseIntError>),
    PowerChanged(Result<u32, ParseIntError>),
    TickSliderChanged(u32),
}

#[derive(Debug, Clone, Copy)]

struct Aquarium {
    base: u32,
    power: u32,
    tick_rate: u32,
}

impl Sandbox for Aquarium {
    type Message = Message;

    fn new() -> Self {
        Self {
            base: 2,
            power: 1,
            tick_rate: 1,
        }
    }

    fn title(&self) -> String {
        String::from("Exponent Aquarium")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::BaseChanged(base) => {
                match base {
                    Ok(val) => {
                        self.base = val;
                    },
                    Err(e) => {
                        self.base = 2;
                    }
                }
            } 
            Message::PowerChanged(power) => {
                match power {
                    Ok(val) => {
                        self.base = val;
                    },
                    Err(e) => {
                        self.base = 2;
                    }
                }
            }
            Message::TickSliderChanged(tick_rate) => {
                self.tick_rate = tick_rate;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let base_str = self.base.to_string().as_str();
        let base_input = TextInput::new(
            "Default base: 2",
            base_str,
            Message::BaseChanged(base_str.parse())
        );
        row![
            slider(1..=16, self.tick_rate as u32, Message::TickSliderChanged)
        ]
        .padding(10)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}
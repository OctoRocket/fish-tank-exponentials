use iced::{
    Element,
    Sandbox,
    Settings,
    Alignment,
    Theme,
    widget::{row, slider, text_input},
};

pub fn main() -> iced::Result {
    Aquarium::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    BaseChanged(String),
    PowerChanged(String),
    TickSliderChanged(u32),
}


struct Aquarium {
    base: String,
    power: String,
    tick_rate: u32,
}

impl Sandbox for Aquarium {
    type Message = Message;

    fn new() -> Self {
        Self {
            base: String::from("2"),
            power: String::from("1"),
            tick_rate: 1,
        }
    }

    fn title(&self) -> String {
        String::from("Exponent Aquarium")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::BaseChanged(base) => {
                self.base = base;
            } 
            Message::PowerChanged(power) => {
                self.power = power
            }
            Message::TickSliderChanged(tick_rate) => {
                self.tick_rate = tick_rate;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let base_input = text_input(
            "Default base: 2",
            &self.base,
            Message::BaseChanged,
        )
        .padding(10)
        .size(20);
        let power_input = text_input(
            "Default power: 1",
            &self.power,
            Message::PowerChanged
        )
        .padding(10)
        .size(20);
        row![
            base_input,
            power_input,
            slider(1..=16, self.tick_rate as u32, Message::TickSliderChanged)
        ]
        .padding(10)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
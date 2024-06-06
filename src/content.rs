use crate::fl;
use cosmic::widget;
use cosmic::{theme, Element};

pub struct Content {
    input: String,
    options: Radios,
}

#[derive(Debug, Clone)]
pub enum Message {
    Input(String),
    Radio(Radios),
    Submit,
}

pub enum Command {
    Save(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Radios {
    Option1,
    Option2,
    Option3,
}

impl Content {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            options: Radios::Option1,
        }
    }

    fn title(&self) -> Element<Message> {
        widget::text::title1(fl!("welcome")).into()
    }

    fn button(&self) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        widget::button::button(widget::text::text(fl!("save")))
            .on_press(Message::Submit)
            .padding([spacing.space_xxs, spacing.space_m])
            .into()
    }

    fn radios(&self) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        let selected = Some(self.options);
        let r1 = widget::radio::Radio::new("1", Radios::Option1, selected, Message::Radio).size(12);
        let r2 = widget::radio::Radio::new("2", Radios::Option2, selected, Message::Radio).size(12);
        let r3 = widget::radio::Radio::new("3", Radios::Option3, selected, Message::Radio).size(12);
        widget::column().spacing(spacing.space_xxs).push(r1).push(r2).push(r3).into()
    }

    pub fn view(&self) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        widget::column()
            .spacing(spacing.space_xs)
            .push(self.title())
            .push(widget::warning::warning("Warning: I don't know what I'm doing!"))
            .push(
                widget::text_input::text_input("Enter your name", &self.input)
                    .on_input(Message::Input)
                    .label("Name")
                    .width(150)
            )
            .push(self.radios())
            .push(self.button())
            .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Command> {
        match message {
            Message::Input(input) => {
                self.input = input;
                None
            }
            Message::Submit => Some(Command::Save(self.input.clone())),
            Message::Radio(radio) => {
                self.options = radio;
                println!("Radio: {:?}", radio);
                None
            }
        }
    }
    
}

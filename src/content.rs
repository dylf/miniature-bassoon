use crate::fl;
use cosmic::widget;
use cosmic::{theme, Element};

pub struct Content {
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Input(String),
    Submit,
}

pub enum Command {
    Save(String),
}

impl Content {
    pub fn new() -> Self {
        Self {
            input: String::new()
        }
    }

    fn title(&self) -> Element<Message> {
        widget::text::title1(fl!("welcome")).into()
    }

    fn button(&self) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        widget::button::button(widget::text::text(fl!("save")))
            .on_press(Message::Submit)
            .padding(spacing.space_xxs)
            .into()
    }

    pub fn view(&self) -> Element<Message> {
        widget::column()
            .push(self.title())
            .push(
                widget::text_input::text_input("Enter your name", &self.input)
                    .on_input(Message::Input)
                    .size(10)
                    .label("Name")
                    .padding(10),
            )
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
        }
    }
    
}

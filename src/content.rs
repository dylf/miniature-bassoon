use cosmic::widget;
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Length;
use cosmic::{Apply, Element};
use crate::fl;

pub struct Content {

}

#[derive(Debug, Clone)]
pub enum Message {
    DoNothing,
}

impl Content {
    pub fn new() -> Self {
        Self {}
    }

    fn title(&self) -> Element<Message> {
        widget::text::title1(fl!("welcome"))
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Top)
            .into()
    }

    fn button(&self) -> Element<Message> {
        widget::button::button(widget::text::text(fl!("click_me")))
            .on_press(Message::DoNothing)
            .padding(10)
            .into()
    }

    pub fn view(&self) -> Element<Message> {
        widget::column()
            .push(self.title())
            .push(self.button())
            .into()
    }
}

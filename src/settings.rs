use cosmic::{cosmic_theme, theme, widget, Element};

use crate::{app::{App, Message as AppMessage}, fl};


#[derive(Debug, Clone)]
pub enum Message {
    CloseToTray(bool),
}

pub enum Task {
    Save,
}

impl App {

pub fn settings(&self) -> Element<AppMessage> {
    let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

    // let icon = widget::svg(widget::svg::Handle::from_memory(
    //     &include_bytes!("../res/icons/hicolor/128x128/apps/com.example.CosmicAppTemplate.svg")
    //         [..],
    // ));

    let title = widget::text::title3(fl!("menu-settings"));
    let checkbox = widget::checkbox(fl!("close-to-tray"), false).on_toggle(
            |val| { AppMessage::Setting(Message::CloseToTray(val))}
        );


    widget::column()
        // .push(icon)
        .push(title)
         .push(checkbox)
        // .align_items(Alignment::Center)
        .spacing(space_xxs)
        .into()
}

pub fn update_settings(&mut self, message: Message) -> Option<Task> {
    match message {
        Message::CloseToTray(_) => {
            None
        }
    }
}

}

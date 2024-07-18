use crate::device;
use crate::device::*;
use crate::fl;
use cosmic::widget;
use cosmic::{theme, Element};
use std::f32;

pub struct Content {
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Slider(u32, f32),
    Boolean(u32, bool),
    Menu(u32),
    Submit,
}

pub enum Command {
    Save(String),
}

impl Content {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    fn title(&self) -> Element<Message> {
        widget::text::title1(fl!("welcome")).into()
    }

    fn device_controls<'a>(&self, dev: &'a VideoDevice) -> Element<'a, Message> {
        let spacing = theme::active().cosmic().spacing;
        let form = widget::column()
            .padding([0, spacing.space_s, 0, 0])
            .spacing(spacing.space_xxs);
        let mut groups = 0;
        let form = form.push(widget::text::title2(String::from("Controls")))
            .push(widget::warning::warning(dev.capabilities.to_string()));
        
        dev.controls.iter().fold(form, |form, control| {
            match control {
                DeviceControls::ControlGroup(group) => {
                    let form = if groups > 0 {
                        form.push(widget::divider::horizontal::default())
                    } else {
                        form
                    };
                    groups += 1;

                    let form = form.push(widget::text::title4(group.name.clone()));
                    let form = group.controls.iter().fold(form, |form, control| {
                        match control {
                            device::DeviceControls::Boolean(control) => {
                                let val = control.value;
                                let id = control.id;
                                form.push(widget::toggler(control.name.clone(), val, move |x| { Message::Boolean(id, x)}))
                            }
                            device::DeviceControls::Integer(control) => {
                                let min = control.min as f32;
                                let max = control.max as f32;
                                let val = control.value as f32;
                                let id = control.id;
                                form.push(widget::text::text(control.name.clone()))
                                    .push(widget::slider(min..=max, val, move |x| { Message::Slider(id, x)}))
                            },
                            device::DeviceControls::Menu(control) => {
                                let val = control.value;
                                form.push(widget::text::text(control.name.clone()))
                                    .push(widget::dropdown(&control.menu_items, val, move |x| {
                                        Message::Menu(x as u32)
                                    }))
                            },
                            device::DeviceControls::Control(control) => {
                                form.push(
                                    widget::text::text(
                                        format!(
                                            "No Widget {}: {:?} - {:?}",
                                            control.name,
                                            control.control_type,
                                            control.value
                                        )
                                    )
                                )
                            } 
                            _ => form
                        }
                    });
                    form
                }
                DeviceControls::Control(control) => {
                    form.push(widget::text::text(control.name.clone()))
                }
                _ => form.push(widget::text::text("No Widget"))
            }
        }).into()
    }

    pub fn view<'a>(&'a self, dev: &'a VideoDevice) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        widget::scrollable(widget::column()
            .spacing(spacing.space_xs)
            .push(self.title())
            .push(self.device_controls(dev))
        ).into()
    }

    pub fn update(&mut self, dev: &VideoDevice, message: Message) -> Option<Command> {
        match message {
            Message::Submit => Some(Command::Save(self.input.clone())),
            Message::Slider(id, val) => {
                set_control_val(dev, id, v4l::control::Value::Integer(val as i64)).unwrap();
                None
            },
            Message::Boolean(id, val) => {
                set_control_val(dev, id, v4l::control::Value::Boolean(val)).unwrap();
                None
            }
            Message::Menu(_) => {
                None
            }
        }
    }
}

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

    fn device_controls(&self, dev: &v4l::device::Device) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        let form = widget::column().spacing(spacing.space_xxs);
        let mut groups = 0;
        match get_device_controls(dev) {
            Ok(controls) => {
                let form = form.push(widget::text::title2(String::from("Controls")))
                    .push(widget::warning::warning(get_caps_string(dev)));
                
                controls.iter().fold(form, |form, control| {
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
                                match control.control_type {
                                    v4l::control::Type::Boolean => {
                                        let val = match control.value {
                                            v4l::control::Value::Boolean(val) => val,
                                            _ => false,
                                        };
                                        let id = control.id;
                                        form.push(widget::toggler(control.name.clone(), val, move |x| { Message::Boolean(id, x)}))
                                    }
                                    v4l::control::Type::Integer => {

                                        let min = control.min as f32;
                                        let max = control.max as f32;
                                        let val = match control.value {
                                            v4l::control::Value::Integer(val) => val as f32,
                                            _ => 0.0,
                                        };
                                        let id = control.id;
                                        form.push(widget::text::text(control.name.clone()))
                                            .push(widget::slider(min..=max, val, move |x| { Message::Slider(id, x)}))
                                    },
                                    _ => form.push(widget::text::text(format!("No Widget {}: {:?}", control.name, control.control_type)))
                                }
                            });
                            form
                        }
                        DeviceControls::Control(control) => {
                            form.push(widget::text::text(control.name.clone()))
                        }
                    }
                }).into()
            }
            Err(e) => {
                form.push(widget::warning::warning(e)).into()
            }
        }
    }

    pub fn view(&self, dev: &v4l::device::Device) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        widget::column()
            .spacing(spacing.space_xs)
            .push(self.title())
            .push(self.device_controls(dev))
            .into()
    }

    pub fn update(&mut self, dev: &v4l::device::Device, message: Message) -> Option<Command> {
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
        }
    }
}

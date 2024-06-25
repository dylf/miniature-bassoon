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

    fn device_controls(&self, path: String) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        let form = widget::column().spacing(spacing.space_xxs);
        let mut groups = 0;
        let Ok(device) = get_device_by_path(&path) else {
            return form
                .push(widget::warning::warning(fl!("device-load-failed")))
                .into();
        };
        match get_device_controls(&device) {
            Ok(controls) => {
                let form = form.push(widget::text::title2(String::from("Controls")))
                    .push(widget::warning::warning(get_caps_string(&path)));
                
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
                                let min = control.min as f32;
                                let max = control.max as f32;
                                let default = control.default as f32;
                                let val = match control.value {
                                    v4l::control::Value::Integer(val) => val as f32,
                                    _ => 0.0,
                                };
                                let id = control.id;
                                println!("Default: {:?}", default);
                                println!("Val: {:?}", val);
                                form.push(widget::text::text(control.name.clone()))
                                    .push(widget::slider(min..=max, val, move |x| { Message::Slider(id, x)}))
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

    pub fn view(&self, device_path: String) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        widget::column()
            .spacing(spacing.space_xs)
            .push(self.title())
            .push(self.device_controls(device_path))
            .into()
    }

    pub fn update(&mut self, path: String, message: Message) -> Option<Command> {
        match message {
            Message::Submit => Some(Command::Save(self.input.clone())),
            Message::Slider(id, val) => {
                println!("Slider: {:?}", val);
                set_control_val(path, id, v4l::control::Value::Integer(val as i64)).unwrap();
                None
            }
        }
    }


    
}

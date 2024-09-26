use crate::device;
use crate::device::*;
use crate::fl;
use crate::storage::SaveData;
use cosmic::widget;
use cosmic::{theme, Element, theme::Theme};
use cosmic::theme::style::iced::Slider;
use std::rc::Rc;
use cosmic::iced_style::slider;
use cosmic::iced_style::slider::Rail;
use cosmic::iced_core::Color;
use std::f32;

pub struct Content {
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Slider(u32, f32),
    Boolean(u32, bool),
    Menu(u32, u32),
    ButtonPress(u32),
    Save,
    None,
}

pub enum Command {
    Save,
}

pub fn slider_style(disabled: bool) -> cosmic::theme::style::iced::Slider {
    if !disabled {
        return Slider::Standard;
    }
    let style = Rc::new(|theme: &Theme | {
        let cosmic = theme.cosmic();
        let disabled_color = cosmic.palette.neutral_3.into();
        slider::Appearance {
            rail: Rail {
                colors: slider::RailBackground::Pair(
                    disabled_color,
                    disabled_color,
                ),
                width: 4.0,
                border_radius: cosmic.corner_radii.radius_xs.into(),
            },

            handle: slider::Handle {
                shape: slider::HandleShape::Rectangle {
                    height: 20,
                    width: 20,
                    border_radius: cosmic.corner_radii.radius_m.into(),
                },
                color: disabled_color,
                border_color: Color::TRANSPARENT,
                border_width: 0.0,
            },

            breakpoint: slider::Breakpoint {
                color: disabled_color,
            },
        }
    });
    Slider::Custom {
        active: style.clone(),
        hovered: style.clone(),
        dragging: style.clone(),
    }
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
                                let default = control.default as f32;
                                let id = control.id;
                                let disabled = control.is_disabled();
                                form.push(widget::text::text(format!("{}: {}", control.name, control.value)))
                                    .push(widget::slider(
                                        min..=max, val,
                                        move |x| {
                                            if disabled {
                                                return Message::None;
                                            }
                                            Message::Slider(id, x)
                                        })
                                        .step(control.step as f32)
                                        .style(slider_style(disabled))
                                    )
                                    .push(
                                        widget::tooltip(
                                        widget::button::icon(widget::icon::from_svg_bytes(
                                            &include_bytes!("../res/icons/reset.svg")
                                            [..],
                                        ))
                                        .on_press(Message::Slider(id, default)),
                                        fl!("reset-control"),
                                        widget::tooltip::Position::Bottom,
                                        )
                                    )
                            },
                            device::DeviceControls::Menu(control) => {
                                let val = control.menu_items.iter().position(|x| x.id == (control.value.unwrap_or(0) as u32));
                                let id = control.id;
                                form.push(widget::text::text(control.name.clone()))
                                    .push(widget::dropdown(&control.menu_items, val, move |x| {
                                        let item = control.menu_items[x].id;
                                        Message::Menu(id, item)
                                    }))
                            },
                            device::DeviceControls::Button(control) => {
                                let id = control.id;
                                form.push(
                                    widget::button(widget::text::text(control.name.clone()))
                                        .on_press(Message::ButtonPress(id))
                                        .padding([spacing.space_xxs, spacing.space_s])
                                )
                            },
                            device::DeviceControls::Control(control) => {
                                form.push(
                                    widget::text::text(
                                        format!(
                                            "No Widget {}: {:?} - {:?}",
                                            control.name,
                                            control.control_type,
                                            control
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
        })
            .push(widget::button(widget::text::text(fl!("save")))
                .on_press(Message::Save)
                .padding([spacing.space_xxs, spacing.space_s])
            )
            .into()
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
            Message::None => None,
            Message::Save => Some(Command::Save),
            Message::Slider(id, val) => {
                set_control_val(dev, id, v4l::control::Value::Integer(val as i64)).unwrap();
                None
            },
            Message::Boolean(id, val) => {
                set_control_val(dev, id, v4l::control::Value::Boolean(val)).unwrap();
                None
            }
            Message::Menu(id, val) => {
                set_control_val(dev, id, v4l::control::Value::Integer(val as i64)).unwrap();

                None
            }
            Message::ButtonPress(id) => {
                set_control_val(dev, id, v4l::control::Value::None).unwrap();
                None
            },
        }
    }
}

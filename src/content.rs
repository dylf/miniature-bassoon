use crate::device;
use crate::device::*;
use crate::fl;
use crate::widgets;
use cosmic::iced::Alignment;
use cosmic::widget;
use cosmic::{theme, Element};
use std::f32;

pub struct Content {
    open_dialog: OpenDialog,
}

#[derive(Debug, Clone)]
pub enum OpenDialog {
    None,
    Info,
}

#[derive(Debug, Clone)]
pub enum Message {
    Slider(u32, f32),
    Boolean(u32, bool),
    Menu(u32, u32),
    ButtonPress(u32),
    OpenDialog(OpenDialog),
    Save,
    None,
}

pub enum Task {
    Save,
}

impl Content {
    pub fn new() -> Self {
        Self {
            open_dialog: OpenDialog::None,
        }
    }

    fn title(&self) -> Element<Message> {
        widget::text::title1(fl!("welcome")).into()
    }

    fn dialog_button<'a>(
        &self,
        button_label: impl Into<std::borrow::Cow<'a, str>>,
        dialog_content: impl Into<std::borrow::Cow<'a, str>>,
    ) -> Element<'a, Message> {
        let mut popover = widget::popover(
            widget::button::standard(
                button_label
            ).on_press(Message::OpenDialog(OpenDialog::Info))
        );
        if let OpenDialog::Info = self.open_dialog {
            popover = popover.popup(
                widget::dialog().body(dialog_content)
            )
            .on_close(Message::OpenDialog(OpenDialog::None));
        }
        popover.into()
    }

    fn device_controls<'a>(&self, dev: &'a VideoDevice) -> Element<'a, Message> {
        let spacing = theme::active().cosmic().spacing;
        let form = widget::column()
            .padding([0, spacing.space_s, 0, 0])
            .spacing(spacing.space_xxs)
            .push(widget::text::title2(dev.name.clone()))
            .push(
                widget::row()
                    .push(
                        self.dialog_button(
                            fl!("show-device-info"),
                            dev.capabilities.to_string()
                        )
                        // widget::popover(
                        //     widget::button::standard(
                        //         fl!("show-device-info")
                        //     )
                        // ).popup(
                        //     widget::dialog(dev.capabilities.to_string())
                        //     // widget::dialog(fl!("show-device-info"))
                        // )
                    )
            );
        let mut groups = 0;
        let form = form.push(widget::text::title3(String::from("Controls")));

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
                                let default = control.default;
                                let disabled = control.is_disabled();
                                form.push(
                                    widget::row()
                                        .align_y(Alignment::Center)
                                        .spacing(spacing.space_s)
                                        .push(
                                            widget::toggler(val)
                                                .label(control.name.clone())
                                                .on_toggle(
                                                    move |x| {
                                                        Message::Boolean(id, x)
                                                    }
                                                )
                                        ).push(
                                            widgets::reset_button(Message::Boolean(id, default), fl!("reset-control"), disabled || default == val)
                                        )
                                )
                            }
                            device::DeviceControls::Integer(control) => {
                                let min = control.min as f32;
                                let max = control.max as f32;
                                let val = control.value as f32;
                                let default = control.default as f32;
                                let id = control.id;
                                let disabled = control.is_disabled();
                                form.push(
                                    widget::row()
                                        .align_y(Alignment::Center)
                                        .spacing(spacing.space_s)
                                        .push(
                                            widget::text::text(format!("{}: {}", control.name, control.value))
                                        ).push(
                                            widgets::reset_button(Message::Slider(id, default), fl!("reset-control"), disabled || default == val)
                                        )
                                    ).push(crate::widgets::custom_slider(
                                        min,
                                        max,
                                        val,
                                        control.step as f32,
                                        disabled,
                                        move |x| {
                                            if disabled {
                                                return Message::None;
                                            }
                                            Message::Slider(id, x)
                                        })
                                    )
                            },
                            device::DeviceControls::Menu(control) => {
                                let val = control.menu_items.iter().position(|x| x.id == (control.value.unwrap_or(0) as u32));
                                let default = control.default as u32;
                                let ctrl_val = control.value.unwrap_or(0) as u32;
                                let disabled = control.is_disabled();
                                let id = control.id;
                                form.push(
                                    widget::row()
                                        .align_y(Alignment::Center)
                                        .spacing(spacing.space_s)
                                        .push(
                                            widget::text::text(control.name.clone())
                                        ).push(
                                            widgets::reset_button(
                                                Message::Menu(id, default),
                                                fl!("reset-control"),
                                                disabled || default == ctrl_val
                                            )
                                        )
                                    )
                                    .push(widget::dropdown(&control.menu_items, val, move |x| {
                                        let item = control.menu_items[x].id;
                                        Message::Menu(id, item)
                                    }))
                            },
                            device::DeviceControls::Button(control) => {
                                let id = control.id;
                                form.push(
                                    widget::button::standard(control.name.clone())
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
            .push(widget::button::standard(fl!("save"))
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

    pub fn update(&mut self, dev: &VideoDevice, message: Message) -> Option<Task> {
        match message {
            Message::None => None,
            Message::Save => Some(Task::Save),
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
            Message::OpenDialog(dialog) => {
                self.open_dialog = dialog;
                None
            },
        }
    }
}

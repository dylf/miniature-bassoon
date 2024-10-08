use std::borrow::Cow;

use cosmic::{widget, Element};
use crate::content::Message;

pub fn reset_button<'a>(
    msg: Message,
    tooltip: impl Into<Cow<'a, str>>,
    is_disabled: bool,
) -> Element<'a, Message> {
    if is_disabled {
       return widget::horizontal_space(0.0).into();
    }
    let icon = widget::icon::from_name("object-rotate-left-symbolic");
    widget::button::icon(icon)
        .on_press(msg)
        .extra_small()
        .tooltip(tooltip)
        .padding(0.0)
        .into()
}

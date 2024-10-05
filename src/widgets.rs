use std::borrow::Cow;

use cosmic::widget;
use crate::content::Message;

pub fn reset_button<'a>(
    msg: Message,
    tooltip: impl Into<Cow<'a, str>>
) -> widget::Tooltip<'a, Message> {
    let icon = widget::icon::from_name("object-rotate-left-symbolic");
    let button = widget::button::icon(icon).on_press(msg);
    widget::tooltip::<'a>(
        button,
        tooltip,
        widget::tooltip::Position::Top
    )
}

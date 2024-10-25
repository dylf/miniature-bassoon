use std::{borrow::Cow, rc::Rc};

use cosmic::{iced::Background, widget::{self, slider, Slider}, Element};
use crate::content::Message;
use cosmic::theme::iced::Slider as SliderTheme;
use cosmic::theme::Theme;

pub fn reset_button<'a>(
    msg: Message,
    tooltip: impl Into<Cow<'a, str>>,
    is_disabled: bool,
) -> Element<'a, Message> {
    if is_disabled {
       return widget::horizontal_space().into();
    }
    let icon = widget::icon::from_name("object-rotate-left-symbolic");
    widget::button::icon(icon)
        .on_press(msg)
        .extra_small()
        .tooltip(tooltip)
        .padding(0.0)
        .into()
}

pub fn custom_slider<'a, T, Message>(min: T, max: T, val: T, step: T, disabled: bool, on_change: impl Fn(T) -> Message + 'a ) -> Slider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
{
    let slider = widget::slider::<'a, T, Message, Theme>(
        min..=max,
        val,
        on_change
    );
    if !disabled {
        return slider
    }
    slider.class(SliderTheme::Custom {
        active: Rc::new(slider_disabled_style),
        hovered: Rc::new(slider_disabled_style),
        dragging: Rc::new(slider_disabled_style),
    })
}

fn slider_disabled_style(t: &Theme) -> slider::Style {
    let cosmic = t.cosmic();
    let disabled_color = cosmic.palette.neutral_3.into();
    let mut s =
        slider::Catalog::style(t, &SliderTheme::default(), slider::Status::Active);
    s.rail.backgrounds = (
        Background::Color(disabled_color),
        Background::Color(disabled_color),
    );
    s.handle.background = Background::Color(disabled_color);
    s.breakpoint.color = disabled_color;
    s
}

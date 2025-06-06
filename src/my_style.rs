// We place all the style stuff in its own file to reduce clutter

use cursive::theme::Style;
use cursive::style::BaseColor::*;
use cursive::style::ColorStyle;
use cursive::style::ColorType;
use cursive::style::Effects;
use cursive::theme::Effect;
use cursive::utils::span::SpannedString;

pub fn emphasis_style() -> Style {
    let color_style = ColorStyle::new(Yellow.dark(), ColorType::InheritParent);
    let effects = Effects::only(Effect::Bold);
    let style = Style {
        effects: effects,
        color: color_style
    };
    return style;
}

#[allow(dead_code)]
pub fn emphasis_string(text: &str) -> SpannedString<Style> {
    return SpannedString::styled(text, emphasis_style());
}

pub fn plain_string(text: &str) -> SpannedString<Style> {
    return SpannedString::<Style>::plain(text);
}

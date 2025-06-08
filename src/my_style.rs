// We place all the style stuff in its own file to reduce clutter

use cursive::theme::Style;
use cursive::style::BaseColor::*;
use cursive::style::ColorStyle;
use cursive::style::ColorType;
use cursive::style::Effects;
use cursive::theme::Effect;
use cursive::utils::span::SpannedString;

pub fn yellow_style() -> ColorStyle {
    return ColorStyle::new(Yellow.dark(), ColorType::InheritParent);
}

pub fn bold_yellow_style() -> Style {
    let effects = Effects::only(Effect::Bold);
    let style = Style {
        effects: effects,
        color: yellow_style()
    };
    return style;
}

pub fn green_style() -> ColorStyle {
    return ColorStyle::new(Green.dark(), ColorType::InheritParent);
}

pub fn bold_green_style() -> Style {
    let effects = Effects::only(Effect::Bold);
    let style = Style {
        effects: effects,
        color: green_style()
    };
    return style;
}

#[allow(dead_code)]
pub fn yellow_string(text: &str) -> SpannedString<Style> {
    return SpannedString::styled(text, yellow_style());
}

pub fn bold_yellow_string(text: &str) -> SpannedString<Style> {
    return SpannedString::styled(text, bold_yellow_style());
}

pub fn bold_green_string(text: &str) -> SpannedString<Style> {
    return SpannedString::styled(text, bold_green_style());
}

#[allow(dead_code)]
pub fn green_string(text: &str) -> SpannedString<Style> {
    return SpannedString::styled(text, green_style());
}

pub fn plain_string(text: &str) -> SpannedString<Style> {
    return SpannedString::<Style>::plain(text);
}

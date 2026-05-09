use iced::{font, Font};
use iced::widget::Text;

pub const ICON_LAN: &str = "\u{eb2f}";
pub const ICON_ACCOUNT_TREE: &str = "\u{e97a}";

pub fn load_fonts() -> Vec<std::borrow::Cow<'static, [u8]>> {
    vec![
        include_bytes!("../assets/fonts/gsans_code.ttf").as_slice().into(),
        include_bytes!("../assets/fonts/gsans_code_bold.ttf").as_slice().into(),
        include_bytes!("../assets/fonts/material_symbols_rounded.ttf").as_slice().into(),
    ]
}

pub const GSANSCODE_BOLD: Font = Font {
    family: font::Family::Name("Google Sans Code"),
    weight: font::Weight::Bold,
    stretch: font::Stretch::Normal,
    style: font::Style::Normal,
};

pub fn icon(codepoint: &str) -> Text<'static> {
    Text::new(codepoint.to_string())
        .font(Font {
            family: font::Family::Name("Material Symbols Rounded"),
            ..Font::DEFAULT
        })
}
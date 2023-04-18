//! Theme colors for demo application

use rtwins::colors::{ColorBg, ColorFg};
use rtwins::esc;

// ---------------------------------------------------------------------------------------------- //

// RGB color values: https://en.wikipedia.org/wiki/Web_colors

#[allow(dead_code)]
#[repr(u8)]
pub enum ColorFgTheme {
    // double-state colors
    Checkbox = ColorFg::Theme00 as u8,
    CheckboxIntense,
    // single-state colors
    Window,
    ButtonGreen,
    ButtonRed,
    ButtonOrange,
    PanelChbox,
    //
    ThemeEnd,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum ColorBgTheme {
    Window = ColorBg::Theme00 as u8,
    ButtonGreen,
    ButtonRed,
    ButtonOrange,
    PanelChbox,
    PanelVer,
    PanelKeyCodes,
    PanelLeds,
    Edit1,
    Edit1Intense,
    Edit2,
    Edit2Intense,
    EditPsw,
    EditPswIntense,
    LabelFtr,
    //
    ThemeEnd,
}

rtwins::static_assert!((ColorFgTheme::ThemeEnd as u8) <= ColorFg::ThemeEnd as u8);
rtwins::static_assert!((ColorBgTheme::ThemeEnd as u8) <= ColorBg::ThemeEnd as u8);

impl ColorFgTheme {
    pub const fn into(self) -> ColorFg {
        // SAFETY: static_assert() assure that the self value is within correct ColorFG range
        unsafe { core::mem::transmute::<ColorFgTheme, ColorFg>(self) }
    }

    pub const fn from(cl: ColorFg) -> ColorFgTheme {
        // SAFETY: static_assert() assure that the self value is within correct ColorFG range
        unsafe { core::mem::transmute::<ColorFg, ColorFgTheme>(cl) }
    }
}

impl ColorBgTheme {
    pub const fn into(self) -> ColorBg {
        // SAFETY: static_assert() assure that the self value is within correct ColorBG range
        unsafe { core::mem::transmute::<ColorBgTheme, ColorBg>(self) }
    }

    pub const fn from(cl: ColorBg) -> ColorBgTheme {
        // SAFETY: static_assert() assure that the self value is within correct ColorBG range
        unsafe { core::mem::transmute::<ColorBg, ColorBgTheme>(cl) }
    }
}

fn color_fg_theme_encode(cl: ColorFg) -> &'static str {
    let cl = ColorFgTheme::from(cl);

    match cl {
        // double-state colors
        ColorFgTheme::Checkbox => esc::FG_DARK_CYAN,
        ColorFgTheme::CheckboxIntense => esc::FG_DARK_TURQUOISE,
        // single-state colors
        ColorFgTheme::Window => rtwins::fg_color!(158),
        ColorFgTheme::ButtonGreen => esc::FG_WHITE,
        ColorFgTheme::ButtonRed => esc::FG_WHITE,
        ColorFgTheme::ButtonOrange => esc::FG_DARK_RED,
        ColorFgTheme::PanelChbox => esc::FG_MEDIUM_BLUE,
        _ => esc::FG_DEFAULT,
    }
}

fn color_bg_theme_encode(cl: ColorBg) -> &'static str {
    let cl = ColorBgTheme::from(cl);

    match cl {
        ColorBgTheme::Window => esc::BG_MIDNIGHT_BLUE,
        ColorBgTheme::ButtonGreen => esc::BG_OLIVE_DRAB,
        ColorBgTheme::ButtonRed => esc::BG_RED,
        ColorBgTheme::ButtonOrange => esc::BG_ORANGE,
        ColorBgTheme::PanelChbox => esc::BG_GAINSBORO,
        ColorBgTheme::PanelVer => rtwins::bg_color!(106),
        ColorBgTheme::PanelKeyCodes => rtwins::bg_color!(169),
        ColorBgTheme::PanelLeds => esc::BG_LIGHT_BLUE,
        ColorBgTheme::Edit1 => esc::BG_CYAN,
        ColorBgTheme::Edit1Intense => esc::BG_CYAN_INTENSE,
        ColorBgTheme::Edit2 => esc::BG_GREEN,
        ColorBgTheme::Edit2Intense => esc::BG_GREEN_INTENSE,
        ColorBgTheme::EditPsw => esc::BG_STEEL_BLUE,
        ColorBgTheme::EditPswIntense => esc::BG_DODGER_BLUE,
        ColorBgTheme::LabelFtr => esc::BG_NAVY,
        _ => esc::BG_DEFAULT,
    }
}

fn color_fg_theme_intensify(cl: ColorFg) -> ColorFg {
    let cl = match ColorFgTheme::from(cl) {
        ColorFgTheme::Checkbox => ColorFgTheme::CheckboxIntense,
        other => other,
    };

    cl.into()
}

fn color_bg_theme_intensify(cl: ColorBg) -> ColorBg {
    let cl = match ColorBgTheme::from(cl) {
        ColorBgTheme::Edit1 => ColorBgTheme::Edit1Intense,
        ColorBgTheme::Edit2 => ColorBgTheme::Edit2Intense,
        ColorBgTheme::EditPsw => ColorBgTheme::EditPswIntense,
        other => other,
    };

    cl.into()
}

pub fn init() {
    ColorFg::set_theme_encoder(color_fg_theme_encode);
    ColorFg::set_theme_intensifier(color_fg_theme_intensify);
    ColorBg::set_theme_encoder(color_bg_theme_encode);
    ColorBg::set_theme_intensifier(color_bg_theme_intensify);
}

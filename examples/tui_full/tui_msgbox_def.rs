//! Simple widget-based interface definition as const

use rtwins::colors::{ColorBg, ColorFg};
use rtwins::common::*;
use rtwins::wgt::prop;
use rtwins::wgt::*;

use super::tui_colors::*;

// ---------------------------------------------------------------------------------------------- //

pub mod idmb {
    use rtwins::wgt::{WId, WIDGET_ID_NONE};

    #[rustfmt::skip]
    rtwins::generate_ids!(
        WND_MSGBOX
            LBL_MSG
            BTN_YES
            BTN_NO
            BTN_CANCEL
            BTN_OK
    );
}

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const WINDOW_MSGBOX: Widget = Widget {
    id: idmb::WND_MSGBOX,
    link: Link::cdeflt(),
    coord: Coord::cdeflt(),
    size: Size { width: 34, height: 10 },
    prop: prop::Window {
        title: "",
        fg_color: ColorFg::Blue,
        bg_color: ColorBg::White,
        is_popup: true,
    }.into(),
    children: &[
        Widget {
            id: idmb::LBL_MSG,
            coord: Coord { col: 2, row: 2 },
            size: Size { width: 30, height: 4 },
            prop: prop::Label {
                title: "",
                fg_color: ColorFg::Inherit,
                bg_color: ColorBg::Inherit,
            }.into(),
            ..Widget::cdeflt()
        },

        Widget {
            id: idmb::BTN_YES,
            coord: Coord { col: 5, row: 7 },
            prop: prop::Button {
                text: "YES",
                fg_color: ColorFgTheme::ButtonGreen.into(),
                bg_color: ColorBgTheme::ButtonGreen.into(),
                style: ButtonStyle::Solid
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: idmb::BTN_NO,
            coord: Coord { col: 13, row: 7 },
            prop: prop::Button {
                text: "NO",
                fg_color: ColorFgTheme::ButtonRed.into(),
                bg_color: ColorBgTheme::ButtonRed.into(),
                style: ButtonStyle::Solid
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: idmb::BTN_CANCEL,
            coord: Coord { col: 20, row: 7 },
            prop: prop::Button {
                text: "CANCEL",
                fg_color: ColorFg::White,
                bg_color: ColorBg::BlackIntense,
                style: ButtonStyle::Solid
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: idmb::BTN_OK,
            coord: Coord { col: 13, row: 7 },
            prop: prop::Button {
                text: "OK",
                fg_color: ColorFgTheme::ButtonGreen.into(),
                bg_color: ColorBgTheme::ButtonGreen.into(),
                style: ButtonStyle::Solid
            }.into(),
            ..Widget::cdeflt()
        },

    ]
};

/// Example of const-evaluated and translated Widgets tree into Widgets array
pub const WND_MSGBOX_WGTS: [Widget; transform::tree_wgt_count(&WINDOW_MSGBOX)] =
    transform::tree_to_array(&WINDOW_MSGBOX);

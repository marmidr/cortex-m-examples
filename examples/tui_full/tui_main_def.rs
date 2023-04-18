//! Simple widget-based interface definition as const

use rtwins::colors::{ColorBg, ColorFg};
use rtwins::common::*;
use rtwins::wgt::prop;
use rtwins::wgt::*;

use super::tui_colors::*;

// ---------------------------------------------------------------------------------------------- //

pub mod id {
    use rtwins::wgt::{WId, WIDGET_ID_NONE};

    #[rustfmt::skip]
    rtwins::generate_ids!(
        WND_MAIN
            BTN_TOASTER
            PG_CONTROL
                PAGE_PANELS
                    PANEL_VERSIONS
                        LABEL_FW_VERSION
                        LABEL_DATE
                        LABEL_ABOUT
                    PANEL_STATE
                        LED_PUMP
                        LED_LOCK
                        LED_BATTERY
                    PANEL_KEY
                        LABEL_INPSEQ
                        LABEL_INPNAME
                    CHBX_ENBL
                    CHBX_LOCK
                    BTN_YES
                    BTN_NO
                    BTN_POPUP
                    PRGBAR1
                    PRGBAR2
                    PRGBAR3
                PAGE_LISTBOX
                    LAYER1
                        LABEL_MULTI_FMT
                        LIST_BOX
                    LAYER2
                        RADIO1
                        RADIO2
                        RADIO3
                    CHBX_L1
                    CHBX_L2
                PAGE_TEXTEDIT
                    PANEL_EDT
                        LBL_EDT1_TITLE
                        EDIT1
                        LBL_EDT2_TITLE
                        EDIT2
                        LBL_EDIT_PSW_TITLE
                        EDIT_PSW
                    PANEL_CHBX
                        LBL_CHBX_TITLE
                        CHBX_A
                        CHBX_B
                        CHBX_C
                        CHBX_D
                PAGE_COMBOBOX
                    CBX_OPTIONS
                    CBX_COLORS
                    LBX_UNDEROPTIONS
                    BTN_SAY_YES
                    BTN_SAY_NO
                    BTN_1P5
                PAGE_TEXTBOX
                    TBX_WIDE
                    TBX_NARROW
                PAGE_MISC
                    LBL_CUSTOM
                    CUSTOM_WGT1
                PAGE_INACTIV
                    PANEL_EMPTY_1
                        LBL_WORDWRAP
                        BTN_NOACTION
                    PANEL_EMPTY_2
                        LBL_EMPTY2
            LABEL_FTR
    );
}

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const PNL_STATE_CHILDREN: &[Widget] = &[
    Widget {
        id: id::LED_BATTERY,
        coord: Coord { col: 2, row: 1 },
        prop: prop::Led {
            text: "(BATT)",
            fg_color: ColorFg::Black,
            bg_color_off: ColorBg::White,
            bg_color_on: ColorBg::Magenta,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::LED_LOCK,
        coord: Coord { col: 9, row: 1 },
        prop: prop::Led {
            text: "(LOCK)",
            fg_color: ColorFg::Black,
            bg_color_off: ColorBg::White,
            bg_color_on: ColorBg::Green,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::LED_PUMP,
        coord: Coord { col: 16, row: 1 },
        prop: prop::Led {
            text: "(PUMP)",
            fg_color: ColorFg::Red,
            bg_color_off: ColorBg::White,
            bg_color_on: ColorBg::Yellow,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_PANELS_CHILDREN: &[Widget] = &[
    Widget {
        id: id::PANEL_VERSIONS,
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 1 },
        size: Size { width: 21, height: 5 },
        prop: prop::Panel {
            title: "VER 🍁",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::PanelVer.into(),
            no_frame: false,
        }.into(),
        children: &[
            Widget {
                id: id::LABEL_FW_VERSION,
                coord: Coord { col: 2, row: 1 },
                prop: prop::Label {
                    title: "FwVer: 1.1",
                    fg_color: ColorFg::Blue,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LABEL_DATE,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 18, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::Black,
                    bg_color: ColorBg::White,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LABEL_ABOUT,
                coord: Coord { col: 2, row: 3 },
                size: Size { width: 0, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ]
    },
    Widget {
        id: id::PANEL_STATE,
        link: Link::cdeflt(),
        coord: Coord { col: 30, row: 1 },
        size: Size { width: 25, height: 3 },
        prop: prop::Panel {
            title: "STATE: Leds",
            fg_color: ColorFg::Blue,
            bg_color: ColorBgTheme::PanelLeds.into(),
            no_frame: false,
        }.into(),
        children: PNL_STATE_CHILDREN
    },
    Widget {
        id: id::PANEL_KEY,
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 7 },
        size: Size { width: 26, height: 4 },
        prop: prop::Panel {
            title: "KEY-CODES",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::PanelKeyCodes.into(),
            no_frame: false,
        }.into(),
        children: &[
            Widget {
                id: id::LABEL_INPSEQ,
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 22, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LABEL_INPNAME,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 22, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ]
    },
    Widget {
        id: id::CHBX_ENBL,
        coord: Coord { col: 30, row: 5 },
        prop: prop::CheckBox {
            text: "Enable",
            fg_color: ColorFg::White,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::CHBX_LOCK,
        coord: Coord { col: 45, row: 5 },
        prop: prop::CheckBox {
            text: "Lock",
            fg_color: ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BTN_YES,
        coord: Coord { col: 30, row: 7 },
        prop: prop::Button {
            text: "YES",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::ButtonGreen.into(),
            style: ButtonStyle::Solid
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BTN_NO,
        coord: Coord { col: 38, row: 7 },
        prop: prop::Button {
            text: "NO",
            fg_color: ColorFgTheme::ButtonOrange.into(),
            bg_color: ColorBgTheme::ButtonOrange.into(),
            style: ButtonStyle::Solid
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BTN_POPUP,
        coord: Coord { col: 45, row: 7 },
        prop: prop::Button {
            text: "POPUP",
            fg_color: ColorFg::White,
            bg_color: ColorBg::Inherit,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::PRGBAR1,
        coord: Coord { col: 30, row: 9 },
        size: Size { width: 25, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::Yellow,
            style: PgBarStyle::Hash
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::PRGBAR2,
        coord: Coord { col: 30, row: 10 },
        size: Size { width: 12, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::White,
            style: PgBarStyle::Shade
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::PRGBAR3,
        coord: Coord { col: 43, row: 10 },
        size: Size { width: 12, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::Black,
            style: PgBarStyle::Rectangle
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_LISTBOX_CHILDREN: &[Widget] = &[
    Widget {
        id: id::LAYER1,
        prop: prop::Layer {
        }.into(),
        children: &[
            Widget {
                id: id::LABEL_MULTI_FMT,
                coord: Coord { col: 24, row: 2 },
                size: Size { width: 35, height: 4 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::YellowIntense,
                    bg_color: ColorBg::BlueIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LIST_BOX,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 20, height: 8 },
                prop: prop::ListBox {
                    fg_color: ColorFg::Green,
                    bg_color: ColorBg::White,
                    no_frame: false
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::LAYER2,
        prop: prop::Layer {
        }.into(),
        children: &[
            Widget {
                id: id::RADIO1,
                coord: Coord { col: 25, row: 7 },
                prop: prop::Radio {
                    text : "YES",
                    fg_color : ColorFg::GreenIntense,
                    group_id : 1,
                    radio_id : 0,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::RADIO2,
                coord: Coord { col: 35, row: 7 },
                prop: prop::Radio {
                    text : "NO",
                    fg_color : ColorFg::Yellow,
                    group_id : 1,
                    radio_id : 1,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::RADIO3,
                coord: Coord { col: 44, row: 7 },
                prop: prop::Radio {
                    text : "Don't know",
                    fg_color : ColorFg::Inherit,
                    group_id : 1,
                    radio_id : 3,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::CHBX_L1,
        coord: Coord { col: 25, row: 9 },
        prop: prop::CheckBox {
            text : "Layer 1",
            fg_color : ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::CHBX_L2,
        coord: Coord { col: 40, row: 9 },
        prop: prop::CheckBox {
            text : "Layer 2",
            fg_color : ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_TEXTEDIT_CHILDREN: &[Widget] = &[
    Widget {
        id: id::PANEL_EDT,
        coord: Coord { col: 2, row: 1 },
        size: Size { width: 32, height: 8 },
        prop: prop::Panel {
            title : "",
            fg_color : ColorFg::White,
            bg_color : ColorBg::White,
            no_frame : true,
        }.into(),
        children: &[
            Widget {
                id: id::LBL_EDT1_TITLE,
                coord: Coord { col: 1, row: 0 },
                size: Size { width: 30, height: 1 },
                prop: prop::Label {
                    title: "Text edit:",
                    fg_color: ColorFg::WhiteIntense,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::EDIT1,
                coord: Coord { col: 1, row: 1 },
                size: Size { width: 30, height: 1 },
                prop: prop::TextEdit {
                    fg_color: ColorFg::Black,
                    bg_color: ColorBgTheme::Edit1.into(),
                    psw_mask: false,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LBL_EDT2_TITLE,
                coord: Coord { col: 1, row: 2 },
                size: Size { width: 30, height: 1 },
                prop: prop::Label {
                    title: "Num edit: UP/DOWN + Ctr/Shift:",
                    fg_color: ColorFg::WhiteIntense,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::EDIT2,
                coord: Coord { col: 1, row: 3 },
                size: Size { width: 30, height: 1 },
                prop: prop::TextEdit {
                    fg_color: ColorFg::Black,
                    bg_color: ColorBgTheme::Edit2.into(),
                    psw_mask: false,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LBL_EDIT_PSW_TITLE,
                coord: Coord { col: 1, row: 5 },
                size: Size { width: 10, height: 1 },
                prop: prop::Label {
                    title: "Password:",
                    fg_color: ColorFg::WhiteIntense,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::EDIT_PSW,
                coord: Coord { col: 11, row: 5 },
                size: Size { width: 20, height: 1 },
                prop: prop::TextEdit {
                    fg_color: ColorFg::Black,
                    bg_color: ColorBgTheme::EditPsw.into(),
                    psw_mask: true,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::PANEL_CHBX,
        coord: Coord { col: 36, row: 1 },
        size: Size { width: 22, height: 7 },
        prop: prop::Panel {
            title : "",
            fg_color : ColorFgTheme::PanelChbox.into(),
            bg_color : ColorBgTheme::PanelChbox.into(),
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: id::LBL_CHBX_TITLE,
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 14, height: 1 },
                prop: prop::Label {
                    title: "Check list:", // concat!(bold!(), "Check list:", normal!()),
                    fg_color: ColorFg::Blue,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::CHBX_A,
                coord: Coord { col: 2, row: 2 },
                prop: prop::CheckBox {
                    text : "Check A ",
                    fg_color : ColorFg::Green,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::CHBX_B,
                coord: Coord { col: 2, row: 3 },
                prop: prop::CheckBox {
                    text : "Check B ",
                    fg_color : ColorFgTheme::Checkbox.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::CHBX_C,
                coord: Coord { col: 2, row: 4 },
                prop: prop::CheckBox {
                    text : "Check C ",
                    fg_color : ColorFgTheme::Checkbox.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::CHBX_D,
                coord: Coord { col: 2, row: 5 },
                prop: prop::CheckBox {
                    text : "Check D ",
                    fg_color : ColorFgTheme::Checkbox.into(),
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_TXTBOX_CHILDREN: &[Widget] = &[
    Widget {
        id: id::TBX_WIDE,
        coord: Coord { col: 3, row: 1 },
        size: Size { width: 40, height: 10 },
        prop: prop::TextBox {
            fg_color: ColorFg::White,
            bg_color: ColorBg::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::TBX_NARROW,
        coord: Coord { col: 45, row: 1 },
        size: Size { width: 13, height: 10 },
        prop: prop::TextBox {
            fg_color: ColorFg::White,
            bg_color: ColorBg::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_CMBBOX_CHILDREN: &[Widget] = &[
    Widget {
        id: id::CBX_OPTIONS,
        coord: Coord { col: 10, row: 2 },
        size: Size { width: 20, height: 1 },
        prop: prop::ComboBox {
            fg_color: ColorFg::Blue,
            bg_color: ColorBg::White,
            drop_down_size: 4
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::CBX_COLORS,
        coord: Coord { col: 8, row: 4 },
        size: Size { width: 24, height: 1 },
        prop: prop::ComboBox {
            fg_color: ColorFg::GreenIntense,
            bg_color: ColorBg::Black,
            drop_down_size: 4
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::LBX_UNDEROPTIONS,
        coord: Coord { col: 5, row: 6 },
        size: Size { width: 30, height: 5 },
        prop: prop::ListBox {
            fg_color: ColorFg::Inherit,
            bg_color: ColorBg::Inherit,
            no_frame: false
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BTN_SAY_YES,
        coord: Coord { col: 38, row: 2 },
        prop: prop::Button {
            text: "Say YES",
            fg_color: ColorFgTheme::ButtonGreen.into(),
            bg_color: ColorBg::Green,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BTN_SAY_NO,
        coord: Coord { col: 38, row: 4 },
        prop: prop::Button {
            text: "Say NO",
            fg_color: ColorFgTheme::ButtonRed.into(),
            bg_color: ColorBg::Red,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BTN_1P5,
        coord: Coord { col: 38, row: 7 },
        prop: prop::Button {
            text: "",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::ButtonGreen.into(),
            style: ButtonStyle::Solid1p5
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_INACT_CHILDREN: &[Widget] = &[
    Widget {
        id: id::PANEL_EMPTY_1,
        coord: Coord { col: 5, row: 1 },
        size: Size { width: 20, height: 10 },
        prop: prop::Panel {
            title : "Word-wrap",
            fg_color : ColorFg::Inherit,
            bg_color : ColorBg::Inherit,
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: id::LBL_WORDWRAP,
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 16, height: 6 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::BlueIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::BTN_NOACTION,
                coord: Coord { col: 5, row: 8 },
                prop: prop::Button {
                    text: "...",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                    style: ButtonStyle::Simple
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::PANEL_EMPTY_2,
        coord: Coord { col: 40, row: 1 },
        size: Size { width: 12, height: 10 },
        prop: prop::Panel {
            title : "...",
            fg_color : ColorFg::Inherit,
            bg_color : ColorBg::Inherit,
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: id::LBL_EMPTY2,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 1, height: 1 },
                prop: prop::Label {
                    title: "---",
                    fg_color: ColorFg::Inherit,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_MISC_CHILDREN: &[Widget] = &[
    Widget {
        id: id::LBL_CUSTOM,
        coord: Coord { col: 2, row: 1 },
        prop: prop::Label {
            title: "Custom Widget:",
            fg_color: ColorFg::Inherit,
            bg_color: ColorBg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::CUSTOM_WGT1,
        coord: Coord { col: 2, row: 2 },
        size: Size { width: 32, height: 8 },
        prop: prop::CustomWgt {
        }.into(),
        ..Widget::cdeflt()
    },
];

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const WINDOW_MAIN: Widget = Widget {
    id: id::WND_MAIN,
    link: Link::cdeflt(),
    coord: Coord { col: 5, row: 2 },
    size: Size { width: 80, height: 15 },
    prop: prop::Window {
        title: "",
        fg_color: ColorFgTheme::Window.into(),
        bg_color: ColorBgTheme::Window.into(),
        is_popup: false,
    }.into(),
    children: &[
        Widget {
            id: id::BTN_TOASTER,
            coord: Coord { col: 1, row: 1 },
            size: Size { width: 14, height: 1 },
            prop: prop::Button {
                text: "",
                fg_color: ColorFg::Yellow,
                bg_color: ColorBg::Inherit,
                style: ButtonStyle::Simple
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: id::PG_CONTROL,
            coord: Coord { col: 1, row: 1 },
            size: Size { width: 75, height: 12 },
            prop: prop::PageCtrl {
                tab_width: 14,
                vert_offs: 2
            }.into(),
            children: &[
                Widget {
                    id: id::PAGE_PANELS,
                    prop: prop::Page {
                        title: "Panels",
                        fg_color: ColorFg::Yellow,
                    }.into(),
                    children: PAGE_PANELS_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PAGE_LISTBOX,
                    prop: prop::Page {
                        title: "ListBox ∑",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_LISTBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PAGE_TEXTEDIT,
                    prop: prop::Page {
                        title: "TextEdit",
                        fg_color: ColorFg::Yellow,
                    }.into(),
                    children: PAGE_TEXTEDIT_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PAGE_COMBOBOX,
                    prop: prop::Page {
                        title: "Combo Box",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_CMBBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PAGE_TEXTBOX,
                    prop: prop::Page {
                        title: "Text Box",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_TXTBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PAGE_INACTIV,
                    prop: prop::Page {
                        title: "Misc.",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_MISC_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PAGE_MISC,
                    prop: prop::Page {
                        title: "Inactiv 🍀",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_INACT_CHILDREN,
                    ..Widget::cdeflt()
                },
            ],
            ..Widget::cdeflt()
        },
        Widget {
            id: id::LABEL_FTR,
            coord: Coord { col: 1, row: 13 },
            prop: prop::Label {
                title:  concat!(
                    " ",
                    rtwins::csi!("7m"),
                    "F2",
                    rtwins::csi!("27m"),
                    " Wnd En • ",
                    "F4 Mouse On • ",
                    "F5 Refresh • ",
                    "F6 Clr Logs • ",
                    "F9/F10 Page • ",
                    "\u{2581}\u{2582}\u{2583}\u{2584}\u{2585}\u{2586}\u{2587}\u{2588}"
                ),
                fg_color: ColorFg::White,
                bg_color: ColorBgTheme::LabelFtr.into(),
            }.into(),
            ..Widget::cdeflt()
        },
    ]
};

/// Example of const-evaluated and translated Widgets tree into Widgets array
pub const WND_MAIN_WGTS: [Widget; transform::tree_wgt_count(&WINDOW_MAIN)] =
    transform::tree_to_array(&WINDOW_MAIN);

//! Demo - window state

#![allow(unused_variables)]
#![allow(dead_code)]

use rtwins::common::*;
use rtwins::esc;
use rtwins::input::*;
use rtwins::string_ext::StringExt;
use rtwins::utils;
use rtwins::wgt::{self, rstate, WId, Widget, WIDGET_ID_NONE};
use rtwins::Term;
use rtwins::TERM;
use rtwins::*;

use super::tui_commands::*;
use super::tui_main_def::id;
use crate::tui_msgbox_def::idmb;

use core::cell::RefCell;

extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};

// ---------------------------------------------------------------------------------------------- //

/// State of all the DemoWindow widget dynamic properties
pub struct MainWndState {
    // id of the window
    pub wnd_id: WId,
    /// all window widgets, starting with the window widget itself
    widgets: &'static [wgt::Widget],
    /// widgets runtime state
    pub rs: wgt::RuntimeStates,
    /// currently focused widget, for each pagecontrol page
    focused_ids: Vec<WId>,
    /// list of widgets to redraw
    invalidated: Vec<WId>,
    //
    radiogrp1_idx: i16,
    //
    lbx_items: Vec<&'static str>,
    // text box raw source string and source splitted into rows
    tbx_text: String,
    tbx_wide_lines: utils::StringListRc,
    tbx_narrow_lines: utils::StringListRc,
    // app-wide commands queue
    cmds: Rc<RefCell<CommandsQueue>>,
}

impl MainWndState {
    pub fn new(wnd_id: WId, widgets: &'static [Widget], cmds: Rc<RefCell<CommandsQueue>>) -> Self {
        let mut wnd_state = MainWndState {
            wnd_id,
            widgets,
            rs: wgt::RuntimeStates::default(),
            focused_ids: vec![],
            invalidated: Vec::with_capacity(4),
            radiogrp1_idx: 1,
            lbx_items: vec![],
            tbx_text: String::with_capacity(400),
            tbx_wide_lines: Arc::new(RefCell::new(vec![])),
            tbx_narrow_lines: Arc::new(RefCell::new(vec![])),
            cmds,
        };

        // initial tsate of focused id for each page
        wnd_state.focused_ids.resize(
            rtwins::wgt::pagectrl_page_count(widgets, id::PG_CONTROL) as usize,
            WIDGET_ID_NONE,
        );

        wnd_state.lbx_items.extend_from_slice(&[
            "Black",
            "BlackIntense",
            "Red",
            "RedIntense",
            "Green",
            "GreenIntense",
            "Yellow",
            "YellowIntense",
            "Blue",
            "BlueIntense",
            "Magenta",
            "MagentaIntense",
            "Cyan",
            "CyanIntense",
            "White",
            "WhiteIntense",
        ]);

        // prepare text box content
        let _ = wnd_state.tbx_text.stream()
            << esc::BOLD
            << "🔶 Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam arcu magna, placerat sit amet libero at, aliquam fermentum augue.\n"
            << esc::NORMAL
            << esc::FG_GOLD
            << " Morbi egestas consectetur malesuada. Mauris vehicula, libero eget tempus ullamcorper, nisi lorem efficitur velit, vel bibendum augue eros vel lorem. Duis vestibulum magna a ornare bibendum. Curabitur eleifend dictum odio, eu ultricies nunc eleifend et.\n"
            << "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.\n"
            << esc::FG_GREEN_YELLOW
            << "🔷 Interdum et malesuada fames ac ante ipsum primis in faucibus. Aenean malesuada lacus leo, a eleifend lorem suscipit sed.\n"
            << "▄";

        // << esc::BOLD
        // << "🔶 Lorem ipsum \ndolor sit\n amet, consectetur adipiscing elit. Nam arcu magna, placerat sit amet libero at, aliquam fermentum augue. \n"
        // << esc::NORMAL
        // << "▄";

        // setup some widgets initial properties
        use rstate::*;
        wnd_state.rs.set_enabled(id::LABEL_FW_VERSION, false);

        wnd_state
            .rs
            .pgbar
            .insert(id::PRGBAR1, PgbarState { pos: 5, max: 10 });
        wnd_state
            .rs
            .pgbar
            .insert(id::PRGBAR2, PgbarState { pos: 2, max: 10 });
        wnd_state
            .rs
            .pgbar
            .insert(id::PRGBAR3, PgbarState { pos: 8, max: 10 });
        wnd_state
            .rs
            .led
            .insert(id::LED_LOCK, LedState { lit: true });
        wnd_state
            .rs
            .chbx
            .insert(id::CHBX_ENBL, ChbxState { checked: true });
        wnd_state
            .rs
            .pgctrl
            .insert(id::PG_CONTROL, PgctrlState { page: 0 });
        wnd_state.rs.txtbx.insert(
            id::TBX_WIDE,
            TxtbxState {
                top_line: 9,
                lines: Default::default(),
            },
        );
        wnd_state
            .rs
            .chbx
            .insert(id::CHBX_L1, ChbxState { checked: true });
        wnd_state
            .rs
            .chbx
            .insert(id::CHBX_L2, ChbxState { checked: true });
        wnd_state.rs.txte.insert(
            id::EDIT_PSW,
            TxteState {
                txt: "pssst!".to_owned(),
            },
        );

        wnd_state
    }
}

// -----------------------------------------------------------------------------------------------

impl rtwins::wgt::WindowState for MainWndState {
    /** events **/

    fn on_button_down(&mut self, wgt: &Widget, ii: &InputInfo) {
        if wgt.id == id::BTN_YES {
            rtwins::tr_debug!("▼ BTN_YES");
        }
        if wgt.id == id::BTN_NO {
            rtwins::tr_warn!("▼ BTN_NO");
        }
        if wgt.id == id::BTN_POPUP {
            rtwins::tr_info!("▼ BTN_POPUP");
        }
    }

    fn on_button_up(&mut self, wgt: &Widget, ii: &InputInfo) {
        match wgt.id {
            id::BTN_YES => rtwins::tr_debug!("▲ BTN_YES"),
            id::BTN_NO => rtwins::tr_warn!("▲ BTN_NO"),
            id::BTN_POPUP => rtwins::tr_info!("▲ BTN_POPUP"),
            id::BTN_SAY_NO => {
                self.rs.set_enabled(
                    id::BTN_SAY_YES,
                    !self.rs.get_enabled_or_default(id::BTN_SAY_YES),
                );

                self.invalidate(id::BTN_SAY_YES);

                self.rs
                    .set_enabled(id::BTN_1P5, !self.rs.get_enabled_or_default(id::BTN_1P5));

                self.invalidate(id::BTN_1P5);
            }
            _ => {}
        }
    }

    fn on_button_click(&mut self, wgt: &Widget, ii: &InputInfo) {
        rtwins::tr_debug!("BTN_CLICK");

        if wgt.id == id::BTN_POPUP {
            match self.cmds.try_borrow_mut() {
                Ok(ref mut cmds) => {
                    cmds.push(
                        Command::ShowPopup {
                            title: "Lorem Titlum".to_owned(),
                            message: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                                tempor incididunt ut labore et dolore magna aliqua. \
                                Ut enim ad minim veniam, quis nostrud exercitation ullamco \
                                laboris nisi ut aliquip ex ea commodo consequat.".to_owned(),
                            buttons: "ync",
                            on_button: Box::new(move |btn_id| {
                                let msg = match btn_id {
                                    idmb::BTN_YES => "YES",
                                    idmb::BTN_NO => "NO",
                                    idmb::BTN_CANCEL => "CANCEL",
                                    _ => ""
                                };
                                tr_info!("{}MsgBox callback: {msg}{}", esc::BG_DARK_CYAN, esc::BG_DEFAULT);
                            }),
                        }
                    );
                }
                Err(e) => {
                    tr_err!("Cannot borrow the commands");
                }
            }

            // panel shall remain hidden, as it lays on another page:
            self.invalidate(id::PANEL_EDT);
        }

        if wgt.id == id::BTN_YES {
            wgt::pagectrl_select_page(self, id::PG_CONTROL, id::PAGE_TEXTBOX);
        }
    }

    fn on_button_key(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        if wgt.id == id::BTN_1P5 {
            rtwins::tr_debug!("BTN_ON_KEY");

            if let InputEvent::Char(ref ch) = ii.evnt {
                if ch.utf8seq[0] == b' ' {
                    rtwins::wgt::mark_button_down(wgt, true);
                    self.instant_redraw(wgt.id);
                    // wait and unpress the button
                    // TODO: PAL::
                    // std::thread::sleep(core::time::Duration::from_millis(200));
                    rtwins::wgt::mark_button_down(wgt, false);
                    self.invalidate(wgt.id);
                    // clear input queue as it may be full of Keyboar key events;
                    // ... rbKeybInput is out of reach from here 🙁
                    return true;
                }
            }

            rtwins::wgt::mark_button_down(wgt, false);
            self.invalidate(wgt.id);
        }

        false
    }

    fn on_text_edit_change(&mut self, wgt: &Widget, txt: &mut String) {
        rtwins::tr_debug!("TXT_EDIT_CHANGE: {}", txt);
        self.rs.txte.entry(wgt.id).or_default().txt = core::mem::take(txt);
    }

    fn on_text_edit_input_evt(
        &mut self,
        wgt: &Widget,
        ii: &InputInfo,
        txt: &mut String,
        cursor_pos: &mut i16,
    ) -> bool {
        if wgt.id == id::EDIT2 {
            return rtwins::utils::num_edit_input_evt(
                ii,
                txt,
                cursor_pos,
                i64::MIN,
                i64::MAX,
                true,
            );
        }

        // false means key not handled, continue with default code
        false
    }

    fn on_checkbox_toggle(&mut self, wgt: &Widget) {
        let rs = self.rs.chbx.entry(wgt.id).or_default();
        rs.checked = !rs.checked;

        match wgt.id {
            id::CHBX_ENBL => {
                rtwins::tr_debug!("CHBX_ENBL");
                let en = self.rs.get_enabled_or_default(id::PANEL_STATE);
                self.rs.set_enabled(id::PANEL_STATE, !en);
                self.invalidate(id::PANEL_STATE);
            }
            id::CHBX_LOCK => rtwins::tr_debug!("CHBX_LOCK"),
            id::CHBX_L1 | id::CHBX_L2 => self.invalidate(id::PAGE_LISTBOX),
            _ => rtwins::tr_debug!("CHBX"),
        }
    }

    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: i16) {
        rtwins::tr_info!("NewPageIdx={}", new_page_idx);
        let rs = self.rs.pgctrl.entry(wgt.id).or_default();
        rs.page = new_page_idx;
    }

    fn on_list_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {
        rtwins::tr_debug!("LISTBOX_SELECT={}", new_sel_idx);
        let rs = self.rs.lbx.entry(wgt.id).or_default();
        rs.sel_idx = new_sel_idx;
    }

    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {
        rtwins::tr_debug!("LISTBOX_CHANGE={}", new_idx);
        let rs = self.rs.lbx.entry(wgt.id).or_default();
        rs.item_idx = new_idx;
    }

    fn on_combo_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {
        rtwins::tr_debug!("COMBOBOX_SELECT={}", new_sel_idx);
        let rs = self.rs.cbbx.entry(wgt.id).or_default();
        rs.sel_idx = new_sel_idx;
    }

    fn on_combo_box_change(&mut self, wgt: &Widget, new_idx: i16) {
        rtwins::tr_debug!("COMBOBOX_CHANGE={}", new_idx);
        let rs = self.rs.cbbx.entry(wgt.id).or_default();
        rs.item_idx = new_idx;
    }

    fn on_combo_box_drop(&mut self, wgt: &Widget, drop_state: bool) {
        rtwins::tr_debug!("COMBOBOX_DROP={}", drop_state);
        let rs = self.rs.cbbx.entry(wgt.id).or_default();
        rs.drop_down = drop_state;
    }

    fn on_radio_select(&mut self, wgt: &Widget) {
        if let wgt::Property::Radio(ref p) = wgt.prop {
            rtwins::tr_debug!("RADIO_SELECT.radio.id={}", p.radio_id);
            self.radiogrp1_idx = p.radio_id;
        }
    }

    fn on_text_box_scroll(&mut self, wgt: &Widget, top_line: i16) {
        let rs = self.rs.txtbx.entry(wgt.id).or_default();
        rs.top_line = top_line;
    }

    fn on_custom_widget_draw(&mut self, wgt: &Widget, term_cell: &RefCell<&mut Term>) {
        let coord = rtwins::wgt::get_screen_coord(self, wgt);
        let sz = &wgt.size;
        let mut term = term_cell.borrow_mut();

        term.move_to(coord.col as u16, coord.row as u16);
        term.write_char_n('-', sz.width as i16);
        term.move_to(coord.col as u16, coord.row as u16 + sz.height as u16);
        term.write_char_n('-', sz.width as i16);
    }

    fn on_custom_widget_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        if let InputEvent::Mouse(ref mouse) = ii.evnt {
            if let Some(mut term_guard) = TERM.try_lock() {
                let term = &mut *term_guard;
                term.move_to(mouse.col as u16, mouse.row as u16);
                let mark = mouse.evt.as_mark();
                term.write_char(mark);
            }
            else {
                rtwins::tr_warn!("Cannot lock the term");
            }
        }
        true
    }

    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        rtwins::tr_debug!("onWindowUnhandledInputEvt={}", ii.name);
        false
    }

    /** common state queries **/

    fn is_enabled(&self, wgt: &Widget) -> bool {
        if wgt.id == id::CHBX_C {
            false
        }
        else {
            self.rs.get_enabled_or_default(wgt.id)
        }
    }

    fn is_focused(&self, wgt: &Widget) -> bool {
        let rs_opt = self.rs.pgctrl.get(&id::PG_CONTROL);
        rs_opt.map_or(false, |rs| self.focused_ids[rs.page as usize] == wgt.id)
    }

    fn is_visible(&self, wgt: &Widget) -> bool {
        if let wgt::Property::Page(_) = wgt.prop {
            let pgctrl = wgt::get_parent(wgt);

            return self.rs.pgctrl.get(&pgctrl.id).map_or(true, |rs| {
                wgt::page_page_idx(wgt).map_or(false, |pg_idx| pg_idx == rs.page)
            });
        }

        if wgt.id == id::LAYER1 {
            return self.rs.chbx.get(&id::CHBX_L1).map_or(true, |rs| rs.checked);
        }

        if wgt.id == id::LAYER2 {
            return self.rs.chbx.get(&id::CHBX_L2).map_or(true, |rs| rs.checked);
        }

        true
    }

    fn is_desktop(&self) -> bool {
        true
    }

    fn get_focused_id(&mut self) -> WId {
        let rs = self.rs.pgctrl.entry(id::PG_CONTROL).or_default();
        self.focused_ids[rs.page as usize]
    }

    fn set_focused_id(&mut self, wid: WId) {
        let rs = self.rs.pgctrl.entry(id::PG_CONTROL).or_default();
        self.focused_ids[rs.page as usize] = wid;
    }

    fn get_widgets(&self) -> &'static [Widget] {
        self.widgets
    }

    fn get_rstate(&mut self) -> Option<&mut wgt::RuntimeStates> {
        Some(&mut self.rs)
    }

    /** widget-specific queries; all mutable params are outputs **/

    fn get_window_coord(&mut self) -> Coord {
        self.widgets.first().map_or(Coord::cdeflt(), |w| w.coord)
    }

    fn get_window_size(&mut self) -> Size {
        self.widgets.first().map_or(Size::cdeflt(), |w| w.size)
    }

    fn get_window_title(&mut self, wgt: &Widget, out: &mut String) {
        let _ = out.stream()
            << esc::BOLD
            << "** Service Menu **"
            << esc::NORMAL
            << esc::UNDERLINE_ON
            << " (Ctrl+D quit)"
            << esc::UNDERLINE_OFF;
    }

    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool {
        let rs = self.rs.chbx.entry(wgt.id).or_default();
        rs.checked
    }

    fn get_label_text(&mut self, wgt: &Widget, out: &mut String) {
        if wgt.id == id::LABEL_DATE {
            out.push_str(format!("Date•{}", "<datetime>").as_str());
        }
        else if wgt.id == id::LABEL_ABOUT {
            out.push_str(rtwins::url_link!(
                "https://github.com/marmidr/rtwins",
                "About..."
            ));
        }
        else if wgt.id == id::LABEL_MULTI_FMT {
            let _ = out.stream()
                << "  ▫▫▫▫▫ "
                << esc::INVERSE_ON
                << "ListBox manual:"
                << esc::INVERSE_OFF
                << " ▫▫▫▫▫\n"
                << "• "
                << esc::UNDERLINE_ON
                << "Up/Down"
                << esc::UNDERLINE_OFF
                << " -> change item\n"
                << "• "
                << esc::UNDERLINE_ON
                << "PgUp/PgDown"
                << esc::UNDERLINE_OFF
                << " -> scroll page\n"
                << "• "
                << esc::UNDERLINE_ON
                << "Enter"
                << esc::UNDERLINE_OFF
                << " -> select the item";
        }
        else if wgt.id == id::LBL_WORDWRAP {
            let mut tmp = String::with_capacity(100);
            let _ = tmp.stream()
                << esc::BOLD
                << "Name:\n"
                << esc::NORMAL
                << "  20 Hits on 2\n"
                << esc::BOLD
                << "Description:\n"
                << esc::NORMAL
                << "  Latest, most lo💖ed radio hits. ";

            let wrapped = utils::word_wrap(wgt.size.width as usize - 2, &tmp);
            *out = wrapped.take().join("\n");
        }
        else {
            let rs = self.rs.lbl.entry(wgt.id).or_default();
            *out = rs.txt.clone();
        }
    }

    fn get_text_edit_text(&mut self, wgt: &Widget, out: &mut String, edit_mode: bool) {
        *out = self.rs.txte.entry(wgt.id).or_default().txt.clone();
    }

    fn get_led_lit(&mut self, wgt: &Widget) -> bool {
        let rs = self.rs.led.entry(wgt.id).or_default();
        rs.lit
    }

    fn get_led_text(&mut self, wgt: &Widget, out: &mut String) {
        *out = "led-text".to_owned();
    }

    fn get_progress_bar_state(&mut self, wgt: &Widget, out: &mut rstate::PgbarState) {
        let rs = self.rs.pgbar.entry(wgt.id).or_default();
        *out = *rs;
    }

    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> i16 {
        let rs = self.rs.pgctrl.entry(wgt.id).or_default();
        rs.page
    }

    fn get_list_box_state(&mut self, wgt: &Widget, out: &mut rstate::LbxState) {
        let rs = self.rs.lbx.entry(wgt.id).or_default();
        *out = *rs;
        out.items_cnt = self.lbx_items.len() as i16
    }

    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: i16, out: &mut String) {
        if wgt.id == id::LIST_BOX {
            let plants = ['🌷', '🌱', '🌲', '🌻'];

            if item_idx == 3 {
                out.push_str(esc::BOLD);
                out.push_str("Item");
                out.push_str(esc::NORMAL);
                out.push_str(" 0034567890123456789*");
            }
            else {
                out.push_str(&format!(
                    "{}Item{} {:03} {}",
                    esc::FG_BLACK,
                    esc::FG_BLUE,
                    item_idx,
                    plants[item_idx as usize & 0x03]
                ));
            }
        }
        else if (item_idx as usize) < self.lbx_items.len() {
            out.push_str(format!("{:02} {}", item_idx, self.lbx_items[item_idx as usize]).as_str());
        }
        else {
            out.push_str("<...>");
        }
    }

    fn get_combo_box_state(&mut self, wgt: &Widget, out: &mut rstate::CbbxState) {
        let rs = self.rs.cbbx.entry(wgt.id).or_default();
        *out = *rs;
        out.items_cnt = self.lbx_items.len() as i16;
    }

    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: i16, out: &mut String) {
        out.push_str(self.lbx_items[item_idx as usize]);
    }

    fn get_radio_index(&mut self, wgt: &Widget) -> i16 {
        self.radiogrp1_idx
    }

    fn get_text_box_state(&mut self, wgt: &Widget, out: &mut rstate::TxtbxState) {
        let rs = self.rs.txtbx.entry(wgt.id).or_default();
        out.top_line = rs.top_line;

        if wgt.id == id::TBX_WIDE {
            if self.tbx_wide_lines.borrow().len() == 0 {
                self.tbx_wide_lines = utils::word_wrap(wgt.size.width as usize - 2, &self.tbx_text);
            }
            out.lines = Arc::clone(&self.tbx_wide_lines);
        }
        else if wgt.id == id::TBX_NARROW {
            if self.tbx_narrow_lines.borrow().len() == 0 {
                self.tbx_narrow_lines =
                    utils::word_wrap(wgt.size.width as usize - 2, &self.tbx_text);
            }
            out.lines = Arc::clone(&self.tbx_narrow_lines);
        }
    }

    fn get_button_text(&mut self, wgt: &Widget, out: &mut String) {
        if wgt.id == id::BTN_TOASTER {
            out.push_str("  🐸  📢  ");
        }
        else if wgt.id == id::BTN_1P5 {
            out.push_str("1.5 🍋 Height");
        }
    }

    /* requests */

    fn instant_redraw(&mut self, wid: WId) {
        if let Some(mut term_guard) = TERM.try_lock() {
            term_guard.draw(self, &[wid]);
            term_guard.flush_buff();
        }
        else {
            rtwins::tr_warn!("Cannot lock the term");
        }
    }

    fn invalidate_many(&mut self, wids: &[WId]) {
        self.invalidated.extend(wids.iter());
    }

    fn clear_invalidated(&mut self) {
        self.invalidated.clear();
    }

    fn get_invalidated(&mut self, out: &mut Vec<WId>) {
        core::mem::swap(&mut self.invalidated, out);
    }
}

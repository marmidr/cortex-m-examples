//! Demo - window state

#![allow(unused_variables)]
#![allow(dead_code)]

use rtwins::common::*;
use rtwins::input;
use rtwins::input::*;
use rtwins::utils;
use rtwins::wgt::{self, WId, Widget, WIDGET_ID_NONE};
use rtwins::TERM;
use rtwins::*;

use super::tui_commands::*;
use super::tui_msgbox_def::idmb;

use core::cell::RefCell;

extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------------------------- //

/// State of all the DemoWindow widget dynamic properties
pub struct MsgBoxState {
    // id of the window
    pub wnd_id: WId,
    /// all window widgets, starting with the window widget itself
    widgets: &'static [wgt::Widget],
    /// widgets runtime state
    pub rs: wgt::RuntimeStates,
    /// currently focused widget
    focused_id: WId,
    /// list of widgets to redraw
    invalidated: Vec<WId>,
    // popup coordinates, centered over main window
    coord: Coord,
    /// button click handler
    on_button: Box<dyn Fn(WId) + Send>,
    /// popup title
    wnd_title: String,
    /// popup message
    wnd_message: String,
    /// visible buttons
    buttons: &'static str,
    // app-wide commands queue
    cmds: Rc<RefCell<CommandsQueue>>,
}

impl MsgBoxState {
    pub fn new(wnd_id: WId, widgets: &'static [Widget], cmds: Rc<RefCell<CommandsQueue>>) -> Self {
        MsgBoxState {
            wnd_id,
            widgets,
            rs: wgt::RuntimeStates::default(),
            focused_id: WIDGET_ID_NONE,
            invalidated: Vec::with_capacity(4),
            coord: Coord::cdeflt(),
            on_button: Box::new(|_id: WId| {}),
            wnd_title: String::new(),
            wnd_message: String::new(),
            buttons: "ynoc",
            cmds,
        }
    }

    pub fn center_on(&mut self, wnd: &Widget) {
        let wndpopup = &self.widgets[0];
        // calc location on the main window center
        self.coord.col = (wnd.size.width - wndpopup.size.width) / 2;
        self.coord.col += wnd.coord.col;
        self.coord.row = (wnd.size.height - wndpopup.size.height) / 2;
        self.coord.row += wnd.coord.row;
    }

    /// Setup the MessageBox befor showing
    ///
    /// buttons: string of 'ynoc' defining visibility of Yes/No/Ok/Cancel buttons
    pub fn setup(
        &mut self,
        title: String,
        message: String,
        buttons: &'static str,
        on_button: Box<dyn Fn(WId) + Send>,
    ) {
        self.wnd_title = title;
        if let Some(lbl) = wgt::find_by_id(self.widgets, idmb::LBL_MSG) {
            self.wnd_message = utils::word_wrap(lbl.size.width as usize, &message)
                .take()
                .join("\n");
        }
        self.buttons = buttons;
        self.on_button = on_button;
    }
}

// -----------------------------------------------------------------------------------------------

impl rtwins::wgt::WindowState for MsgBoxState {
    /** events **/

    fn on_button_click(&mut self, wgt: &Widget, ii: &InputInfo) {
        rtwins::tr_debug!("BTN_CLICK");
        self.on_button.as_ref()(wgt.id);

        match self.cmds.try_borrow_mut() {
            Ok(ref mut cmds) => cmds.push(Command::HidePopup),
            Err(e) => tr_err!("Cannot borrow commands"),
        }
    }

    fn on_button_key(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        false
    }

    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        rtwins::tr_debug!("on_window_unhandled_input_evt={}", ii.name);
        if let InputEvent::Key(input::Key::Esc) = ii.evnt {
            if let Ok(ref mut cmds) = self.cmds.try_borrow_mut() {
                cmds.push(Command::HidePopup);
            }
            return true;
        }

        false
    }

    /** common state queries **/

    fn is_enabled(&self, wgt: &Widget) -> bool {
        self.rs.get_enabled_or_default(wgt.id)
    }

    fn is_focused(&self, wgt: &Widget) -> bool {
        self.focused_id == wgt.id
    }

    fn is_visible(&self, wgt: &Widget) -> bool {
        match wgt.id {
            // TODO: idmb::WND_MSGBOX => return twins::glob::wMngr.topWnd() == this;
            idmb::BTN_YES => self.buttons.as_bytes().contains(&b'y'),
            idmb::BTN_NO => self.buttons.as_bytes().contains(&b'n'),
            idmb::BTN_CANCEL => self.buttons.as_bytes().contains(&b'c'),
            idmb::BTN_OK => self.buttons.as_bytes().contains(&b'o'),
            _ => true,
        }
    }

    fn get_focused_id(&mut self) -> WId {
        self.focused_id
    }

    fn set_focused_id(&mut self, wid: WId) {
        self.focused_id = wid;
    }

    fn get_widgets(&self) -> &'static [Widget] {
        self.widgets
    }

    fn get_rstate(&mut self) -> Option<&mut wgt::RuntimeStates> {
        Some(&mut self.rs)
    }

    /** widget-specific queries; all mutable params are outputs **/

    fn get_window_coord(&mut self) -> Coord {
        self.coord
    }

    fn get_window_size(&mut self) -> Size {
        self.widgets.first().unwrap().size
    }

    fn get_window_title(&mut self, wgt: &Widget, out: &mut String) {
        out.push_str(&self.wnd_title)
    }

    fn get_label_text(&mut self, wgt: &Widget, out: &mut String) {
        if wgt.id == idmb::LBL_MSG {
            // let wrapped = utils::word_wrap(wgt.size.width as usize, &self.wnd_message);
            // txt.push_str(wrapped.take().join("\n").as_str());
            out.push_str(&self.wnd_message);
        }
    }

    /* requests */

    fn invalidate_many(&mut self, wids: &[WId]) {
        // iterate, to avoid adding the same ID twice
        for wid in wids.iter() {
            if !self.invalidated.contains(wid) {
                self.invalidated.push(*wid);
            }
            else {
                rtwins::tr_warn!("Invalidated again: id::{}", wid);
            }
        }
    }

    fn instant_redraw(&mut self, wid: WId) {
        if let Some(mut term_guard) = TERM.try_lock() {
            term_guard.draw(self, &[wid]);
            term_guard.flush_buff();
        }
        else {
            rtwins::tr_warn!("Cannot lock the term");
        }
    }

    fn clear_invalidated(&mut self) {
        self.invalidated.clear();
    }

    fn get_invalidated(&mut self, out: &mut Vec<WId>) {
        core::mem::swap(&mut self.invalidated, out);
    }
}

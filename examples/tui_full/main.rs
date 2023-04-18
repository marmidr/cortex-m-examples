//! # RTWins full demo app

#![cfg_attr(target_os = "none", no_main)]
#![cfg_attr(target_os = "none", no_std)]

use rtwins::wgt::{WId, WindowState};
use rtwins::wnd_manager::WindowManager;
use rtwins::{tetrary, wgt, TERM};

// use core::prelude::rust_2021::*;
use core::cell::RefCell;

extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::format;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;

use crate::tui_commands::Command;
use crate::tui_main_def::id;

// https://doc.rust-lang.org/cargo/guide/project-layout.html
mod tui_colors;
mod tui_commands;
mod tui_main_def;
mod tui_main_state;
mod tui_msgbox_def;
mod tui_msgbox_state;

#[cfg(target_os = "linux")]
mod input_libc_tty;
#[cfg(target_os = "linux")]
mod pal_std;

#[cfg(target_os = "none")]
mod pal_semihosting;
#[cfg(target_os = "none")]
use alloc_cortex_m::CortexMHeap;
#[cfg(target_os = "none")]
use cortex_m_rt::entry;
#[cfg(target_os = "none")]
use cortex_m_semihosting::debug;
#[cfg(target_os = "none")]
use panic_semihosting as _;

// ---------------------------------------------------------------------------------------------- //

struct WndMngr {
    cmdque: Rc<RefCell<tui_commands::CommandsQueue>>,
    visible: Vec<WId>,
    main: tui_main_state::MainWndState,
    msgbox: tui_msgbox_state::MsgBoxState,
}

impl WndMngr {
    // rtwins::generate_ids!() - starts from 1, but Window ids are indexes and should start from 0
    pub const MAIN: rtwins::wgt::WId = 0;
    pub const MSGBOX: rtwins::wgt::WId = 1;

    fn new() -> Self {
        let cmdque = Rc::new(RefCell::new(tui_commands::CommandsQueue::default()));

        let mut ret = Self {
            cmdque: Rc::clone(&cmdque),
            visible: vec![],
            // all UI windows:
            main: tui_main_state::MainWndState::new(
                Self::MAIN,
                &tui_main_def::WND_MAIN_WGTS[..],
                Rc::clone(&cmdque),
            ),
            msgbox: tui_msgbox_state::MsgBoxState::new(
                Self::MSGBOX,
                &tui_msgbox_def::WND_MSGBOX_WGTS[..],
                Rc::clone(&cmdque),
            ),
        };

        ret.msgbox
            .center_on(ret.main.get_widgets().first().unwrap());
        ret
    }
}

/*
/// Iterator over manager windows
struct WndMngrIter<'a> {
    wm: &'a WndMngr,
    iter_idx: usize,
}

impl<'a> WndMngrIter<'a> {
    fn new(wm: &'a WndMngr) -> Self {
        WndMngrIter{
            wm,
            iter_idx: 0
        }
    }
}

impl<'a> Iterator for WndMngrIter<'a> {
    type Item = &'a dyn WindowState;

    fn next(&mut self) -> Option<Self::Item> {
        let result= match self.iter_idx {
            0 => Some(&self.wm.main as &dyn WindowState),
            1 => Some(&self.wm.msgbox as &dyn WindowState),
            _ => None,
        };

        self.iter_idx += 1;
        result
    }
}
*/

// impl<'a> WindowManager<WndMngrIter<'a>> for WndMngr {
impl WindowManager for WndMngr {
    fn get_ref(&self, wnd_id: WId) -> Option<&dyn WindowState> {
        match wnd_id {
            WndMngr::MAIN => Some(&self.main),
            WndMngr::MSGBOX => Some(&self.msgbox),
            _ => None,
        }
    }

    fn get_mut(&mut self, wnd_id: WId) -> Option<&mut dyn WindowState> {
        match wnd_id {
            WndMngr::MAIN => Some(&mut self.main),
            WndMngr::MSGBOX => Some(&mut self.msgbox),
            _ => None,
        }
    }

    #[inline]
    fn get_visible(&self) -> &[WId] {
        &self.visible[..]
    }

    #[inline]
    fn get_visible_mut(&mut self) -> &mut Vec<WId> {
        &mut self.visible
    }

    // fn iter(&self) -> WndMngrIter<'a> {
    //     WndMngrIter::new(self)
    // }
}

// ---------------------------------------------------------------------------------------------- //

// this is the allocator the application will use
#[cfg(target_os = "none")]
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cfg(target_os = "linux")]
fn main() {
    tui();
}

#[cfg(target_os = "none")]
#[entry]
fn main() -> ! {
    tui();

    if cfg!(feature = "qemu") {
        // exit QEMU
        // NOTE do not run this on hardware; it can corrupt OpenOCD state
        debug::exit(debug::EXIT_SUCCESS);
    }

    loop {}
}

fn tui() {
    // Initialize the allocator BEFORE you use it
    #[cfg(target_os = "none")]
    unsafe {
        const HEAP_SIZE: usize = 1024 * 20; // in bytes
        ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE);
    }

    #[cfg(target_os = "none")]
    let cp = cortex_m::Peripherals::take().unwrap();

    #[cfg(target_os = "none")]
    {
        let delay = cortex_m::delay::Delay::new(cp.SYST, 32_000_000);
        TERM.try_lock().unwrap().pal = Box::new(pal_semihosting::SemihostingPal::new(delay));
    }

    #[cfg(target_os = "linux")]
    {
        // replace default PAL with our own:
        TERM.try_lock().unwrap().pal = Box::new(pal_std::DemoPal::new());

        // register function providing traces timestamp
        rtwins::tr_set_timestr_function!(|| {
            let local_time = chrono::Local::now();
            local_time.format("%H:%M:%S%.3f ").to_string()
        });
    }

    // window manager and all windows
    let mut wmngr = WndMngr::new();

    // configure terminal
    if let Some(mut term_guard) = TERM.try_lock() {
        term_guard.trace_row = {
            let coord = wmngr.main.get_window_coord();
            let sz = wmngr.main.get_window_size();
            coord.row as u16 + sz.height as u16 + 1
            // 1
        };
        term_guard.write_str(rtwins::esc::TERM_RESET);
        term_guard.mouse_mode(rtwins::MouseMode::M2);
    }

    tui_colors::init();
    // first draw of the UI
    wmngr.show(WndMngr::MAIN);

    rtwins::tr_info!("Press Ctrl-D to quit");
    rtwins::tr_info!(
        "Size of WND_MAIN_WGTS: {} B",
        core::mem::size_of_val(&tui_main_def::WND_MAIN_WGTS)
    );
    if cfg!(feature = "qemu") {
        rtwins::tr_info!(
            "{}Running from QEMU{}",
            rtwins::esc::FG_BLUE_VIOLET,
            rtwins::esc::FG_DEFAULT
        );
    }
    // rtwins::tr_warn!("WARN MACRO 1");
    // rtwins::tr_err!("ERR MACRO 1");
    // rtwins::tr_debug!("DEBUG MACRO: X={} N={}", 42, "Warduna");
    rtwins::tr_flush!(&mut TERM.try_lock().unwrap());

    #[cfg(target_os = "linux")]
    let mut inp = {
        let tty_path = {
            // type `tty` in separate terminal, to get it's number
            let path_opt = std::env::args().find(|a| a.starts_with("--tty=")).map(|tty|
                // --tty=/dev/pts/10
                // --tty=10
                tty.split_once('=')
                .unwrap_or_default().1.to_owned());

            if let Some(ref p) = path_opt {
                rtwins::tr_info!("Input TTY: {}", p);
            }

            path_opt
        };

        input_libc_tty::InputTty::new(tty_path, 1000)
    };
    #[cfg(target_os = "none")]
    let mut inp = pal_semihosting::InputSemiHost::new();

    let mut ique = rtwins::input_decoder::InputQue::new();
    let mut dec = rtwins::input_decoder::Decoder::default();
    let mut ii = rtwins::input::InputInfo::default();
    let mut mouse_on = true;

    #[allow(unused_labels)]
    'mainloop: loop {
        let (inp_seq, q) = inp.read_input();
        // TODO: detect that application was sent to background and restore terminal config

        if q {
            rtwins::tr_warn!("Exit requested");
            break;
        }
        else if !inp_seq.is_empty() {
            ique.extend(inp_seq.iter());

            while dec.decode_input_seq(&mut ique, &mut ii) > 0 {
                // check for Ctrl+D
                #[cfg(target_os = "none")]
                if let InputEvent::Char(ref cb) = ii.evnt {
                    if cb.as_str() == "D" && ii.kmod.has_ctrl() {
                        rtwins::tr_warn!("Exit requested");
                        break 'mainloop;
                    }
                }

                use rtwins::input::InputEvent;
                use rtwins::input::Key;

                // pass the input event to the top-window
                let _key_handled = wmngr
                    .get_top_mut()
                    .map_or(false, |ws| wgt::process_input(ws, &ii));

                // input debug info
                match ii.evnt {
                    InputEvent::Char(ref ch) => {
                        rtwins::tr_debug!("char='{}'", ch.as_str());
                    }
                    InputEvent::Key(ref _k) => {
                        rtwins::tr_debug!("key={}", ii.name);
                    }
                    InputEvent::Mouse(ref m) => {
                        let mut r = rtwins::Rect::cdeflt();
                        let wgt_opt = wmngr
                            .get_top_mut()
                            .map_or(None, |ws| wgt::find_at(ws, m.col, m.row, &mut r));

                        if let Some(w) = wgt_opt {
                            rtwins::tr_debug!(
                                "mouse={:?} at {}:{} ({})",
                                m.evt,
                                m.col,
                                m.row,
                                w.prop
                            );
                        }
                        else {
                            rtwins::tr_debug!("mouse={:?} at {}:{}", m.evt, m.col, m.row);
                        }
                    }
                    InputEvent::None => {}
                }

                // input processing
                if let InputEvent::Key(ref key) = ii.evnt {
                    if *key == Key::F2 {
                        if let Some(top_ws) = wmngr.get_top_mut() {
                            let en = !top_ws.is_enabled(&top_ws.get_widgets()[0]);

                            top_ws
                                .get_rstate()
                                .unwrap()
                                .set_enabled(tui_main_def::id::WND_MAIN, en);
                            top_ws.invalidate(wgt::WIDGET_ID_ALL);
                        }
                    }
                    else if *key == Key::F4 {
                        mouse_on = !mouse_on;
                        rtwins::tr_info!("Mouse {}", if mouse_on { "ON" } else { "OFF" });
                        let mut term_guard = TERM.try_lock().unwrap();
                        term_guard.mouse_mode(tetrary!(
                            mouse_on,
                            rtwins::MouseMode::M2,
                            rtwins::MouseMode::Off
                        ));
                        term_guard.flush_buff();
                    }
                    else if *key == Key::F5 {
                        TERM.try_lock().unwrap().screen_clr_all();
                        // draw windows from bottom to top
                        wmngr.draw_all();
                    }
                    else if *key == Key::F6 {
                        let mut term_guard = TERM.try_lock().unwrap();
                        term_guard.trace_area_clear();
                    }
                    else if ii.kmod.has_ctrl() && (*key == Key::PgUp || *key == Key::PgDown) {
                        if wmngr.is_top(WndMngr::MAIN) {
                            if let Some(main_ws) = wmngr.get_top_mut() {
                                wgt::pagectrl_select_next_page(
                                    main_ws,
                                    tui_main_def::id::PG_CONTROL,
                                    *key == Key::PgDown,
                                );
                            }
                        }
                    }
                    else if wmngr.is_top(WndMngr::MAIN) && (*key == Key::F9 || *key == Key::F10) {
                        if let Some(main_ws) = wmngr.get_top_mut() {
                            wgt::pagectrl_select_next_page(
                                main_ws,
                                tui_main_def::id::PG_CONTROL,
                                *key == Key::F10,
                            );
                        }
                    }
                }

                if wmngr.is_top(WndMngr::MAIN) {
                    wmngr.main.rs.lbl.entry(id::LABEL_INPSEQ).or_default().txt =
                        rtwins::input_decoder::inp_seq_debug(inp_seq);
                    wmngr.main.rs.lbl.entry(id::LABEL_INPNAME).or_default().txt =
                        ii.name.to_owned();
                    wmngr
                        .main
                        .invalidate_many(&[id::LABEL_INPNAME, id::LABEL_INPSEQ]);
                }

                // process the command queue
                {
                    let cmdque = wmngr.cmdque.borrow_mut().take_commands();

                    if let Some(cmdque) = cmdque {
                        for cmd in cmdque.into_iter() {
                            match cmd {
                                Command::ShowPopup {
                                    title,
                                    message,
                                    buttons,
                                    on_button,
                                } => {
                                    rtwins::tr_info!("Command: ShowPopup");
                                    wmngr.msgbox.setup(title, message, buttons, on_button);
                                    wmngr.show(WndMngr::MSGBOX);
                                }
                                Command::HidePopup => {
                                    rtwins::tr_info!("Command: HidePopup");
                                    wmngr.hide(WndMngr::MSGBOX);
                                }
                            }
                        }
                    }
                }

                wmngr.draw_top_invalidated();

                // wait for a key
                if cfg!(feature = "qemu") {
                    let mut term_guard = TERM.try_lock().unwrap();
                    term_guard.pal.as_mut().sleep(50);
                }
            } // decode_input_seq
        }

        // flush the trace logs on every loop
        rtwins::tr_flush!(&mut TERM.try_lock().unwrap());
    }

    // epilogue
    {
        let mut term_guard = TERM.try_lock().unwrap();
        term_guard.mouse_mode(rtwins::MouseMode::Off);
        rtwins::tr_flush!(&mut term_guard);

        term_guard.pal.as_mut().sleep(1_000);
        // clear logs below the cursor
        term_guard.trace_area_clear();

        // set the cursor on the expected position
        let logs_row = term_guard.trace_row;
        term_guard.move_to(0, logs_row);
        term_guard.flush_buff();
    }
}

// ---------------------------------------------------------------------------------------------- //

#[test]
fn test_esc_codes() {
    use rtwins::esc;

    println!(
        "** {}{}{} ** demo; lib v{}{}{}",
        esc::BOLD,
        rtwins::url_link!("https://github.com/marmidr/rtwins", "RTWins"),
        esc::NORMAL,
        esc::FG_HOT_PINK,
        rtwins::VER,
        esc::FG_DEFAULT
    );

    println!(
        "{}Faint{} {}Bold{} {}Italic{}",
        esc::FAINT,
        esc::NORMAL,
        esc::BOLD,
        esc::NORMAL,
        esc::ITALICS_ON,
        esc::ITALICS_OFF
    );
}

#[test]
fn test_property_access() {
    for (idx, w) in tui_main_def::WND_MAIN_WGTS.iter().enumerate() {
        let parent = wgt::get_parent(w);
        println!(
            "  {:2}. {:2}:{:10}, idx:{}, chidx:{}, parid {}:{}",
            idx, w.id, w.prop, w.link.own_idx, w.link.children_idx, parent.id, parent.prop
        );
    }

    println!("sizeof Widget: {}", core::mem::size_of::<wgt::Widget>());
    println!("sizeof Property: {}", core::mem::size_of::<wgt::Property>());
}

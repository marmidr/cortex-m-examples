//! # RTWins minimal demo app

#![no_main]
#![no_std]

use rtwins::colors::{ColorBg, ColorFg};
use rtwins::common::*;
use rtwins::input::*;
use rtwins::wgt;
use rtwins::wgt::prop;
use rtwins::wgt::*;
use rtwins::TERM;

extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

// use panic_halt as _;
use alloc_cortex_m::CortexMHeap;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprint};
use panic_semihosting as _;
use try_lock::TryLock;

// ---------------------------------------------------------------------------------------------- //

struct DemoMiniPal {
    line_buff: String,
    delay: TryLock<cortex_m::delay::Delay>,
}

impl DemoMiniPal {
    fn new(d: cortex_m::delay::Delay) -> Self {
        DemoMiniPal {
            line_buff: String::with_capacity(100),
            delay: TryLock::new(d),
        }
    }
}

impl rtwins::pal::Pal for DemoMiniPal {
    fn write_char_n(&mut self, c: char, repeat: i16) {
        for _ in 0..repeat {
            self.line_buff.push(c);
        }
    }

    fn write_str_n(&mut self, s: &str, repeat: i16) {
        self.line_buff.reserve(s.len() * repeat as usize);

        for _ in 0..repeat {
            self.line_buff.push_str(s);
        }
    }

    fn flush_buff(&mut self) {
        // if let Ok(ref mut out) = cortex_m_semihosting::hio::hstdout() {
        //     let _ = out.write_all(self.line_buff.as_bytes());
        // }

        hprint!("{}", self.line_buff);
        self.line_buff.clear();
        self.sleep(1);
    }

    fn sleep(&self, ms: u16) {
        if let Some(mut d) = self.delay.try_lock() {
            d.delay_ms(ms as u32);
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

mod id {
    use rtwins::wgt::{WId, WIDGET_ID_NONE};

    #[rustfmt::skip]
    rtwins::generate_ids!(
        WND_MAIN
            LBL_TITLE
            BTN_OK
            BTN_CANCEL
    );
}

#[rustfmt::skip]
const WINDOW_MAIN: Widget = Widget {
    id: id::WND_MAIN,
    link: Link::cdeflt(),
    coord: Coord { col: 10, row: 2 },
    size: Size { width: 40, height: 8 },
    prop: prop::Window {
        title: concat!(
            "Demo mini ",
            rtwins::underline_on!(),
                "(Ctrl+D to quit)",
            rtwins::underline_off!()
        ),
        fg_color: ColorFg::White,
        bg_color: ColorBg::Blue,
        is_popup: false,
    }.into(),
    children: &[
        Widget {
            id: id::LBL_TITLE,
            coord: Coord { col: 6, row: 2 },
            prop: prop::Label {
                title: concat!(
                    rtwins::inverse_on!(),
                        "Minimalistic RTWins TUI demo",
                    rtwins::inverse_off!()
                ),
                fg_color: ColorFg::White,
                bg_color: ColorBg::GreenIntense,
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: id::BTN_OK,
            coord: Coord { col: 10, row: 5 },
            prop: prop::Button {
                text: " OK ",
                fg_color: ColorFg::Green,
                bg_color: ColorBg::Black,
                style: ButtonStyle::Solid
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: id::BTN_CANCEL,
            coord: Coord { col: 22, row: 5 },
            prop: prop::Button {
                text: "Cancel",
                fg_color: ColorFg::RedIntense,
                bg_color: ColorBg::Black,
                style: ButtonStyle::Solid
            }.into(),
            ..Widget::cdeflt()
        },
    ],
};

const WND_MAIN_WGTS: [Widget; transform::tree_wgt_count(&WINDOW_MAIN)] =
    transform::tree_to_array(&WINDOW_MAIN);

// ---------------------------------------------------------------------------------------------- //

struct MainWndState {
    focused_id: WId,
    invalidated: Vec<WId>,
}

impl Default for MainWndState {
    fn default() -> Self {
        MainWndState {
            focused_id: WIDGET_ID_NONE,
            invalidated: vec![],
        }
    }
}

impl rtwins::wgt::WindowState for MainWndState {
    fn on_button_click(&mut self, wgt: &Widget, _ii: &InputInfo) {
        match wgt.id {
            id::BTN_OK => rtwins::tr_info!("OK clicked"),
            id::BTN_CANCEL => rtwins::tr_info!("Cancel clicked"),
            other => rtwins::tr_warn!("Unknown button clicked (id:{})", other),
        }
    }

    fn is_focused(&self, wgt: &Widget) -> bool {
        self.focused_id == wgt.id
    }

    fn get_focused_id(&mut self) -> WId {
        self.focused_id
    }

    fn set_focused_id(&mut self, wid: WId) {
        self.focused_id = wid;
    }

    fn get_widgets(&self) -> &'static [Widget] {
        &WND_MAIN_WGTS
    }

    fn get_window_coord(&mut self) -> Coord {
        WND_MAIN_WGTS.first().map_or(Coord::cdeflt(), |w| w.coord)
    }

    fn get_window_size(&mut self) -> Size {
        WND_MAIN_WGTS.first().map_or(Size::cdeflt(), |w| w.size)
    }

    fn instant_redraw(&mut self, wid: WId) {
        if let Some(mut term_guard) = TERM.try_lock() {
            term_guard.draw(self, &[wid]);
            term_guard.flush_buff();
        } else {
            rtwins::tr_warn!("Cannot lock the term");
        }
    }

    fn invalidate_many(&mut self, wids: &[WId]) {
        self.invalidated.extend(wids.iter());
    }

    fn clear_invalidated(&mut self) {
        self.invalidated.clear();
    }

    fn take_invalidated(&mut self) -> Vec<WId> {
        let mut ret = Vec::with_capacity(4);
        core::mem::swap(&mut self.invalidated, &mut ret);
        ret
    }
}

// ---------------------------------------------------------------------------------------------- //

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    const HEAP_SIZE: usize = 1024 * 2; // in bytes
                                       // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // start the TUI interface
    tui();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[allow(dead_code)]
pub struct InputSemiHost {
    input_timeout_ms: u16,
    input_buff: [u8; rtwins::esc::SEQ_MAX_LENGTH],
    input_len: u8,
}

impl InputSemiHost {
    /// Createas new TTY input reader with given timeout in [ms];
    /// the timeout applies when calling `read_input()`
    pub fn new(timeout_ms: u16) -> Self {
        InputSemiHost {
            input_timeout_ms: timeout_ms,
            input_buff: [0u8; rtwins::esc::SEQ_MAX_LENGTH],
            input_len: 0,
        }
    }

    /// Returns tuple with ESC sequence slice and bool marker set to true,
    /// if application termination was requested (C-d)
    pub fn read_input(&mut self) -> (&[u8], bool) {
        (&self.input_buff[..0], false)
    }
}

fn tui() {
    let cp = cortex_m::Peripherals::take().unwrap();

    // replace default PAL with our own:
    {
        let delay = cortex_m::delay::Delay::new(cp.SYST, 20_000_000);
        let pal = Box::new(DemoMiniPal::new(delay));
        TERM.try_lock().unwrap().pal = pal;
    }

    // create window state:
    let mut ws_main = MainWndState::default();

    // configure terminal
    if let Some(mut term_guard) = TERM.try_lock() {
        term_guard.trace_row = {
            let coord = ws_main.get_window_coord();
            let sz = ws_main.get_window_size();
            coord.row as u16 + sz.height as u16 + 1
        };
        term_guard.write_str(rtwins::esc::TERM_RESET);
        term_guard.mouse_mode(rtwins::MouseMode::M2);
    }

    TERM.try_lock().unwrap().draw_wnd(&mut ws_main);
    rtwins::tr_info!("Press Ctrl-D to quit");
    if cfg!(feature = "qemu") {
        rtwins::tr_info!(
            "{}Running from QEMU{}",
            rtwins::esc::FG_BLUE_VIOLET,
            rtwins::esc::FG_DEFAULT
        );
    }
    rtwins::tr_flush!(&mut TERM.try_lock().unwrap());

    TERM.try_lock().unwrap().pal.as_mut().sleep(2_000);

    let mut inp = InputSemiHost::new(10);
    let mut ique = rtwins::input_decoder::InputQue::new();
    let mut dec = rtwins::input_decoder::Decoder::default();
    let mut ii = rtwins::input::InputInfo::default();

    // main loop
    loop {
        let (inp_seq, q) = inp.read_input();

        if q {
            rtwins::tr_info!("Exit requested");
            break;
        } else if !inp_seq.is_empty() {
            ique.extend(inp_seq.iter());

            while dec.decode_input_seq(&mut ique, &mut ii) > 0 {
                let _key_handled = wgt::process_input(&mut ws_main, &ii);
            }
        }

        TERM.try_lock().unwrap().draw_invalidated(&mut ws_main);
        rtwins::tr_flush!(&mut TERM.try_lock().unwrap());

        if cfg!(feature = "qemu") {
            // exit the loop for emulated run
            break;
        }
    }

    // epilogue
    {
        let mut term_guard = TERM.try_lock().unwrap();
        rtwins::tr_flush!(&mut term_guard);
        term_guard.mouse_mode(rtwins::MouseMode::Off);
        term_guard.trace_area_clear();

        // clear logs below the cursor
        let logs_row = term_guard.trace_row;
        term_guard.move_to(0, logs_row);
        term_guard.flush_buff();
    }
}

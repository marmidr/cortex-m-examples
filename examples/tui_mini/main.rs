//! # RTWins minimal demo app

#![no_main]
#![no_std]

use rtwins::colors::{ColorBg, ColorFg};
use rtwins::common::*;
use rtwins::esc;
use rtwins::input::*;
use rtwins::wgt;
use rtwins::wgt::prop;
use rtwins::wgt::*;
use rtwins::TERM;

extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

use alloc_cortex_m::CortexMHeap;
// use embedded_alloc::Heap; // TODO: linker error when used
use cortex_m_rt::entry;
use cortex_m_semihosting::debug;
// use cortex_m_semihosting::hprint;
use panic_semihosting as _;
// use try_lock::TryLock;

#[cfg(not(any(windows, unix)))]
mod pal_semihosting;

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

    fn get_invalidated(&mut self, out: &mut Vec<WId>) {
        core::mem::swap(&mut self.invalidated, out);
    }
}

// ---------------------------------------------------------------------------------------------- //

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
// static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe {
        const HEAP_SIZE: usize = 1024 * 2; // in bytes
        ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE);
    }

    // {
    //     use core::mem::MaybeUninit;
    //     const HEAP_SIZE: usize = 1024 * 2;
    //     static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    //     unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE); }
    // }

    // start the TUI interface
    tui();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

fn tui() {
    let cp = cortex_m::Peripherals::take().unwrap();

    // replace default PAL with our own:
    {
        let delay = cortex_m::delay::Delay::new(cp.SYST, 32_000_000);
        let pal = Box::new(pal_semihosting::SemihostingPal::new(delay));
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
        term_guard.write_str(esc::TERM_RESET);
        term_guard.mouse_mode(rtwins::MouseMode::M2);
        term_guard.draw_wnd(&mut ws_main);
    }
    else {
        panic!("Could not lock the TERM");
    }

    rtwins::tr_info!("Press Ctrl-D to quit");
    rtwins::tr_info!("Size of WND_MAIN_WGTS: {} B", core::mem::size_of_val(&WND_MAIN_WGTS));

    if cfg!(feature = "qemu") {
        rtwins::tr_info!(
            "{}Running from QEMU{}",
            esc::FG_BLUE_VIOLET,
            esc::FG_DEFAULT
        );
    }
    rtwins::tr_flush!(&mut TERM.try_lock().unwrap());

    let mut inp = pal_semihosting::InputSemiHost::new();
    let mut ique = rtwins::input_decoder::InputQue::default();
    let mut dec = rtwins::input_decoder::Decoder::default();
    let mut ii = rtwins::input::InputInfo::default();

    'mainloop: loop  {
        let inp_seq = inp.read_input().0;

        if !inp_seq.is_empty() {
            ique.extend(inp_seq.iter());

            while dec.decode_input_seq(&mut ique, &mut ii) > 0 {
                // check for Ctrl+D
                if let InputEvent::Char(ref cb) = ii.evnt {
                    if cb.as_str() == "D" && ii.kmod.has_ctrl() {
                        rtwins::tr_info!("Exit requested");
                        break 'mainloop;
                    }
                }

                rtwins::tr_debug!("Input: {}{}{}, bytes: {:?}",
                    esc::BOLD, ii.name, esc::NORMAL, inp_seq);
                let _key_handled = wgt::process_input(&mut ws_main, &ii);
            }
        }

        {
            let mut term_guard = TERM.try_lock().unwrap();
            term_guard.draw_invalidated(&mut ws_main);
            rtwins::tr_flush!(&mut term_guard);

            // wait for a key
            if cfg!(feature = "qemu") {
                term_guard.pal.as_mut().sleep(50);
            }
        }
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

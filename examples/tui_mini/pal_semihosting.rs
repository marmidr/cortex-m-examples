//! # RTWins PAL for Cortex-M

use try_lock::TryLock;

// use core::prelude::rust_2021::*;

extern crate alloc;
use alloc::string::String;

// ---------------------------------------------------------------------------------------------- //

pub struct SemihostingPal {
    line_buff: String,
    delay: TryLock<cortex_m::delay::Delay>,
}

impl SemihostingPal {
    pub fn new(d: cortex_m::delay::Delay) -> Self {
        SemihostingPal {
            line_buff: String::with_capacity(100),
            delay: TryLock::new(d),
        }
    }
}

impl rtwins::pal::Pal for SemihostingPal {
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

        if self.line_buff.len() > 50 {
            self.flush_buff();
        }
    }

    fn flush_buff(&mut self) {
        // hprint!("{}", self.line_buff);

        if let Ok(ref mut out) = cortex_m_semihosting::hio::hstdout() {
            let _ = out.write_all(self.line_buff.as_bytes());
        }

        self.line_buff.clear();
        // self.sleep(50);
    }

    fn sleep(&self, ms: u16) {
        if let Some(mut d) = self.delay.try_lock() {
            d.delay_ms(ms as u32);
        }
    }
}

// #[allow(dead_code)]
pub struct InputSemiHost {
    input_buff: [u8; rtwins::esc::SEQ_MAX_LENGTH],
    input_len: usize,
    stdin_fd: isize
}

impl InputSemiHost {
    /// Createas a new Cortex-M semihosting input reader
    pub fn new() -> Self {
        let stdin_fd = unsafe {
            cortex_m_semihosting::syscall!(OPEN, ":tt\0".as_ptr(),
                cortex_m_semihosting::nr::open::R, 3) as isize
        };

        if stdin_fd == -1 {
            rtwins::tr_err!("Unable to open stdin");
        }

        InputSemiHost {
            input_buff: [0u8; rtwins::esc::SEQ_MAX_LENGTH],
            input_len: 0,
            stdin_fd
        }
    }

    /// Returns tuple with ESC sequence slice;
    /// bool value is here unused
    pub fn read_input(&mut self) -> (&[u8], bool) {
        self.input_len = self.hstdin();

        if self.input_len != 0 {
            (&self.input_buff[..self.input_len as usize], false)
        }
        else {
            (&[], false)
        }
    }

    fn hstdin(&self) -> usize {
        // TODO: for ~3 seconds after start reads nothing
        let rc = unsafe {
            // https://developer.arm.com/documentation/dui0471/e/semihosting/sys-read--0x06-
            // READC - not implemented
            let rc = cortex_m_semihosting::syscall!(READ,
                self.stdin_fd, self.input_buff.as_ptr(), self.input_buff.len());
            // 8 -> 0 bytes read
            // 5 -> 3 bytes read
            rc
        };

        // returns number of bytes read
        self.input_buff.len() - rc
    }
}

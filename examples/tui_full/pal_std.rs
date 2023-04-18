//! # RTWins demo PAL

use std::io::Write;

// ---------------------------------------------------------------------------------------------- //

pub struct DemoPal {
    line_buff: String,
    writing_logs: bool,
    started_at: std::time::Instant,
}

impl DemoPal {
    pub fn new() -> Self {
        DemoPal {
            line_buff: String::with_capacity(500),
            writing_logs: false,
            started_at: std::time::Instant::now(),
        }
    }
}

impl rtwins::pal::Pal for DemoPal {
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
        std::io::stdout()
            .lock()
            .write_all(self.line_buff.as_bytes())
            .expect("Error writing to stdout");

        std::io::stdout()
            .lock()
            .flush()
            .expect("Error flushing stdout");

        self.line_buff.clear();

        #[cfg(feature = "slow_flush")]
        self.sleep(50); // helpful when debugging drawing process
    }

    fn mark_logging(&mut self, active: bool) {
        if self.writing_logs && !active {
            // write to logs.txt
        }

        self.writing_logs = active;
    }

    fn sleep(&self, ms: u16) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }

    fn get_timestamp_ms(&self) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32
    }

    fn get_timespan_ms(&self, prev_timestamp: u32) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32 - prev_timestamp
    }
}

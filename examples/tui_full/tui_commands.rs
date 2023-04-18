//! Demo - commands queue

use rtwins::wgt::WId;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------------------------- //

pub enum Command {
    ShowPopup {
        title: String,
        message: String,
        buttons: &'static str,
        on_button: Box<dyn Fn(WId) + Send>,
    },
    HidePopup,
}

/// Deferred commands
#[derive(Default)]
pub struct CommandsQueue {
    commands: Vec<Command>,
}

impl CommandsQueue {
    /// Pushes a new command on the queue
    pub fn push(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    /// Take out the commands queue, return Some if non-empty
    pub fn take_commands(&mut self) -> Option<Vec<Command>> {
        if self.commands.is_empty() {
            None
        }
        else {
            Some(core::mem::take(&mut self.commands))
        }
    }

    /// Number of commands waiting to be run
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.commands.len()
    }
}

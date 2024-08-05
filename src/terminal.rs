use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{stdout, Stdout};

pub struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn prompt_loop(&self) {
        loop {}
    }
}

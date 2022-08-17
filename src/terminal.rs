use crate::Position;
use log::LevelFilter;

use std::io::{self, stdout, Error, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let size = termion::terminal_size()?;
        let _ = simple_logging::log_to_file("debug.log", LevelFilter::Trace);
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        println!("{}", termion::cursor::Goto(x, y));
        //log::debug!("Cursor after executing cursor_position is at ({},{})", x, y);
    }
    pub fn clear_screen() {
        println!("{}", termion::clear::All);
    }
    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }
    pub fn cursor_show() {
        println!("{}", termion::cursor::Show);
    }
}

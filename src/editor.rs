use crate::Document;
use crate::Row;
use crate::Terminal;
use std::{env, io::Error};
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    frame_counter: u32,
}

#[derive(Default, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn print_state(&self) {
        let state = format!(
            "cursor_position : {:?} offset : {:?} frame_counter : {} term_width: {} , term_height: {}",
            self.cursor_position, self.offset, self.frame_counter, self.terminal.size().width, self.terminal.size().height
        );
        log::trace!("Current State {} ", state);
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
            if self.should_quit {
                Terminal::clear_screen();
                Terminal::cursor_position(&Position::default());
                println!("GoodBye.\r");
                break;
            }
            self.frame_counter += 1;
            self.print_state();
        }
    }
    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        self.frame_counter += 1;

        if self.should_quit {
            return Ok(());
        }
        self.draw_rows();
        Terminal::cursor_position(&Position {
            x: self.cursor_position.x.saturating_add(self.offset.x),
            y: self.cursor_position.y.saturating_add(self.offset.y),
        });

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            terminal: Terminal::default().expect("Failed to initialize the terminal"),
            should_quit: false,
            cursor_position: Position { x: 0, y: 0 },
            document,
            frame_counter: 0,
            offset: Position::default(),
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.terminal.size().height.saturating_sub(1) as usize;
        let width = self.terminal.size().width.saturating_sub(1) as usize;
        //log::trace!("Current cursor position {:?}", self.cursor_position);
        match key {
            Key::Up => {
                if y > 0 {
                    y = y.saturating_sub(1);
                } else {
                    self.offset.y = self.offset.y.saturating_sub(1);
                }
            }
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                    //log::trace!("matched second block");
                } else {
                    self.offset.y = self.offset.y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageDown => y = height,
            Key::PageUp => y = 0,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        };
        self.cursor_position = Position { x, y };
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        //self.scroll();
        Ok(())
    }

    fn draw_row(&self, row: &Row, row_num: u16) {
        let start = self.offset.x;
        let end = self.offset.x + self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}. {}\r", row_num, row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            if let Some(row) = self
                .document
                .row(terminal_row as usize + self.offset.y as usize)
            {
                self.draw_row(row, terminal_row + 1);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hector editor --version{}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
}

fn die(error: &Error) {
    Terminal::clear_screen();
    panic!("{}", error);
}

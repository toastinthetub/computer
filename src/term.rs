use crossterm::{
    event::{self, KeyEvent, MouseEvent},
    execute,
    terminal::{self, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnterAlternateScreen},
    ExecutableCommand,
};
use std::{
    error::Error,
    io::{self, Stdout, Write},
    sync::{Arc, Mutex},
};
use tokio::time::{sleep, Duration};

use crate::utils::logger;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub size: (u16, u16),
    pub response_domain: ResponseDomain,
    pub prompt_domain: PromptDomain,
    pub window_newline_index: u16,
}

#[derive(Debug, Clone)]
pub struct ResponseDomain {
    pub zero: (u16, u16),
    pub size_e: (u16, u16),
    pub buf: String,
    pub holder: String,
    pub indices: Vec<u16>,
    pub newline_index: u16,
    pub is_done: bool,
}

#[derive(Debug, Clone)]
pub struct PromptDomain {
    pub zero: (u16, u16),
    pub size_e: (u16, u16),
    pub buf: String,
    pub holder: String,
    pub newline_index: u16,
    pub indices: Vec<u16>,
    pub is_submit: bool,
}

impl Canvas {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        logger("[] BUILDING CANVAS");
        let mut stdout = std::io::stdout(); // temporary handle to stdout
        match execute!(stdout, EnterAlternateScreen) {
            Ok(_) => {
                logger("[] entered alternate screen successfully");
            }
            Err(e) => {
                logger(&format!(
                    "[] failed to enter alternate screen! error: {}",
                    e
                ));
                return Err(Box::new(e));
            }
        }
        match crossterm::terminal::enable_raw_mode() {
            Ok(_) => {
                logger("[] raw terminal enabled");
            }
            Err(e) => {
                logger(&format!("[] failed to enable raw mode! error: {}", e));
                return Err(Box::new(e));
            }
        }
        // match execute!(stdout, DisableLineWrap) {
        //     Ok(_) => {
        //         logger("[] successfully disabled line wrapping");
        //     }
        //     Err(e) => {
        //         logger(&format!("[] failed to disable line wrapping! error: {}", e));
        //         return Err(Box::new(e));
        //     }
        // }  why the fuck did i disable line wrapping
        match execute!(stdout, Clear(ClearType::All)) {
            Ok(_) => {
                logger("[] successfully cleared alternate screen buffer");
            }
            Err(e) => {
                logger(&format!(
                    "[] failed to clear alternate screen! error: {}",
                    e
                ));
                return Err(Box::new(e));
            }
        }
        let size: (u16, u16) = match terminal::size() {
            Ok((w, h)) => {
                logger(&format!("[] retrieved terminal size: ({}, {})", &w, &h));
                (w, h)
            }
            Err(e) => {
                logger(&format!("[] failed to get terminal size! error: {}", e));
                return Err(Box::new(e));
            }
        };

        let response_domain = ResponseDomain::build((size.0, size.1));
        logger("[] response domain constructed]");
        let prompt_domain = PromptDomain::build((size.0, size.1));
        logger("[] response domain constructed");
        logger("[] CANVAS SUCCESSFULLY CONSTRUCTED");
        let window_newline_index = 0;

        Ok(Self {
            size,
            response_domain,
            prompt_domain,
            window_newline_index,
        })
    }
}

impl ResponseDomain {
    // Response domain gets approx upper 2 thirds of screen.
    pub fn build(size: (u16, u16)) -> Self {
        let zero = (0, 0); // because upper 2/3 of screen, top left will be 0, 0
        let size_e = (size.0, (2 * size.1) / 3); // full
        let buf = String::new();
        let newline_index = 0;
        let indices: Vec<u16> = Vec::new();
        let holder: String = String::new();
        let is_done = false;
        Self {
            zero,
            size_e,
            buf,
            holder,
            indices,
            newline_index,
            is_done,
        }
    }
    pub fn update(&mut self, size: (u16, u16)) {
        self.zero = (0, 0); // because upper 2/3 of screen, top left will be 0, 0
        self.size_e = (size.0, (2 * size.1) / 3); // full
    }
    pub fn calculate_buf_lines(&self) -> u16 {
        let lines = self.buf.len() / self.size_e.0 as usize; // lines that will fit
        lines as u16
    }
    // pub fn format_buf(&mut self) {
    //     self.indices.clear();
    //     let lines = self.calculate_buf_lines();
    //     for x in 0..=lines {
    //         let y = x.clone();
    //         let index = (self.buf.len() * 1) - y as usize;
    //         let result = insert_char(&self.buf, index, '\n');
    //         self.indices.push(index as u16);
    //         self.buf = result;
    //     }
    //     logger(&format!("BUFFER: {}", self.buf));
    // }
    // pub fn format_string\
}

impl PromptDomain {
    // Prompt domain gets bottom 1/3 of screen.
    pub fn build(size: (u16, u16)) -> Self {
        let zero = (0, (2 * size.1) / 3); // 0 on x, 2/3rds way down on y
        let size_e = (size.0, (size.1 - zero.1)); // effective size of window = (width, (distance from starting y coord to bottom))
        let buf = String::new();
        let newline_index = 0;
        let is_submit = false;
        let indices: Vec<u16> = Vec::new();
        let holder: String = String::new();
        Self {
            zero,
            size_e,
            buf,
            holder,
            newline_index,
            indices,
            is_submit,
        }
    }
    pub fn update(&mut self, size: (u16, u16)) {
        self.zero = (0, (2 * size.1) / 3); // 0 on x, 2/3rds way down on y
        self.size_e = (size.0, (size.1 - self.zero.1)); // effective size of window = (width, (distance from starting y coord to bottom))
    }
    pub fn calculate_buf_lines(&self) -> u16 {
        let lines = self.buf.len() / self.size_e.0 as usize; // lines that will fit
                                                             // let last = self.buf.len() % self.size_e.0 as usize;
        lines as u16
    }
    // pub fn format_buf(&mut self) {
    //     self.indices.clear();
    //     // let lines: Vec<&str> = self.buf.lines().collect();
    //     let mut string = String::new();
    //     let lines = split_into_slices(&self.buf, self.size_e.0 as usize);
    //     for line in lines {
    //         string.push_str(line);
    //         string.push('\n');
    //         self.indices.push(line.len() as u16 - 1);
    //     }
    // for line in lines {
    //     if line.len() >= self.size_e.0 as usize || line.len() >= self.size_e.0 as usize - 1 {
    //         let line = format!("{}\n", line);
    //         string.push_str(&line);
    //         self.indices.push(line.len() as u16 + 1);
    //         logger(&format!(
    //             "[] index pushed: {}, canvas width: {}",
    //             line.len() as u16 + 1,
    //             self.size_e.0
    //         ));
    //     }
    // }
    // self.indices.clear();
    // let lines = self.calculate_buf_lines();
    // if lines <= 1 || lines <= 0 {
    //     return; // returns
    // }
    // for x in 0..=lines {
    //     let index = (self.buf.len() * 1) - x as usize;
    //     let result = insert_char(&self.buf, index, '\n');
    //     self.indices.push(index as u16);
    //     self.buf = result;
    // }
    // logger(&format!("BUFFER: {}", self.buf));
    // }
}

pub async fn clear_canvas(stdout: Arc<Mutex<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout.lock().unwrap();
    match stdout.execute(terminal::Clear(ClearType::All)) {
        Ok(_) => {
            logger("[] successfully cleared stdout buffer");
            return Ok(());
        }
        Err(e) => {
            logger(&format!("[] failed to clear stdout buffer! error: {}", e));
            return Err(Box::new(e));
        }
    }
}

fn insert_char(original: &str, index: usize, ch: char) -> String {
    let mut result = String::new();
    result.push_str(&original[..index]);
    result.push(ch);
    result.push_str(&original[index..]);
    result
}

fn split_into_slices(s: &str, n: usize) -> Vec<&str> {
    let mut slices = Vec::new();
    let mut start = 0;

    while start < s.len() {
        let end = (start + n).min(s.len());
        slices.push(&s[start..end]);
        start += n;
    }

    slices
}

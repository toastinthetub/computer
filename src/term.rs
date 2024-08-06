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
}

#[derive(Debug, Clone)]
pub struct ResponseDomain {
    pub zero: (u16, u16),
    pub size_e: (u16, u16),
    pub buf: String,
    pub newline_index: u16,
}

#[derive(Debug, Clone)]
pub struct PromptDomain {
    pub zero: (u16, u16),
    pub size_e: (u16, u16),
    pub buf: String,
    pub newline_index: u16,
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
        match execute!(stdout, DisableLineWrap) {
            Ok(_) => {
                logger("[] successfully disabled line wrapping");
            }
            Err(e) => {
                logger(&format!("[] failed to disable line wrapping! error: {}", e));
                return Err(Box::new(e));
            }
        }
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

        Ok(Self {
            size,
            response_domain,
            prompt_domain,
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
        Self {
            zero,
            size_e,
            buf,
            newline_index,
        }
    }
}

impl PromptDomain {
    // Prompt domain gets bottom 1/3 of screen.
    pub fn build(size: (u16, u16)) -> Self {
        let zero = (0, (2 * size.1) / 3); // 0 on x, 2/3rds way down on y
        let size_e = (size.0, (size.1 - zero.1)); // effective size of window = (width, (distance from starting y coord to bottom))
        let buf = String::new();
        let newline_index = 0;
        Self {
            zero,
            size_e,
            buf,
            newline_index,
        }
    }
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

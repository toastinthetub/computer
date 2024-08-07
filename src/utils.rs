use crossterm::cursor::{
    DisableBlinking, Hide, MoveLeft, MoveToColumn, MoveToNextLine, SetCursorStyle,
};
use crossterm::style::Stylize;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    cursor::{MoveDown, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};

// TODO: Implement linewrapping on prompt
// TODO: Implement linewrapping on response
// TODO: IMPLEMENT SCROLLING ON RESPONSE!! THIS IS GOING TO BE FUCKED
// TODO: Implement submit

use tokio::io::stdout;
use tokio::time::{Duration, Sleep};

use crate::{
    color::{color_bg, color_style, Color},
    computer::Computer,
};
use std::env::current_exe;
use std::iter::Repeat;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Stdout, Write},
    path::Path,
    sync::{Arc, Mutex},
};

pub const FILEPATH: &str = "./logs.txt";
pub const BOX_CHAR: &str = "\x1b[0;105m \x1b[0m";

/*
.___   _____ __________________ __________________________    __________________._.
|   | /     \\______   \_____  \\______   \__    ___/  _  \   \      \__    ___/| |
|   |/  \ /  \|     ___//   |   \|       _/ |    | /  /_\  \  /   |   \|    |   | |
|   /    Y    \    |   /    |    \    |   \ |    |/    |    \/    |    \    |    \|
|___\____|__  /____|   \_______  /____|_  / |____|\____|__  /\____|__  /____|    __
            \/                 \/       \/                \/         \/          \/
*/

pub struct State {
    stdout: Arc<Mutex<std::io::Stdout>>,
    pub computer: Computer,
    pub status: String,
}

// ---------------------------

impl State {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let mut stdout = std::io::stdout();
        if let Err(e) = execute!(
            stdout,
            SetCursorStyle::SteadyUnderScore,
            DisableBlinking,
            Hide
        ) {
            return Err(Box::new(e));
        }
        let computer = match Computer::build() {
            Ok(computer) => {
                logger("[] successfully built computer!");
                computer
            }
            Err(e) => {
                logger(&format!("[] failed to build computer! error: {}", e));
                return Err(e);
            }
        };
        let status = String::new();
        let stdout = Arc::new(Mutex::new(std::io::stdout()));
        Ok(Self {
            stdout,
            computer,
            status,
        })
    }
    pub async fn event_listener(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if event::poll(Duration::from_millis(16)).unwrap() {
                // 12 fps
                let event = event::read()?;
                self.event_handler(event).await?;
            }
            if let Err(e) = self.render() {
                return Err(e);
            }
            if let Err(e) = self.update_margins() {
                return Err(e);
            }
        }
    }
    async fn event_handler(&mut self, event: Event) -> io::Result<()> {
        // let mut stdout_lock = self.stdout.lock().unwrap();
        // logger(&format!(
        //     "[] debug event! if theres a keycode here it is: {} + {}",
        //     (match event {
        //         Event::Key(KeyEvent { code, .. }) => {
        //             code.to_string() // scope shit i don't want it to be a string
        //         }
        //         _ => {
        //             "no event".to_owned()
        //         }
        //     }),
        //     (match event {
        //         Event::Key(KeyEvent { modifiers, .. }) => {
        //             modifiers.to_string()
        //         }
        //         _ => {
        //             "no modifiers".to_owned()
        //         }
        //     })
        // ));
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                KeyCode::Backspace => {
                    self.computer.canvas.prompt_domain.buf.pop();
                }
                KeyCode::Enter => {
                    // TODO: Fix shift + enter. Why the fuck doesnt this work?
                    self.computer.canvas.prompt_domain.buf.push('\n'); // clear buffer...
                                                                       // TODO: submit()
                }
                KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                    // SUBMIT! Save the buffer, handle it

                    // handle_buf();
                    self.computer.canvas.prompt_domain.buf.clear();
                }
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    logger(&format!(
                        "[] ctrl c was pressed, exiting! final str:\n---\n{}\n---\n",
                        self.computer.canvas.prompt_domain.buf,
                    ));
                    kill(&format!(
                        "ctrl c pressed! final string:\n{}",
                        self.computer.canvas.prompt_domain.buf
                    ));
                    return Ok(());
                }
                KeyCode::Char(x) => {
                    self.computer.canvas.prompt_domain.buf.push(x); // christ this is ugly
                }
                _ => {
                    logger(&format!("[] unhandled keypress! key: {}", code));
                }
            },
            Event::Resize(width, height) => {
                self.computer.canvas.size = (width, height);
                logger("[] hey! the size of the terminal changed!");
            }
            _ => logger(&format!("[] hey! unhandled event! event: {:?}", event)),
        }
        Ok(())
    }
    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        // let mut stdout = std::io::stdout();
        // let mut stdout = self.stdout.lock().unwrap();
        {
            // different scope so that stdout.lock() can obtain deref self...fucking rust compiler ._.
            let mut stdout = self.stdout.lock().unwrap();
            match execute!(stdout, Clear(ClearType::All)) {
                Ok(_) => {
                    // do nothing
                }
                Err(e) => {
                    logger(&format!("[] failed to clear stdout buffer! error: {}", e));
                    return Err(Box::new(e));
                }
            }
            match execute!(stdout, MoveToNextLine(1)) {
                Ok(_) => {
                    // do nothing
                }
                Err(e) => {
                    logger(&format!("[] failed to reposition cursor! error: {}", e));
                    return Err(Box::new(e));
                }
            }
            // match execute!(stdout, MoveLeft(self.computer.canvas.size.0)) {
            //     Ok(_) => {
            //         // do nothing
            //     }
            //     Err(e) => {
            //         logger(&format!("[] failed to reposition cursor! error: {e}"));
            //         return Err(Box::new(e));
            //     }
            // }
            for line in self.computer.canvas.prompt_domain.buf.lines() {
                if let Err(e) = stdout.write_all(line.as_bytes()) {
                    return Err(Box::new(e));
                }
                if let Err(e) = execute!(stdout, MoveToNextLine(1)) {
                    return Err(Box::new(e));
                }
            }
            // if let Err(e) = stdout.write_all(self.computer.canvas.prompt_domain.buf.as_bytes()) {
            //     logger(&format!("[] failed to write to stdout buf! error: {}", e));
            //     return Err(Box::new(e));
            // }
            stdout.flush().unwrap();

            // THIS SUCKS REALLY BAD! ============================================================================

            match self.computer.canvas.response_domain.buf.chars().last() {
                Some(char) => {
                    if char == '\n' {
                        logger("[] the last char was \\n!");
                        match execute!(
                            stdout,
                            MoveToNextLine(1) // MoveLeft(
                                              //     self.computer
                                              //         .canvas
                                              //         .response_domain
                                              //         .buf
                                              //         .lines()
                                              //         .last()
                                              //         .to_owned()
                                              //         .unwrap()
                                              //         .len() as u16
                                              // )
                        ) {
                            Ok(_) => {
                                // success, do nothing
                            }
                            Err(e) => {
                                logger(&format!("[] failed to move the cursor! error: {}", e));
                                return Err(Box::new(e));
                            }
                        }
                    }
                }
                None => {
                    // empty string
                    // do nothing
                }
            }

            // KILL THIS WITH FIRE!!! ============================================================================
        } // end of lock scope
        if let Err(e) = self.create_seperator() {
            return Err(e);
        }
        Ok(())
        // TODO: handle some fcking errors!
    }
    fn create_seperator(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stdout = self.stdout.lock().unwrap();
        if let Err(e) = execute!(
            stdout,
            MoveTo(
                self.computer.canvas.prompt_domain.zero.0,
                self.computer.canvas.prompt_domain.zero.1
            )
        ) {
            logger(&format!("[] failed to reposition cursor! error: {}", e));
            return Err(Box::new(e));
        }
        let bar = BOX_CHAR.repeat(self.computer.canvas.size.0 as usize);
        match stdout.write_all(bar.as_bytes()) {
            Ok(_) => {
                // do nothing
            }
            Err(e) => {
                logger(&format!(
                    "[] failed to write bytes to stdout buf! error: {}",
                    e
                ));
                return Err(Box::new(e));
            }
        }
        let center_pos =
            (self.computer.canvas.size.0.checked_div(2).unwrap()) - (self.status.len() / 2) as u16;
        let styled_status = color_style(&self.status, Color::White, "none");
        let styled_status = color_bg(&styled_status, Color::HighIntensityPurple); // YOU WERE HERE!
        if let Err(e) = execute!(stdout, MoveToColumn(center_pos)) {
            return Err(Box::new(e));
        }
        // if let Err(e) = execute!(stdout, MoveLeft(self.computer.canvas.size.0)) {
        //     return Err(Box::new(e));
        // }
        print!("{}", styled_status);
        if let Err(e) = stdout.flush() {
            logger(&format!("[] failed to flush stdout buf! error: {}", e));
            return Err(Box::new(e));
        }
        Ok(())
    }
    pub fn update_margins(&mut self) -> Result<(), Box<dyn Error>> {
        let size: (u16, u16) = self.computer.canvas.size;
        self.computer.canvas.response_domain.update(size);
        self.computer.canvas.prompt_domain.update(size);
        Ok(())
    }
    pub fn update_status(&mut self, txt: &str) {
        self.status = txt.to_owned();
    }
}

pub fn logger(txt: &str) {
    let mut file = match OpenOptions::new().append(true).create(true).open(FILEPATH) {
        Ok(file) => file,
        Err(e) => {
            kill(&format!(
                "failed to open log file. something has gone terribly wrong.\nheres the error: {}",
                e
            ));
            std::process::exit(1); // must please compiler
        }
    };
    match writeln!(file, "{}", txt) {
        Ok(_) => {

        }
        Err(e) => {
            kill(&format!("failed to write to log file. it does exist tho. something has gone terribly wrong.\nheres the error: {}", e))
        }
    }
}

pub fn kill(msg: &str) {
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen).expect("failed to leave alternate screen! fuck!");
    terminal::disable_raw_mode().expect("failed to disable raw mode! fuck!");
    println!("kill message: {}", msg);
    std::process::exit(0);
}

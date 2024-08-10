use crossterm::cursor::{
    DisableBlinking, Hide, MoveLeft, MoveToColumn, MoveToNextLine, SetCursorStyle,
};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::QueueableCommand;
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

use tokio::fs::try_exists;
use tokio::io::stdout;
use tokio::sync::SetError;
use tokio::time::{Duration, Sleep};
use tokio_stream::StreamExt;

use crate::{
    color::{self, color_bg, color_fg, Color, Style},
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
    pub stdout: Arc<Mutex<std::io::Stdout>>,
    pub computer: Computer,
    pub loading: bool,
    pub status: String,
}

// ---------------------------

impl State {
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        let mut stdout = std::io::stdout();
        if let Err(e) = execute!(
            stdout,
            SetCursorStyle::SteadyUnderScore,
            DisableBlinking,
            Hide
        ) {
            return Err(Box::new(e));
        }
        let computer = match Computer::build().await {
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
        let loading: bool = false;
        Ok(Self {
            stdout,
            computer,
            loading,
            status,
        })
    }
    pub async fn event_listener(&mut self) -> Result<(), Box<dyn Error>> {
        // let task = tokio::task::spawn(test_async());
        loop {
            // let mut stdout = match self.stdout.lock() {
            //     Ok(stdout) => stdout,
            //     Err(e) => return panic!("failed to get stdout lock! {}", e), // will this always succeed? why compiler say unreachable
            // };
            // let str = &format!(
            //     "terminal of size {}x{} is too small.",
            //     self.computer.canvas.size.0, self.computer.canvas.size.1
            // );
            // let str = color_fg(str, Color::HighIntensityRed);
            // if self.computer.canvas.size.0 <= 45 || self.computer.canvas.size.1 <= 20 {
            //     if let Err(e) = stdout.queue(Clear(ClearType::All)) {
            //         return Err(Box::new(e));
            //     }
            //     if let Err(e) = stdout.queue(MoveTo(
            //         self.computer.canvas.size.0 - str.len() as u16,
            //         self.computer.canvas.size.1,
            //     )) {
            //         return Err(Box::new(e));
            //     }
            //     print!("{}", str);
            //     if let Err(e) = stdout.flush() {
            //         return Err(Box::new(e));
            //     }
            // }
            // std::mem::drop(stdout);
            if event::poll(Duration::from_millis(16)).unwrap() {
                // 60fps
                let event = event::read()?;
                self.event_handler(event).await?;
            }
            // self.computer.canvas.prompt_domain.format_buf();
            // let lines: Vec<&str> = self.computer.canvas.prompt_domain.buf.lines().collect();
            // let len = lines.len();
            // logger(&format!(
            // "[] buf lines: {}, term: {}x{}",
            // len, self.computer.canvas.size.0, self.computer.canvas.size.1
            // ));
            if let Err(e) = self.render() {
                return Err(e);
            }
            if let Err(e) = self.update_margins() {
                return Err(e);
            }
            if let Err(e) = self.draw_status() {
                return Err(e);
            }
            if !self.computer.canvas.response_domain.is_done {
                {
                    let mut stream = self.computer.stream.lock().expect("shits poisoned");
                    match stream.next().await {
                        Some(Ok(res)) => {
                            if let Some(msg) = res.message {
                                self.computer
                                    .canvas
                                    .response_domain
                                    .buf
                                    .push_str(&msg.content);
                                logger(&format!("theres somethign!! {}", msg.content));
                            }
                        }
                        Some(Err(_)) => {
                            disable_raw_mode().unwrap();
                            panic!("this is fucked!")
                        }
                        None => {
                            // Stream is done being written to
                            // self.computer.canvas.response_domain.is_done = true;
                            logger("[] we triggered done lol");
                        }
                    }
                    // if let Some(Ok(res)) = stream.next().await {
                    //     logger("there was some stream or something");
                    //     if let Some(msg) = res.message {
                    //         self.computer
                    //             .canvas
                    //             .response_domain
                    //             .buf
                    //             .push_str(&msg.content);
                    //         logger(&format!("[] pushed str to buf: {}", msg.content));
                    //     }
                    // }
                    // std::mem::drop(stream); gotta do it the rust way i guess
                }
            }
            if self.computer.canvas.prompt_domain.is_submit
            // && self.computer.canvas.response_domain.is_done
            {
                logger("[] submitted buf!");
                if let Err(e) = self.handle_buf().await {
                    return Err(e);
                }
                self.computer.canvas.prompt_domain.is_submit = false;
            } else if self.computer.canvas.prompt_domain.is_submit
                && !self.computer.canvas.response_domain.is_done
            {
                logger("[] tried and failed to submit buf!")
            }

            // if counter == 4 {
            //     future.await;
            //     return Ok(());
            // } else {
            //     count(4).await;
            // }
            // if task.is_finished() {
            // task.await.unwrap();
            // return Ok(());
            // }
        }
    }
    async fn event_handler(&mut self, event: Event) -> io::Result<()> {
        self.computer.canvas.prompt_domain.is_submit = false;
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
                    self.computer.canvas.prompt_domain.holder =
                        self.computer.canvas.prompt_domain.buf.clone(); // deep copy buf and hold it
                    self.computer.canvas.prompt_domain.buf.clear();
                    self.computer.canvas.prompt_domain.is_submit = true;
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
            let str = &format!(
                "terminal of size {}x{} is too small.",
                self.computer.canvas.size.0, self.computer.canvas.size.1
            );
            let str = color_fg(str, Color::HighIntensityRed);
            // while self.computer.canvas.size.0 <= 30 || self.computer.canvas.size.1 <= 20 {
            //     let (_, _) = match crossterm::terminal::size() {
            //         Ok((w, h)) => {
            //             self.computer.canvas.size = (w, h);
            //             if w > 30 && h > 20 {
            //                 break;
            //             } else {
            //                 if let Err(e) = stdout.queue(Clear(ClearType::All)) {
            //                     return Err(Box::new(e));
            //                 }
            //                 if let Err(e) = stdout.queue(MoveTo(
            //                     self.computer.canvas.size.0 - str.len() as u16,
            //                     self.computer.canvas.size.1,
            //                 )) {
            //                     return Err(Box::new(e));
            //                 }
            //                 print!("{}", str);
            //                 if let Err(e) = stdout.flush() {
            //                     return Err(Box::new(e));
            //                 }
            //             }
            //             (w, h)
            //         }
            //         Err(e) => {
            //             return Err(Box::new(e));
            //         }
            //     };
            // }
            //
            // fucking garbage stupid shit
            //
            // if self.computer.canvas.size.0 <= 30 || self.computer.canvas.size.1 <= 20 {
            //     if let Err(e) = stdout.queue(Clear(ClearType::All)) {
            //         return Err(Box::new(e));
            //     }
            //     if let Err(e) = stdout.queue(MoveTo(
            //         self.computer.canvas.size.0 - str.len() as u16,
            //         self.computer.canvas.size.1,
            //     )) {
            //         return Err(Box::new(e));
            //     }
            //     print!("{}", str);
            //     if let Err(e) = stdout.flush() {
            //         return Err(Box::new(e));
            //     }
            //     return Ok(());
            // }
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

            stdout.flush().unwrap();

            if let Err(e) = execute!(stdout, MoveTo(0, 0)) {
                return Err(Box::new(e));
            }
            for line in self.computer.canvas.response_domain.buf.lines() {
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
            /*
            match self.computer.canvas.response_domain.buf.chars().last() {
                Some(char) => {
                    if char == '\n' {
                        logger("[] the last char was \\n!");
                        match execute!(
                            stdout,
                            MoveToNextLine(1) // MoveLeft( wow i'm a fucking dumbass
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
            } */

            // KILL THIS WITH FIRE!!! ============================================================================ killed it lol - sam, later
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
        std::mem::drop(stdout);

        Ok(())
    }
    fn draw_status(&mut self) -> Result<(), Box<dyn Error>> {
        // this can fail but i don't know how to solve the lifetime issue
        // fucking lifetimes bruh
        {
            let mut stdout = match self.stdout.lock() {
                Ok(stdout) => stdout,
                Err(e) => {
                    logger(&format!(
                        "[] failed to obtain RAII guard on stdout! error: {}",
                        e
                    ));
                    panic!(
                        "couldn't get the RAII guard :( so we had to panic\nerror:{}",
                        e
                    );
                }
            };
            if let Err(e) = execute!(stdout, MoveToColumn(0)) {
                return Err(Box::new(e));
            }
            let status = color_bg("STATUS:", Color::HighIntensityBgPurple);
            let status = color_fg(&status, Color::HighIntensityWhite);
            print!("{}", &status);
            std::mem::drop(status);
            // self.status = format!(
            //     "{} TERM: {}x{}",
            //     self.status, self.computer.canvas.size.0, self.computer.canvas.size.1
            // );
            let center_pos = (self.computer.canvas.size.0.checked_div(2).expect("failed!"))
                - (self.status.len() / 2) as u16;
            // color_style(&self.status, Color::White, "none");
            let styled_status = color_bg(&self.status, Color::HighIntensityBgPurple); // YOU WERE HERE!
            let styled_status = color_fg(&styled_status, Color::HighIntensityWhite);
            if let Err(e) = execute!(stdout, MoveToColumn(center_pos)) {
                return Err(Box::new(e));
            }
            print!("{}", styled_status);
            if let Err(e) = stdout.flush() {
                logger(&format!("[] failed to flush stdout buf! error: {}", e));
                return Err(Box::new(e));
            }
        }
        Ok(())
    }
    async fn load(&mut self) {
        todo!();
    }
    pub fn update_margins(&mut self) -> Result<(), Box<dyn Error>> {
        let size: (u16, u16) = self.computer.canvas.size;
        self.computer.canvas.response_domain.update(size);
        self.computer.canvas.prompt_domain.update(size);
        self.status = format!(
            "term size: {}x{}",
            self.computer.canvas.size.0, self.computer.canvas.size.1
        );
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

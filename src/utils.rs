use crossterm::cursor::MoveLeft;
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
// TODO: Implement submit

use tokio::time::{Duration, Sleep};

use crate::computer::Computer;
use std::iter::Repeat;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Stdout, Write},
    path::Path,
    sync::{Arc, Mutex},
};

pub const FILEPATH: &str = "./logs.txt";
pub const BOX_CHAR: &str = "▉";

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
}

// ---------------------------

impl State {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let stdout = Arc::new(Mutex::new(io::stdout()));
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
        Ok(Self { stdout, computer })
    }
    pub async fn event_listener(&mut self) -> io::Result<()> {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                let event = event::read()?;
                self.event_handler(event).await?;
            }
            self.render().unwrap();
        }
    }
    async fn event_handler(&mut self, event: Event) -> io::Result<()> {
        // let mut stdout_lock = self.stdout.lock().unwrap();
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                KeyCode::Backspace => {
                    self.computer.canvas.prompt_domain.buf.pop();
                    // logger("[] backspace was pressed!");
                    // logger(&format!(
                    //     "[] current string: [{}]",
                    //     self.computer.canvas.prompt_domain.buf
                    // ))
                }
                // KeyCode::Char('\n') if modifiers.contains(KeyModifiers::SHIFT) => {
                //     self.computer.canvas.prompt_domain.buf.clear(); // clear buffer
                //                                                     // TODO: submit()
                //     logger("[] shift + enter was pressed!]");
                // }
                KeyCode::Enter => {
                    // TODO: Fix shift + enter.
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        self.computer.canvas.prompt_domain.buf.clear(); // clear buffer
                                                                        // TODO: submit()
                        logger("[] shift + enter was pressed!]");
                    } else {
                        self.computer.canvas.prompt_domain.buf.push('\n');
                    }
                }
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    kill("ctrl c pressed!");
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
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();
        self.create_seperator();
        execute!(stdout, MoveDown(1)).unwrap();
        execute!(stdout, MoveLeft(self.computer.canvas.size.0)).unwrap();
        print!("{}", self.computer.canvas.prompt_domain.buf);
        stdout.flush().unwrap();
        Ok(())
        // TODO: handle some fcking errors!
    }

    fn create_seperator(&mut self) {
        let mut stdout = self.stdout.lock().unwrap();
        execute!(
            stdout,
            MoveTo(
                self.computer.canvas.prompt_domain.zero.0,
                self.computer.canvas.prompt_domain.zero.1
            )
        )
        .unwrap();
        // execute!(stdout, SavePosition).unwrap();
        let bar = BOX_CHAR.repeat(self.computer.canvas.size.0 as usize);
        stdout.write(bar.as_bytes()).unwrap();
        stdout.flush().unwrap();
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

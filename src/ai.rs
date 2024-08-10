use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};

use crossterm::{
    cursor::{MoveDown, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};

use std::{error::Error, sync::Arc};

use crate::utils::logger;

pub fn build_ollama() -> Result<(Ollama, Vec<ChatMessage>), Box<dyn Error>> {
    logger("[] building ollama!");
    let ollama = Ollama::default(); // by default is localhost:11434
    logger("[] successfully built model");
    let messages: Vec<ChatMessage> = vec![];
    logger("[] successfully built history vec");
    Ok((ollama, messages))
}

impl crate::utils::State {
    pub async fn handle_buf(&mut self) -> Result<(), Box<dyn Error>> {
        let mut history = self
            .computer
            .history
            .lock()
            .expect("this shit is poisoned you're fucked anyway.");
        history.push(ChatMessage::user(
            self.computer.canvas.prompt_domain.buf.clone(), // THIS IS BAD I KNOW BUT YOU FUCKING SOLVE IT.
        ));
        let mut stream = self
            .computer
            .stream
            .lock()
            .expect("this shit is poisoned lol");
        {
            *stream = self // i fucking hate the borrowchecker
                .computer
                .ollama
                .send_chat_messages_stream(ChatMessageRequest::new(
                    "llama2-uncensored:latest".to_owned(),
                    history.clone(),
                ))
                .await
                .unwrap();
        }
        Ok(())
    }
}

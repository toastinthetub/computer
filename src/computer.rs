use crate::{ai::build_ollama, term::Canvas, utils::logger};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};

use std::{
    error::Error,
    io::Write,
    sync::{Arc, Mutex},
};

use crate::utils::State;

#[derive(Clone)]
pub struct Computer {
    pub ollama: Ollama,
    pub history: Arc<Mutex<Vec<ChatMessage>>>,
    pub stream: Arc<Mutex<ChatMessageResponseStream>>,
    pub canvas: Canvas,
}

impl Computer {
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        logger("[] BUILDING COMPUTER]");
        let (ollama, history) = match build_ollama() {
            Ok((ollama, history)) => {
                logger("[] successfully built ollama and chat vector!");
                (ollama, history)
            }
            Err(e) => {
                logger(&format!("[] failed to build ollama! error: {}", e));
                return Err(e);
            }
        };
        let canvas = match Canvas::build() {
            Ok(canvas) => canvas,
            Err(e) => {
                logger(&format!("[] failed to build canvas! error: {}", e));
                return Err(e); // lol forgot this was already in a box
            }
        };
        let history: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(history));
        let history = history.lock().unwrap();
        let stream: ChatMessageResponseStream = ollama
            .send_chat_messages_stream(ChatMessageRequest::new(
                "llama2-uncensored:latest".to_string(),
                history.clone(),
            ))
            .await?;
        std::mem::drop(history);
        let history: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(Vec::new()));
        let stream = Arc::new(Mutex::new(stream));
        Ok(Self {
            ollama,
            history,
            stream,
            canvas,
        })
    }
}

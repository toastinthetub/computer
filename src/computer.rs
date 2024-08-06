use crate::{ai::build_ollama, term::Canvas, utils::logger};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};

use std::error::Error;

#[derive(Debug, Clone)]
pub struct Computer {
    pub ollama: Ollama,
    pub history: Vec<ChatMessage>,
    pub canvas: Canvas,
}

impl Computer {
    pub fn build() -> Result<Self, Box<dyn Error>> {
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
        Ok(Self {
            ollama,
            history,
            canvas,
        })
    }
}

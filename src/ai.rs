use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};

use std::error::Error;

use crate::utils::logger;

pub fn build_ollama() -> Result<(Ollama, Vec<ChatMessage>), Box<dyn Error>> {
    logger("[] building ollama!");
    let ollama = Ollama::default(); // by default is localhost:11434
    logger("[] successfully built model");
    let messages: Vec<ChatMessage> = vec![];
    logger("[] successfully built history vec");
    Ok((ollama, messages))
}

mod ai;
mod color;
mod computer;
mod term;
mod utils;

use std::io::Write;

use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{
    cursor::{MoveDown, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;
use utils::State;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State::build().await.unwrap();
    state.update_status("this is a status!");
    let future = state.event_listener();
    // println!("something async is happening.");
    // tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    future.await.unwrap();
    let mut stdout = state.stdout.lock().unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
    // println!("something async finished!");

    Ok(())
    // let ollama = Ollama::default();

    // let mut stdout = stdout();

    // let mut messages: Vec<ChatMessage> = vec![];

    // loop {
    //     stdout.write_all(b"\n> ").await?;
    //     stdout.flush().await?;

    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input)?;

    //     let input = input.trim_end();
    //     if input.eq_ignore_ascii_case("exit") {
    //         break;
    //     }

    //     let user_message = ChatMessage::user(input.to_string());
    //     messages.push(user_message);

    //     let mut stream: ChatMessageResponseStream = ollama
    //         .send_chat_messages_stream(ChatMessageRequest::new(
    //             "widow:latest".to_string(),
    //             messages.clone(),
    //         ))
    //         .await?;

    //     let mut response = String::new();
    //     while let Some(Ok(res)) = stream.next().await {
    //         if let Some(assistant_message) = res.message {
    //             stdout
    //                 .write_all(assistant_message.content.as_bytes())
    //                 .await?;
    //             stdout.flush().await?;
    //             response += assistant_message.content.as_str();
    //         }
    //     }
    //     messages.push(ChatMessage::assistant(response));
    // }

    // Ok(())
}

// pub async fn test_async() {
//     let mut stdout = std::io::stdout();
//     let _ = stdout.queue(Clear(ClearType::All));
//     let _ = stdout.queue(MoveTo(0, 0));
//     let _ = std::io::Stdout::flush(&mut stdout); // for some reason it thinks i mean tokio

//     println!("something async finished!");
//     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
// }

use std::sync::{Arc, Mutex};

use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageBody {
    Init {
        msg_id: u64,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        msg_id: u64,
        in_reply_to: u64,
    },
    Echo {
        msg_id: u64,
        echo: String,
    },
    EchoOk {
        in_reply_to: u64,
        msg_id: u64,
        echo: String,
    },
    Generate {
        msg_id: u64,
    },
    GenerateOk {
        in_reply_to: u64,
        msg_id: u64,
        id: String,
    },
    Error {
        in_reply_to: u64,
        msg_id: u64,
        code: u64,
        text: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    body: MessageBody,
    src: String,
    dest: String,
}

#[derive(Clone, Debug)]
struct State {
    next_message_id: u64,
    node_id: String,
}

fn main() -> Result<(), serde_json::Error> {
    let state = Arc::new(Mutex::new(State {
        next_message_id: 0u64,
        node_id: "".to_string(),
    }));
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let message: Message = serde_json::from_str(input.as_str())?;
        handle_message(state.clone(), message)?;
    }
}

fn handle_message(
    state: Arc<Mutex<State>>,
    request_message: Message,
) -> Result<(), serde_json::Error> {
    let mut state = state.lock().unwrap();
    let next_message_id = state.next_message_id;

    let response_body: MessageBody = match request_message.body {
        MessageBody::Init {
            msg_id: in_reply_to,
            node_id,
            node_ids: _,
        } => {
            state.node_id = node_id;

            MessageBody::InitOk {
                msg_id: next_message_id,
                in_reply_to,
            }
        }
        MessageBody::Echo {
            msg_id: in_reply_to,
            echo,
        } => MessageBody::EchoOk {
            msg_id: next_message_id,
            in_reply_to,
            echo,
        },
        MessageBody::Generate {
            msg_id: in_reply_to,
        } => MessageBody::GenerateOk {
            msg_id: next_message_id,
            in_reply_to,
            id: nanoid!(),
        },
        _ => {
            panic!("Unknown message type")
        }
    };

    let response_message = Message {
        body: response_body,
        src: request_message.dest,
        dest: request_message.src,
    };
    println!("{}", serde_json::to_string(&response_message)?);

    state.next_message_id += 1;

    Ok(())
}

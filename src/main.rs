use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum MessageBody {
    #[serde(rename = "init")]
    Init {
        msg_id: u64,
        node_id: String,
        node_ids: Vec<String>,
    },
    #[serde(rename = "init_ok")]
    InitOk { in_reply_to: u64 },
    #[serde(rename = "echo")]
    Echo { msg_id: u64, echo: String },
    #[serde(rename = "echo_ok")]
    EchoOk {
        in_reply_to: u64,
        msg_id: u64,
        echo: String,
    },
    #[serde(rename = "generate")]
    Generate { msg_id: u64 },
    #[serde(rename = "generate_ok")]
    GenerateOk {
        in_reply_to: u64,
        msg_id: u64,
        id: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u64,
    body: MessageBody,
    src: String,
    dest: String,
}

fn main() -> Result<(), serde_json::Error> {
    let mut next_message_id = 0u64;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let message: Message = serde_json::from_str(input.as_str())?;
        handle_message(next_message_id, message)?;
        next_message_id += 1;
    }
}

fn handle_message(next_message_id: u64, message: Message) -> Result<(), serde_json::Error> {
    match message.body {
        MessageBody::Init {
            msg_id,
            node_id: _,
            node_ids: _,
        } => {
            let reply = Message {
                id: next_message_id,
                body: MessageBody::InitOk {
                    in_reply_to: msg_id,
                },
                src: message.dest,
                dest: message.src,
            };
            println!("{}", serde_json::to_string(&reply)?);
        }
        MessageBody::Echo { msg_id, echo } => {
            let reply = Message {
                id: next_message_id,
                body: MessageBody::EchoOk {
                    in_reply_to: msg_id,
                    msg_id: next_message_id,
                    echo,
                },
                src: message.dest,
                dest: message.src,
            };
            println!("{}", serde_json::to_string(&reply)?);
        }
        MessageBody::Generate { msg_id } => {
            let reply = Message {
                id: next_message_id,
                body: MessageBody::GenerateOk {
                    in_reply_to: msg_id,
                    msg_id: next_message_id,
                    id: nanoid!(),
                },
                src: message.dest,
                dest: message.src,
            };
            println!("{}", serde_json::to_string(&reply)?);
        }
        _ => {
            panic!("Unknown message type")
        }
    }
    Ok(())
}

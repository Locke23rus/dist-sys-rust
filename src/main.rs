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
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u64,
    body: MessageBody,
    src: String,
    dest: String,
}

fn main() -> Result<(), serde_json::Error> {
    let mut message_id = 0u64;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let message: Message = serde_json::from_str(input.as_str())?;
        handle_message(message_id, message)?;
        message_id += 1;
    }
}

fn handle_message(message_id: u64, message: Message) -> Result<(), serde_json::Error> {
    match message.body {
        MessageBody::Init {
            msg_id,
            node_id: _,
            node_ids: _,
        } => {
            let reply = Message {
                id: message_id,
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
                id: 2,
                body: MessageBody::EchoOk {
                    in_reply_to: msg_id,
                    msg_id: message_id,
                    echo,
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

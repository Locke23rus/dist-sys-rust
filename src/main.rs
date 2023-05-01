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
    InitOk { msg_id: u64, in_reply_to: u64 },
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

fn handle_message(next_message_id: u64, request_message: Message) -> Result<(), serde_json::Error> {
    let response_body: MessageBody = match request_message.body {
        MessageBody::Init {
            msg_id: in_reply_to,
            node_id: _,
            node_ids: _,
        } => MessageBody::InitOk {
            msg_id: next_message_id,
            in_reply_to,
        },
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

    Ok(())
}

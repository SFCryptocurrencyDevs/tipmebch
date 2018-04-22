use std::env;
use std;
use serde_json;
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};

struct Client {
    out: Sender,
}

impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello WebSocket")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let money = &msg;
        deserialize_string(money);
        // self.out.close(CloseCode::Normal)
        Ok(())
    }

}

// TODO: close stream at some point
pub fn connect_to_stream () {
    let base = "ws://stream.meetup.com/2/event_comments";
    let url = format!("{}?event_id={}", base, &env::var("MEETUP_EVENT_ID").unwrap());

    connect(url, |out| Client { out: out } ).unwrap()
}

fn deserialize_string(money: &Message) -> std::result::Result<(), serde_json::Error> {
    let thing: &str = match money.as_text() {
        Ok(res) => res,
        _ => ""
    };

    let v: serde_json::Value = serde_json::from_str(thing)?;

    println!("{} said: {}", v["member"]["member_name"], v["comment"]);

    Ok(())
}
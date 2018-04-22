use std::env;
use std;
use serde_json;
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
use reqwest;
use std::collections::HashMap;

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

    respond(&v["id"]);

    Ok(())
}

// Really bad rust code -- this is just the beginning :)
pub fn respond(id: &serde_json::Value) -> std::result::Result<(), reqwest::Error> {
    let event_id = &env::var("MEETUP_EVENT_ID").unwrap();
    let id_here = &id.as_i64().unwrap().to_string();
    let mut map = HashMap::new();
    map.insert("comment", "Hello!");
    map.insert("event_id", event_id);
    map.insert("in_reply_to", id_here);

    let client = reqwest::Client::new();

    let base = "https://api.meetup.com/2/event_comment1";
    let url = &format!("{}?key={}", base, &env::var("MEETUP_API_KEY").unwrap());

    let res = client.post(url)
        .form(&map)
        .send()?;

    Ok(())
}
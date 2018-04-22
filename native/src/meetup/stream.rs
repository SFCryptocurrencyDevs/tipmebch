use std::env;

use ws::{connect, Error};
use std;

// TODO: close stream at some point
pub fn connect_to_stream () -> std::result::Result<(), Error>{
    let base = "ws://stream.meetup.com/2/event_comments";
    let url = format!("{}?event_id={}", base, &env::var("MEETUP_EVENT_ID").unwrap());

    // Connect to the url and call the closure
    connect(url, |out| {

        // The handler needs to take ownership of out, so we use move
        move |msg| {

            // Handle messages received on this connection
            println!("Client got message '{}'. ", msg);

            // TODO: don't actually send here (need this so that I return () instead of std::result::Result)
            out.send("Hello WebSocket")
        }

    })
}
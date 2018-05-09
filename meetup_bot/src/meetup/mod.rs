mod stellar;
mod stream;

use rand;
use rand::Rng;
use rocket;

// This endpoint is called by the meetup bot to
// get and display a memo to be used by the user
// to deposit funds into their "tipbot account"
#[get("/gen_memo")]
fn gen_memo() -> String {
    let memo: String = rand::thread_rng().gen_ascii_chars().take(15).collect();

    // Initalize a new poll listener
    stellar::Poll::init(memo.to_owned());

    format!("{}", memo)
}

pub fn init() {
    // Start the meetup bot stream
    stream::connect_to_stream();
    // Startup the rocket server
    rocket::ignite().mount("/", routes![gen_memo]).launch();
}

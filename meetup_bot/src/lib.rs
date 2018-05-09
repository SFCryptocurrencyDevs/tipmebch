#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;
extern crate serde_json;
extern crate ws;
extern crate stellar_client;
extern crate schedule_recv;
extern crate rand;

pub mod meetup;

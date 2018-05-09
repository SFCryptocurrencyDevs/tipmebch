#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate reqwest;
extern crate rocket;
extern crate schedule_recv;
extern crate serde_json;
extern crate stellar_client;
extern crate ws;

pub mod meetup;

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;

mod cors;
mod api;

use crate::{
    cors::CORS,
    api::v1,
};

fn main() {
    rocket::ignite()
        .mount("/", routes![v1::handle_stats])
        .attach(CORS)
        .launch();
}

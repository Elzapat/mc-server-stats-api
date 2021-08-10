#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;

mod cors;
mod api;

pub struct WorldDir<'a> {
    path: &'a str,
}

use crate::{
    cors::CORS,
    api::v1,
};

fn main() {
    rocket::ignite()
        .manage(WorldDir { path: env!("WORLD_DIR") })
        .mount("/", routes![
            v1::stats::handle_stats,
        ])
        .attach(CORS)
        .launch();
}

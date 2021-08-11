#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;

mod cors;
mod api;

use std::collections::HashMap;

pub struct WorldDir<'a> {
    path: &'a str,
}

pub struct UsernameCache<'a> {
    cache: HashMap<&'a str, &'a str>,
}

use crate::{
    cors::CORS,
};

fn main() {
    rocket::ignite()
        .manage(WorldDir { path: env!("WORLD_DIR") })
        .manage(UsernameCache { cache: HashMap::new() })
        .mount("/api/v1", routes![
            api::v1::stats::handle_stats,
        ])
        .mount("/api/v2", routes![
            api::v2::routes::player_stat,
            api::v2::routes::player_stats,
            api::v2::routes::global_stats,
        ])
        .attach(CORS)
        .launch();
}

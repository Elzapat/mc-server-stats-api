#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;

pub mod username_cache;
mod cors;
mod api;

pub struct WorldDir<'a> {
    path: &'a str,
}

use crate::{
    cors::CORS,
};

fn main() {
    rocket::ignite()
        .manage(WorldDir { path: env!("WORLD_DIR") })
        .manage(username_cache::UsernameCache::new())
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

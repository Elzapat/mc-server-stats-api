#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;

use rocket_contrib::json::Json;
use serde::Serialize;
use serde_json::Value;
use std::fs;

#[derive(Serialize)]
struct PlayerStat {
    success: bool,
    uuid: String,
    stat: u64
}

impl PlayerStat {
    pub fn uuid(self, uuid: String) -> Self {
        PlayerStat {
            uuid: uuid,
            ..self
        }
    }
}

#[derive(Serialize)]
struct Stats(Vec<PlayerStat>);

#[get("/stats?<uuid>&<stat_type>&<stat_value>")]
fn handle_stats(mut uuid: String, stat_type: String, stat_value: String) -> Json<Stats> {
    if &uuid == "all" {
        let mut all_stats = vec![];

        let files = match fs::read_dir("world/stats") {
            Ok(f) => f,
            Err(e) => return Json(Stats(vec![PlayerStat {
                success: false,
                uuid: format!("Error: {}", e),
                stat: 0
            }]))
        };
        for file in files {
            let path_os_str = match file {
                Ok(p) => p.path().into_os_string(),
                Err(_) => continue
            };
            let path_str = match path_os_str.into_string() {
                Ok(s) => s,
                Err(_) => continue
            };
            let pos = match path_str.rfind('/') {
                Some(p) => p,
                None => continue,
            };
            let player_uuid = &path_str[pos + 1..path_str.len() - 5];
            all_stats.push(get_stat(player_uuid.to_string(), stat_type.clone(), stat_value.clone()));
        }
        Json(Stats(all_stats))
    } else {
        if !uuid.contains("-") {
            uuid = format!("{}-{}-{}-{}-{}",
                &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32]);
        }
        Json(Stats(vec![get_stat(uuid, stat_type, stat_value)]))
    }
}

fn get_stat(uuid: String, stat_type: String, stat_value: String) -> PlayerStat {
    let failure = PlayerStat {
        success: false,
        uuid: String::from(""),
        stat: 0
    };

    let stats_str = match fs::read_to_string(format!("world/stats/{}.json", uuid)) {
        Ok(s) => s,
        Err(e) => return failure.uuid(format!("Error: {}, this player probably has never logged on the server", e))
    };
    let stats_json: Value = match serde_json::from_str(&stats_str) {
        Ok(v) => v,
        Err(e) => return failure.uuid(format!("Error: {}", e))
    };
    let value: u64 = match stats_json["stats"][stat_type][stat_value].as_u64() {
        Some(s) => s,
        None => return failure.uuid(String::from("Error: couldn't find this stat"))
    };

    PlayerStat {
        success: true,
        uuid: uuid,
        stat: value
    }
}


fn main() {
    rocket::ignite()
        .mount("/", routes![handle_stats])
        .launch();
}

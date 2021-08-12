use rocket_contrib::json::Json;
use rocket::State;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use crate::WorldDir;

#[derive(Serialize)]
pub struct Stats(Vec<PlayerStat>);

#[derive(Serialize)]
struct PlayerStat {
    success: bool,
    uuid: String,
    stat: u64,
    error_message: Option<String>,
}

impl PlayerStat {
    pub fn error_message(self, error_message: String) -> Self {
        PlayerStat {
            error_message: Some(error_message),
            ..self
        }
    }
}

#[get("/stats?<uuid>&<stat_type>&<stat_name>")]
pub fn handle_stats(
    mut uuid: String,
    stat_type: String,
    stat_name: String,
    world_dir: State<WorldDir>
) -> Json<Stats> {
    if &uuid == "all" {
        let mut all_stats = vec![];

        let files = match fs::read_dir(format!("{}/stats", world_dir.path)) {
            Ok(f) => f,
            Err(e) => return Json(Stats(vec![PlayerStat {
                success: false,
                uuid: uuid,
                stat: 0,
                error_message: Some(e.to_string()),
            }])),
        };
        for file in files {
            // let path_os_str = match file {
            //     Ok(p) => p.path().into_os_string(),
            //     Err(_) => continue
            // };
            // let path_str = match path_os_str.into_string() {
            //     Ok(s) => s,
            //     Err(_) => continue
            // };
            // let pos = match path_str.rfind('/') {
            //     Some(p) => p,
            //     None => continue,
            // };
            if let Ok(file) = file {
                if let Ok(path_str) = file.path().into_os_string().into_string() {
                    if let Some(pos) = path_str.rfind('/') {
                        let player_uuid = &path_str[pos + 1..path_str.len() - 5];
                        all_stats.push(get_stat(player_uuid.to_string(), stat_type.clone(), stat_name.clone(), world_dir.path));
                    }
                }
            }
        }
        Json(Stats(all_stats))
    } else {
        if !uuid.contains("-") {
            uuid = format!("{}-{}-{}-{}-{}",
                &uuid[0..8], &uuid[8..12], &uuid[12..16], &uuid[16..20], &uuid[20..32]);
        }
        Json(Stats(vec![get_stat(uuid, stat_type, stat_name, world_dir.path)]))
    }
}

fn get_stat(uuid: String, stat_type: String, stat_name: String, world_dir: &str) -> PlayerStat {
    let failure = PlayerStat {
        success: false,
        uuid: uuid.clone(),
        stat: 0,
        error_message: Some(String::from("")),
    };

    let stats_str = match fs::read_to_string(format!("{}/stats/{}.json", world_dir, uuid)) {
        Ok(s) => s,
        Err(e) => return failure.error_message(format!("Error: {}, this player probably has never logged on the server", e))
    };
    let stats_json: Value = match serde_json::from_str(&stats_str) {
        Ok(v) => v,
        Err(e) => return failure.error_message(format!("Error: {}", e))
    };
    let value: u64 = match stats_json["stats"][stat_type][stat_name].as_u64() {
        Some(s) => s,
        None => return failure.error_message(String::from("Error: couldn't find this stat"))
    };

    PlayerStat {
        success: true,
        uuid: uuid,
        stat: value,
        error_message: None,
    }
}

pub mod routes;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Stat {
    #[serde(rename(serialize = "type"))]
    typ: String,
    name: String,
    value: u64,
}

#[derive(Serialize, Debug)]
pub struct Player {
    uuid: String,
    username: String,
}

#[derive(Serialize, Debug)]
pub struct PlayerStat {
    player: Player,
    stat: Stat,
}

#[derive(Serialize, Debug)]
pub struct PlayerStats {
    player: Player,
    stats: Vec<Stat>,
}

#[derive(Serialize, Debug)]
pub struct PlayerStatValue {
    player: Player,
    stat_value: u64,
}

#[derive(Serialize, Debug)]
struct GlobalStats {
    stat: Stat,
    player_stats: Vec<PlayerStatValue>,
}

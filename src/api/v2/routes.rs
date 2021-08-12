use rocket::State;
use crate::{
    api::{
        api_response::{
            ApiResponse,
            ApiSuccess,
        },
        v2::*,
    },
    username_cache::UsernameCache,
};

#[get("/stats?<stat_type>&<stat_name>")]
pub fn global_stats(
    stat_type: String,
    stat_name: String,
    usernames: State<UsernameCache>,
) -> ApiResponse<GlobalStats> {
    Ok(ApiSuccess(
        GlobalStats {
            stat: Stat {
                typ: stat_type,
                name: stat_name,
                value: None,
            },
            player_stats: vec![]
        }
    ))
}

#[get("/stats/<uuid>?<stat_type>", rank = 1)]
pub fn player_stats(
    uuid: String,
    stat_type: String,
    usernames: State<UsernameCache>,
) -> ApiResponse<PlayerStats> {
    unimplemented!();
}

#[get("/stats/<uuid>?<stat_type>&<stat_name>", rank = 2)]
pub fn player_stat(
    uuid: String,
    stat_type: String,
    stat_name: Option<String>,
    usernames: State<UsernameCache>,
) -> ApiResponse<PlayerStat> {
    unimplemented!();
}

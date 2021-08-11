use rocket::State;
use crate::{
    api::{
        api_response::ApiResponse,
        v2::{
            PlayerStat,
            PlayerStats,
            GlobalStats,
        },
    },
    UsernameCache,
};

#[get("/stats?<stat_type>&<stat_name>")]
pub fn global_stats(
    stat_type: String,
    stat_name: String,
    usernames: State<UsernameCache>,
) -> ApiResponse<GlobalStats> {
    unimplemented!();
}

#[get("/stats?<uuid>&<stat_type>")]
pub fn player_stats(
    uuid: String,
    stat_type: String,
) -> ApiResponse<PlayerStats> {
    unimplemented!();
}

#[get("/stats/<uuid>?<stat_type>&<stat_name>")]
pub fn player_stat(
    uuid: String,
    stat_type: String,
    stat_name: Option<String>,
) -> ApiResponse<PlayerStat> {
    unimplemented!();
}

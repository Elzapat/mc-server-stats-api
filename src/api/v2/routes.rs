use rocket::State;
use crate::UsernameCache;

#[get("/stats?<stat_type>&<stat_name>")]
pub fn global_stats(stat_type: String, stat_name: String, usernames: State<UsernameCache>) -> String {
    "".to_string()
}

#[get("/stats/<uuid>?<stat_type>&<stat_name>")]
pub fn player_stats(uuid: String, stat_type: String, stat_name: Option<String>) -> String {
    "".to_string()
}

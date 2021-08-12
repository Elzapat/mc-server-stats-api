use std::collections::HashMap;
use serde::Deserialize;

pub struct UsernameCache {
    cache: HashMap<String, String>,
}

impl UsernameCache {
    pub fn new() -> Self {
        UsernameCache {
            cache: HashMap::new(),
        }
    }

    pub async fn refresh_cache(&mut self) {
        let mut all_uuids = vec![];
        for (uuid, _) in self.cache.iter() {
            all_uuids.push(uuid.clone());
        }

        for uuid in all_uuids {
            let _ = self.add(uuid);
        }
    }

    pub async fn add(&mut self, uuid: String) -> reqwest::Result<()> {
        #[derive(Deserialize)]
        struct Username {
            name: String,
        }

        const API_ADDRESS: &str = "https://api.mojang.com";

        let request_url = format!("{}/user/profiles/{}/names", API_ADDRESS, uuid);

        let usernames = reqwest::get(request_url)
            .await?
            .json::<Vec<Username>>()
            .await?;

        self.cache.insert(uuid, usernames[0].name.to_owned());

        Ok(())
    }
}

use serde::{Deserialize, Serialize};

use crate::errors::ThemeParksError;
use crate::themeparks::{Client, Endpoint, ThemeParksResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityLiveData {
    pub id: String,

    pub name: String,

    #[serde(rename = "entityType")]
    pub entity_type: String,

    pub status: String,

    #[serde(rename = "lastUpdated")]
    pub last_updated: String,

    #[serde(default = "default_showtimes ")]
    pub showtimes: Vec<LiveShowTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveShowTime {
    pub r#type: String,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,
}

fn default_showtimes() -> Vec<LiveShowTime> {
    vec![]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityLiveDataResponse {
    pub id: String,
    pub name: String,

    #[serde(rename = "entityType")]
    pub entity_type: String,

    #[serde(rename = "liveData")]
    pub live_data: Vec<EntityLiveData>,
}

struct EntityLiveDataEndpoint;

impl Endpoint for EntityLiveDataEndpoint {
    type Output = EntityLiveDataResponse;
}

pub struct Entity<'a> {
    pub client: &'a Client,
}

impl Entity<'_> {
    pub fn get_live_data(
        &self,
        entity_id: &str,
    ) -> Result<ThemeParksResponse<EntityLiveDataResponse>, ThemeParksError> {
        let path = format!("/entity/{}/live", entity_id);
        self.client.get::<EntityLiveDataEndpoint>(&*path)
    }
}

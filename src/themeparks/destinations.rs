use serde::{Deserialize, Serialize};

use crate::errors::ThemeParksError;
use crate::themeparks::{Client, Endpoint, ThemeParksResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct Destination {
    id: String,
    name: String,
    slug: String,
    parks: Vec<Park>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Park {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinationsResponse {
    destinations: Vec<Destination>
}

struct DestinationsEndpoint;

impl Endpoint for DestinationsEndpoint {
    type Output = DestinationsResponse;
}

pub struct Destinations<'a> {
    pub client: &'a Client,
}

impl Destinations<'_> {
    pub fn list_destinations(&self) -> Result<ThemeParksResponse<DestinationsResponse>, ThemeParksError> {
        self.client.get::<DestinationsEndpoint>("/destinations")
    }
}

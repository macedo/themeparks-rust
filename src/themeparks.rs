use crate::errors::ThemeParksError;
use crate::themeparks::destinations::Destinations;
use crate::themeparks::entity::Entity;

use serde;
use serde::de::DeserializeOwned;
use serde_json::Value;
use ureq::{Error, Request, Response};

pub mod destinations;
pub mod entity;

const VERSION: &str = "0.1.1";
const DEFAULT_USER_AGENT: &str = "themepark-rust/";

const API_VERSION: &str = "v1";
const DEFAULT_BASE_URL: &str = "https://api.themeparks.wiki";

pub struct Client {
    agent: ureq::Agent,
    base_url: String,
    user_agent: String,
}

pub trait Endpoint {
    type Output: DeserializeOwned;
}

#[derive(Debug)]
pub struct ThemeParksResponse<T> {
    // The HTTP Status Code
    pub status: u16,

    /// The object or a Vec<T> of objects (the type `T` will depend on the endpoint).
    pub data: Option<T>,

    /// The body as a JSON `Value`
    pub body: Option<Value>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            agent: ureq::agent(),
            base_url: String::from(DEFAULT_BASE_URL),
            user_agent: DEFAULT_USER_AGENT.to_owned() + VERSION,
        }
    }

    pub fn destinations(&self) -> Destinations {
        Destinations { client: self }
    }

    pub fn entity(&self) -> Entity {
        Entity { client: self }
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = String::from(url);
    }

    pub fn versionend_url(&self) -> String {
        let mut url = String::from(&self.base_url);
        url.push('/');
        url.push_str(API_VERSION);
        url
    }

    pub fn get<E: Endpoint>(
        &self,
        path: &str,
    ) -> Result<ThemeParksResponse<E::Output>, ThemeParksError> {
        self.call::<E>(self.build_get_request(path))
    }

    fn build_get_request(&self, path: &str) -> Request {
        self.agent
            .get(&*self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept-Header", "application/json")
    }

    fn call<E: Endpoint>(
        &self,
        request: Request,
    ) -> Result<ThemeParksResponse<E::Output>, ThemeParksError> {
        println!("[{}] => {}", request.method().to_uppercase(), request.url());
        self.proccess_response::<E>(request.call())
    }

    fn proccess_response<E: Endpoint>(
        &self,
        result: Result<Response, Error>,
    ) -> Result<ThemeParksResponse<E::Output>, ThemeParksError> {
        println!("{:#?}", result);
        match result {
            Ok(response) => Self::build_themeparks_response::<E>(response),
            Err(Error::Status(code, response)) => {
                Err(ThemeParksError::parse_response(code, response))
            }
            Err(Error::Transport(transport)) => Err(ThemeParksError::parse_transport(transport)),
        }
    }

    fn build_themeparks_response<E: Endpoint>(
        response: Response,
    ) -> Result<ThemeParksResponse<E::Output>, ThemeParksError> {
        let status = response.status();

        let json = response
            .into_json::<Value>()
            .map_err(|e| ThemeParksError::Deserialization(e.to_string()))?;

        let data = serde_json::from_value(json.clone())
            .map_err(|e| ThemeParksError::Deserialization(e.to_string()))?;

        let body = serde_json::from_value(json)
            .map_err(|e| ThemeParksError::Deserialization(e.to_string()))?;

        Ok(ThemeParksResponse { status, data, body })
    }

    fn url(&self, path: &str) -> String {
        let mut url = self.versionend_url();
        url.push_str(path);
        url
    }
}

impl Default for Client {
    fn default() -> Self {
        Client::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::themeparks::{Client, API_VERSION, DEFAULT_BASE_URL, DEFAULT_USER_AGENT, VERSION};

    #[test]
    fn creates_a_client() {
        let client = Client::new();

        assert_eq!(client.base_url, DEFAULT_BASE_URL);
        assert_eq!(client.user_agent, DEFAULT_USER_AGENT.to_owned() + VERSION);
    }

    #[test]
    fn can_change_base_url() {
        let mut client = Client::new();
        let url = "https://github.com";
        client.set_base_url(url);

        assert_eq!(client.base_url, url);
    }

    #[test]
    fn versionend_url() {
        let client = Client::new();

        assert_eq!(
            client.versionend_url(),
            format!("{}/{}", DEFAULT_BASE_URL, API_VERSION)
        );
    }
}

const VERSION: &str = "0.1.0";
const DEFAULT_USER_AGENT: &str = "themepark-rust/";

const API_VERSION: &str = "v1";
const DEFAULT_BASE_URL: &str = "https://api.themeparks.wiki";

pub struct Client {
    base_url: String,
    user_agent: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: String::from(DEFAULT_BASE_URL),
            user_agent: DEFAULT_USER_AGENT.to_owned() + VERSION,
        }
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
}

impl Default for Client {
    fn default() -> Self {
        Client::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, DEFAULT_BASE_URL, DEFAULT_USER_AGENT, VERSION, API_VERSION};


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

        assert_eq!(client.versionend_url(), format!("{}/{}", DEFAULT_BASE_URL, API_VERSION));
    }
}
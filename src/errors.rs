use serde::{Deserialize, Serialize};
use thiserror::Error;
use ureq::{Response, Transport};

#[derive(Error, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum ThemeParksError {
    #[error("Deserialization Error {0}")]
    Deserialization(String),

    #[error("Transport Error - {0}({1})")]
    Transport(String, String),
}

impl ThemeParksError {
    pub fn parse_response(code: u16, response: Response) -> ThemeParksError {
        println!("{}", code);
        Self::Transport(
            response.status().to_string(),
            response.status_text().to_string(),
        )
    }

    pub fn parse_transport(transport: Transport) -> ThemeParksError {
        Self::Transport(transport.to_string(), transport.kind().to_string())
    }
}

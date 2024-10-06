use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod events;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubContentObject {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i32,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    pub r#type: String,
    pub content: Option<String>,
    pub encoding: String,
    pub _links: HashMap<String, String>,
}

impl GitHubContentObject {
    pub fn decoded_content(&self) -> Option<String> {
        use base64::Engine;
        self.content.as_ref().map(|c| {
            let mut content = c.as_bytes().to_owned();
            content.retain(|b| !b" \n\t\r\x0b\x0c".contains(b));
            let c = base64::prelude::BASE64_STANDARD
                .decode(content)
                .expect("could not decode github content");
            String::from_utf8_lossy(&c).into_owned()
        })
    }
}
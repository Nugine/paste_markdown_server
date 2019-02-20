use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub time: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResp {
    pub location: String,
}

impl Post {
    pub fn empty() -> Self {
        Self {
            title: String::from(""),
            author: String::from(""),
            time: 0,
            content: String::from(""),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }
}

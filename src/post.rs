use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResp {
    pub location: String,
}

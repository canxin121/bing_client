use super::bot_easy_resp_type::{Image, SourceAttribution};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EasyMsg {
    pub author: String,
    pub text: String,
    pub images: Vec<Image>,
    pub sources: Vec<SourceAttribution>,
    pub suggest_replys: Vec<String>,
}

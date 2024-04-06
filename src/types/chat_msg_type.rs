use super::bot_easy_resp_type::{Image, SourceAttribution};
use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct EasyMsg {
    pub author: String,
    pub text: String,
    pub images: Vec<Image>,
    pub sources: Vec<SourceAttribution>,
    pub suggest_replys: Vec<String>,
}

impl Display for EasyMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.text)?;
        if !self.images.is_empty() {
            write!(f, "\nImages:\n\n")?;
            for image in &self.images {
                write!(f, "{}\n", image)?;
            }
        }
        if !self.sources.is_empty() {
            write!(f, "\nSources:\n\n")?;
            for (index, source) in self.sources.iter().enumerate() {
                write!(f, "{index}. {}\n", source)?;
            }
        }
        if !self.suggest_replys.is_empty() {
            write!(f, "\nSuggest Replys:\n\n")?;
            for (index, suggest) in self.suggest_replys.iter().enumerate() {
                write!(f, "{index}. {}\n", suggest)?;
            }
        }
        Ok(())
    }
}

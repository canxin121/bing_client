use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Image {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Limit {
    #[serde(rename = "maxNumUserMessagesInConversation")]
    pub max_num_user_messages: u8,
    #[serde(rename = "numUserMessagesInConversation")]
    pub num_user_messages: u8,
    #[serde(rename = "maxNumLongDocSummaryUserMessagesInConversation")]
    pub max_num_long_doc_summary_user_messages: u32,
    #[serde(rename = "numLongDocSummaryUserMessagesInConversation")]
    pub num_long_doc_summary_user_messages: u32,
}

impl Display for Limit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Limit: {} of {}.",
            self.num_user_messages, self.max_num_user_messages
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SourceAttribution {
    #[serde(rename = "providerDisplayName")]
    pub display_name: Option<String>,
    #[serde(rename = "seeMoreUrl")]
    pub see_more_url: Option<String>,
    pub image: Option<Image>,
}

impl Display for SourceAttribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_name = {
            if let Some(s) = &self.display_name {
                s
            } else {
                if let Some(s) = &self.see_more_url {
                    s
                } else {
                    "Unknown"
                }
            }
        };
        let see_more_url = {
            if let Some(s) = &self.see_more_url {
                s
            } else {
                "Unknown"
            }
        };
        let mut s = format!("[{}]({})", display_name, see_more_url);
        if let Some(image) = &self.image {
            s.push_str(&format!("\n![{}]({})", image.name, image.url));
        }
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BotResp {
    Text(String),
    SuggestReply(Vec<String>),
    Notice(String),
    Image(Vec<Image>),
    Apology(String),
    SourceAttribution(Vec<SourceAttribution>),
    Limit(Limit),
}

impl fmt::Display for BotResp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(content) => write!(f, "{}", content),
            Self::SuggestReply(content) => {
                let mut s = String::new();
                for (index, suggest) in content.iter().enumerate() {
                    s += &format!("{index}: {suggest}");
                }
                write!(f, "{}", s)
            }
            Self::Notice(content) => write!(f, "{}", content),
            Self::Image(images) => {
                let mut rst = String::new();
                for image in images {
                    rst += &format!("![{}]({})\n", image.name, image.url);
                }
                write!(f, "{}", rst)
            }
            Self::Apology(content) => write!(f, "{}", content),
            Self::SourceAttribution(sources) => {
                let mut rst = String::new();
                for source in sources {
                    rst += &(source.to_string() + "\n");
                }
                write!(f, "{}", rst)
            }
            Self::Limit(content) => write!(f, "{}", content),
        }
    }
}

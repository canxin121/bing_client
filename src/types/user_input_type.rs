use std::fmt::Display;

use chrono::{DateTime, Utc};
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{client::BingClient, utils::image_base64::Image, vec_string};

use super::{chat_type::Chat, plugin_type::Plugin};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Arguments {
    pub source: String,
    pub optionsSets: OptionsSets,
    pub allowedMessageTypes: AllowedMessageTypes,
    pub sliceIds: SliceIds,
    pub verbosity: String,
    pub scenario: String,
    pub plugins: Vec<Plugin>,
    pub traceId: String,
    pub conversationHistoryOptionsSets: ConversationHistoryOptionsSets,
    pub gptId: String,
    pub isStartOfSession: bool,
    pub requestId: String,
    pub message: Message,
    pub tone: String,
    pub extraExtensionParameters: ExtraExtensionParameters,
    pub spokenTextMode: String,
    pub conversationId: String,
    pub participant: Participant,
}

impl Arguments {
    pub fn build(
        tone: Tone,
        plugins: Vec<Plugin>,
        uuid: String,
        text_message: String,
        image_url: Option<String>,
        chat: &Chat,
        client: &BingClient,
    ) -> Arguments {
        Arguments {
            source: "cib".to_string(),
            optionsSets: OptionsSets::from_tone(&tone),
            allowedMessageTypes: AllowedMessageTypes::from_tone(&tone),
            sliceIds: SliceIds::from_tone(&tone),
            verbosity: "verbose".to_string(),
            scenario: "SERP".to_string(),
            plugins: plugins,
            traceId: {
                let mut rng = rand::thread_rng();
                let mut rand_buf: [u8; 16] = [0; 16];
                rng.fill_bytes(&mut rand_buf);
                hex::encode(rand_buf)
            },
            conversationHistoryOptionsSets: ConversationHistoryOptionsSets::default(),
            gptId: "copilot".to_string(),
            isStartOfSession: true,
            requestId: uuid.clone(),
            message: Message::build(text_message, image_url, uuid),
            tone: tone.to_string(),
            extraExtensionParameters: ExtraExtensionParameters::default(),
            spokenTextMode: "None".to_string(),
            conversationId: chat.conversation_id.clone(),
            participant: Participant {
                id: client.client_id.clone(),
            },
        }
    }
}

impl OptionsSets {
    fn from_tone(tone: &Tone) -> Self {
        match tone {
            Tone::Creative => Self::creative(),
            Tone::Balanced => Self::balanced(),
            Tone::Precise => Self::precise(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OptionsSets(pub Vec<String>);

impl OptionsSets {
    pub fn creative() -> OptionsSets {
        OptionsSets(vec_string![
            "nlu_direct_response_filter",
            "deepleo",
            "disable_emoji_spoken_text",
            "responsible_ai_policy_235",
            "enablemm",
            "dv3sugg",
            "autosave",
            "iyxapbing",
            "iycapbing",
            "h3imaginative",
            "clgalileo",
            "enbela",
            "gndlogcf",
            "saisgrdcf",
            "codeintfilev2",
            "gptv1desc2",
            "eredirecturl",
            "enable_user_consent",
            "fluxmemcst",
            "ldsummary",
            "ldqa",
            "sdretrieval",
            "nosearchall"
        ])
    }
    pub fn balanced() -> OptionsSets {
        OptionsSets(vec_string![
            "nlu_direct_response_filter",
            "deepleo",
            "disable_emoji_spoken_text",
            "responsible_ai_policy_235",
            "enablemm",
            "dv3sugg",
            "autosave",
            "iyxapbing",
            "iycapbing",
            "galileo",
            "enbela",
            "gndlogcf",
            "saisgrdcf",
            "codeintfilev2",
            "gptv1desc2",
            "eredirecturl",
            "enable_user_consent",
            "fluxmemcst",
            "ldsummary",
            "ldqa",
            "sdretrieval",
            "nosearchall"
        ])
    }
    pub fn precise() -> OptionsSets {
        OptionsSets(vec_string![
            "nlu_direct_response_filter",
            "deepleo",
            "disable_emoji_spoken_text",
            "responsible_ai_policy_235",
            "enablemm",
            "dv3sugg",
            "autosave",
            "iyxapbing",
            "iycapbing",
            "h3precise",
            "enbela",
            "gndlogcf",
            "saisgrdcf",
            "codeintfilev2",
            "gptv1desc2",
            "eredirecturl",
            "enable_user_consent",
            "fluxmemcst",
            "ldsummary",
            "ldqa",
            "sdretrieval",
            "clgalileo",
            "nosearchall"
        ])
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AllowedMessageTypes(pub Vec<String>);

impl AllowedMessageTypes {
    fn from_tone(tone: &Tone) -> Self {
        match tone {
            Tone::Creative => Self::creative(),
            Tone::Balanced => Self::balanced(),
            Tone::Precise => Self::precise(),
        }
    }
}

impl AllowedMessageTypes {
    pub fn creative() -> AllowedMessageTypes {
        AllowedMessageTypes(vec_string![
            "ActionRequest",
            "Chat",
            "ConfirmationCard",
            "Context",
            "InternalSearchQuery",
            "InternalSearchResult",
            "Disengaged",
            "InternalLoaderMessage",
            "Progress",
            "RenderCardRequest",
            "RenderContentRequest",
            "AdsQuery",
            "SemanticSerp",
            "GenerateContentQuery",
            "SearchQuery",
            "GeneratedCode",
            "InternalTasksMessage",
            "Disclaimer"
        ])
    }
    pub fn balanced() -> AllowedMessageTypes {
        AllowedMessageTypes(vec_string![
            "ActionRequest",
            "Chat",
            "ConfirmationCard",
            "Context",
            "InternalSearchQuery",
            "InternalSearchResult",
            "Disengaged",
            "InternalLoaderMessage",
            "Progress",
            "RenderCardRequest",
            "RenderContentRequest",
            "AdsQuery",
            "SemanticSerp",
            "GenerateContentQuery",
            "SearchQuery",
            "GeneratedCode",
            "InternalTasksMessage",
            "Disclaimer"
        ])
    }
    pub fn precise() -> AllowedMessageTypes {
        AllowedMessageTypes(vec_string![
            "ActionRequest",
            "Chat",
            "ConfirmationCard",
            "Context",
            "InternalSearchQuery",
            "InternalSearchResult",
            "Disengaged",
            "InternalLoaderMessage",
            "Progress",
            "RenderCardRequest",
            "RenderContentRequest",
            "AdsQuery",
            "SemanticSerp",
            "GenerateContentQuery",
            "SearchQuery",
            "GeneratedCode",
            "InternalTasksMessage",
            "Disclaimer"
        ])
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SliceIds(pub Vec<String>);

impl SliceIds {
    pub fn creative() -> SliceIds {
        SliceIds(vec_string![
            "inputdes3cf",
            "0311tccs0",
            "gldidentitycf",
            "visperf",
            "visibilityperf",
            "engmhcf",
            "qnav2table1",
            "fltltstcf",
            "cntralignc",
            "streamw",
            "bcece403t",
            "romiccf",
            "rwt2",
            "cmcpupsalltf",
            "advtokc",
            "315endltall",
            "shopgptctrl",
            "320enbela",
            "0215wcrwippsr",
            "0328scnd",
            "fpsrhsticy",
            "0306flowvaca",
            "gptsmobile0",
            "328fixissuess0",
            "saisgrds0",
            "228pyfile",
            "ecipc",
            "kcclickthrucf",
            "cacfrwebt2cf",
            "sswebtop2cf"
        ])
    }
    pub fn balanced() -> SliceIds {
        SliceIds(vec_string![
            "inputdes3cf",
            "0311tccs0",
            "gldidentitycf",
            "visperf",
            "visibilityperf",
            "engmhcf",
            "qnav2table1",
            "fltltstcf",
            "cntralignc",
            "streamw",
            "bcece403t",
            "romiccf",
            "rwt2",
            "cmcpupsalltf",
            "advtokc",
            "315endltall",
            "shopgptctrl",
            "320enbela",
            "0215wcrwippsr",
            "0328scnd",
            "fpsrhsticy",
            "0306flowvaca",
            "gptsmobile0",
            "328fixissuess0",
            "saisgrds0",
            "228pyfile",
            "ecipc",
            "kcclickthrucf",
            "cacfrwebt2cf",
            "sswebtop2cf"
        ])
    }
    pub fn precise() -> SliceIds {
        SliceIds(vec_string![
            "inputdes3cf",
            "0311tccs0",
            "gldidentitycf",
            "visperf",
            "visibilityperf",
            "engmhcf",
            "qnav2table1",
            "fltltstcf",
            "cntralignc",
            "streamw",
            "bcece403t",
            "romiccf",
            "rwt2",
            "cmcpupsalltf",
            "advtokc",
            "315endltall",
            "shopgptctrl",
            "320enbela",
            "0215wcrwippsr",
            "0328scnd",
            "fpsrhsticy",
            "0306flowvaca",
            "gptsmobile0",
            "328fixissuess0",
            "saisgrds0",
            "228pyfile",
            "ecipc",
            "kcclickthrucf",
            "cacfrwebt2cf",
            "sswebtop2cf"
        ])
    }
}
impl SliceIds {
    fn from_tone(tone: &Tone) -> Self {
        match tone {
            Tone::Creative => Self::creative(),
            Tone::Balanced => Self::balanced(),
            Tone::Precise => Self::precise(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationHistoryOptionsSets(pub Vec<String>);
impl Default for ConversationHistoryOptionsSets {
    fn default() -> Self {
        ConversationHistoryOptionsSets(vec_string!["autosave", "savemem", "uprofupd", "uprofgen"])
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LocationHints {
    pub SourceType: i32,
    pub RegionType: i32,
    pub Center: Center,
    pub Radius: i32,
    pub Name: String,
    pub Accuracy: i32,
    pub FDConfidence: i32,
    pub CountryName: String,
    pub CountryConfidence: i32,
    pub Admin1Name: String,
    pub PopulatedPlaceName: String,
    pub PopulatedPlaceConfidence: i32,
    pub PostCodeName: String,
    pub UtcOffset: i32,
    pub Dma: i32,
}

impl Default for LocationHints {
    fn default() -> Self {
        LocationHints {
            SourceType: 1,
            RegionType: 2,
            Center: Center {
                Latitude: 1.3056000471115112,
                Longitude: 103.822998046875,
            },
            Radius: 24902,
            Name: "Singapore, Central Singapore Community Development Council".to_string(),
            Accuracy: 24902,
            FDConfidence: 0,
            CountryName: "Singapore".to_string(),
            CountryConfidence: 8,
            Admin1Name: "Central Singapore Community Development Council".to_string(),
            PopulatedPlaceName: "Singapore".to_string(),
            PopulatedPlaceConfidence: 0,
            PostCodeName: "247964".to_string(),
            UtcOffset: 8,
            Dma: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Center {
    Latitude: f64,
    Longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Message {
    pub locale: String,
    pub market: String,
    pub region: String,
    pub location: String,
    pub locationHints: Vec<LocationHints>,
    pub userIpAddress: String,
    pub timestamp: String,
    pub author: String,
    pub inputMethod: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imageUrl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originalImageUrl: Option<String>,
    pub messageType: String,
    pub requestId: String,
    pub messageId: String,
}

impl Message {
    pub fn build(text_message: String, image_url: Option<String>, uuid: String) -> Self {
        Message {
            locale: "en-US".to_string(),
            market: "en-US".to_string(),
            region: "US".to_string(),
            location: "lat:47.639557;long:-122.128159;re=1000m;".to_string(),
            locationHints: vec![LocationHints::default()],
            userIpAddress: "13.212.211.208".to_string(),
            timestamp: {
                let dt: DateTime<Utc> = Utc::now();
                format!("{}", dt.to_rfc3339())
            },
            author: "user".to_string(),
            inputMethod: "Keyboard".to_string(),
            text: text_message,
            imageUrl: image_url.clone(),
            originalImageUrl: image_url,
            messageType: "Chat".to_string(),
            requestId: uuid.clone(),
            messageId: uuid,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraExtensionParameters {
    #[serde(rename = "gpt-creator-persona")]
    pub gpt_creator_persona: GptCreatorPersona,
}

impl Default for ExtraExtensionParameters {
    fn default() -> Self {
        Self {
            gpt_creator_persona: GptCreatorPersona {
                personaId: "copilot".to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GptCreatorPersona {
    pub personaId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Tone {
    Creative,
    Balanced,
    Precise,
}

impl Tone {
    pub fn build_by_name(name: &str) -> Option<Self> {
        match name {
            "Creative" => Some(Self::Creative),
            "Balanced" => Some(Self::Balanced),
            "Precise" => Some(Self::Precise),
            _ => None,
        }
    }
}

impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Self::Creative => "Creative",
                Self::Balanced => "Balanced",
                Self::Precise => "Precise",
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserInput {
    pub arguments: Vec<Arguments>,
    pub invocationId: String,
    pub target: String,
    pub r#type: i32,
}

impl UserInput {
    pub async fn build(
        text_message: String,
        image_attach: Option<Image>,
        tone: Tone,
        plugins: Vec<Plugin>,
        chat: &Chat,
        client: &BingClient,
    ) -> Result<Self, anyhow::Error> {
        let uuid = Uuid::new_v4().to_string();
        let image_url = match image_attach {
            Some(image) => Some(client.gen_upload_image_url(image, chat).await?),
            None => None,
        };
        let final_plugins = {
            if !chat.plugins.is_empty() {
                chat.plugins.clone()
            } else {
                plugins
            }
        };
        let final_tone = {
            if let Some(name) = &chat.tone {
                {}
                if let Some(tone) = Tone::build_by_name(&name) {
                    tone
                } else {
                    tone
                }
            } else {
                tone
            }
        };
        Ok(UserInput {
            arguments: vec![Arguments::build(
                final_tone,
                final_plugins,
                uuid,
                text_message,
                image_url,
                chat,
                client,
            )],
            invocationId: 0.to_string(),
            target: "chat".to_string(),
            r#type: 4,
        })
    }
}

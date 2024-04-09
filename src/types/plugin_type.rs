use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plugin {
    pub id: String,
    #[serde(default = "defalut_category")]
    pub category: u32,
}

fn defalut_category() -> u32 {
    1
}

impl Plugin {
    pub fn get_name(&self) -> String {
        match self.id.as_str() {
            "c310c353-b9f0-4d76-ab0d-1dd5e979cf68" => "Search".to_string(),
            "46664d33-1591-4ce8-b3fb-ba1022b66c11" => "Instacart".to_string(),
            "d6be744c-2bd9-432f-95b7-76e103946e34" => "Kayak".to_string(),
            "543a7b1b-ebc6-46f4-be76-00c202990a1b" => "OpenTable".to_string(),
            "39e3566a-d481-4d99-82b2-6d739b1e716e" => "Shop".to_string(),
            "22b7f79d-8ea4-437e-b5fd-3e21f09f7bc1" => "Suno".to_string(),
            _ => "Unknown Plugin".to_string(),
        }
    }

    pub fn build_by_name(name: &str) -> Option<Plugin> {
        match name {
            "Search" => Some(Plugin::search()),
            "Instacart" => Some(Plugin::instacart()),
            "Kayak" => Some(Plugin::kayak()),
            "OpenTable" => Some(Plugin::open_table()),
            "Shop" => Some(Plugin::shop()),
            "Suno" => Some(Plugin::suno()),
            _ => None,
        }
    }

    pub fn search() -> Plugin {
        Plugin {
            id: "c310c353-b9f0-4d76-ab0d-1dd5e979cf68".to_string(),
            category: 1,
        }
    }
    pub fn instacart() -> Plugin {
        Plugin {
            id: "46664d33-1591-4ce8-b3fb-ba1022b66c11".to_string(),
            category: 1,
        }
    }
    pub fn kayak() -> Plugin {
        Plugin {
            id: "d6be744c-2bd9-432f-95b7-76e103946e34".to_string(),
            category: 1,
        }
    }
    pub fn klarna() -> Plugin {
        Plugin {
            id: "5f143ea3-8c80-4efd-9515-185e83b7cf8a".to_string(),
            category: 1,
        }
    }
    pub fn open_table() -> Plugin {
        Plugin {
            id: "543a7b1b-ebc6-46f4-be76-00c202990a1b".to_string(),
            category: 1,
        }
    }
    pub fn shop() -> Plugin {
        Plugin {
            id: "39e3566a-d481-4d99-82b2-6d739b1e716e".to_string(),
            category: 1,
        }
    }
    pub fn suno() -> Plugin {
        Plugin {
            id: "22b7f79d-8ea4-437e-b5fd-3e21f09f7bc1".to_string(),
            category: 1,
        }
    }
}

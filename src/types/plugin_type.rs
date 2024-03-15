use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub category: Option<u32>,
}

impl Plugin {
    pub fn search() -> Plugin {
        Plugin {
            id: "c310c353-b9f0-4d76-ab0d-1dd5e979cf68".to_string(),
            category: Some(1),
        }
    }
    pub fn instacart() -> Plugin {
        Plugin {
            id: "46664d33-1591-4ce8-b3fb-ba1022b66c11".to_string(),
            category: Some(1),
        }
    }
    pub fn kayak() -> Plugin {
        Plugin {
            id: "d6be744c-2bd9-432f-95b7-76e103946e34".to_string(),
            category: Some(1),
        }
    }
    pub fn klarna() -> Plugin {
        Plugin {
            id: "5f143ea3-8c80-4efd-9515-185e83b7cf8a".to_string(),
            category: Some(1),
        }
    }
    pub fn open_table() -> Plugin {
        Plugin {
            id: "543a7b1b-ebc6-46f4-be76-00c202990a1b".to_string(),
            category: Some(1),
        }
    }
    pub fn shop() -> Plugin {
        Plugin {
            id: "39e3566a-d481-4d99-82b2-6d739b1e716e".to_string(),
            category: Some(1),
        }
    }
    pub fn suno() -> Plugin {
        Plugin {
            id: "22b7f79d-8ea4-437e-b5fd-3e21f09f7bc1".to_string(),
            category: Some(1),
        }
    }
}

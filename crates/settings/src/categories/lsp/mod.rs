use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LspSettings {
    // placeholder
}

impl Default for LspSettings {
    fn default() -> Self {
        Self {}
    }
}

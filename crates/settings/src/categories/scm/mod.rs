use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScmSettings {
    // placehodler
}

impl Default for ScmSettings {
    fn default() -> Self {
        Self {}
    }
}

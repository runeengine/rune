pub const BRP_LOAD_IMAGE_METHOD: &str = "asset_server.load_image";
pub const BRP_SPAWN_SPRITE_METHOD: &str = "asset_server.spawn_sprite";

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AssetLoadParams {
    pub path: String,
}

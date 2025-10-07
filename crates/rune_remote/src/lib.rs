use rune_remote_brp::{AssetLoadParams, BRP_LOAD_IMAGE_METHOD, BRP_SPAWN_SPRITE_METHOD};

use bevy::prelude::*;
use bevy::remote::http::RemoteHttpPlugin;
use bevy::remote::{BrpResult, RemotePlugin};

pub struct RuneRemotePlugin;
impl Plugin for RuneRemotePlugin {
    fn build(&self, app: &mut App) {
        let remote_plugin = RemotePlugin::default()
            .with_method(BRP_LOAD_IMAGE_METHOD, asset_server_load_image_handler)
            .with_method(BRP_SPAWN_SPRITE_METHOD, spawn_sprite_handler);

        app.add_plugins(remote_plugin);
        app.add_plugins(RemoteHttpPlugin::default());
    }
}

pub fn asset_server_load_image_handler(
    In(params): In<Option<serde_json::Value>>,
    world: &mut World,
) -> BrpResult {
    let asset_load_params = serde_json::from_value::<AssetLoadParams>(params.unwrap()).unwrap();
    let asset_server = world.resource_mut::<AssetServer>();
    let handle = asset_server.load::<bevy::image::Image>(asset_load_params.path);
    Ok(serde_json::json!({ "asset_id": handle.id() }))
}

pub fn spawn_sprite_handler(
    In(params): In<Option<serde_json::Value>>,
    world: &mut World,
) -> BrpResult {
    let asset_load_params = serde_json::from_value::<AssetLoadParams>(params.unwrap()).unwrap();
    let asset_server = world.resource_mut::<AssetServer>();
    let handle = asset_server.load::<bevy::image::Image>(asset_load_params.path);
    let sprite = Sprite::from_image(handle);
    let entity = world.spawn(sprite).id();
    Ok(serde_json::json!({ "entity": entity }))
}

mod events;
mod game;
mod input;
mod loading;
mod resources;
mod sync;
mod tick;

use bevy::{prelude::*, render::texture::ImageSettings};

pub const LAUNCHER_TITLE: &str = "MoM Game";

pub fn app() -> App {
    let mut app = App::new();

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(
            bevy_embedded_assets::EmbeddedAssetPlugin,
        )
    });
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(DefaultPlugins);

    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default());

    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .insert_resource(ImageSettings::default_nearest())
    .add_plugin(game::GamePlugin);
    app
}

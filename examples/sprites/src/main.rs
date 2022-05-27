use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("soldjars_of_fortune_ashes.png"),
        sprite: Sprite {
            flip_x: true,
            flip_y: true,
            ..default()
        },
        ..default()
    });
}

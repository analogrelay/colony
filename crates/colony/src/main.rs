use bevy::prelude::*;

mod camera;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Colony".to_string(),
                    width: 1270.0,
                    height: 720.0,
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            }))
        .add_system(camera::movement)
        .run();
}

use bevy::prelude::*;
use setup::setup;
use playground::spawn_playground;

mod setup;
mod playground;

fn main() {
  App::new()
  .add_plugins(DefaultPlugins.set(WindowPlugin
    {
        primary_window:Some(Window{
            title:"2048".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, (setup, spawn_playground).chain())
  .run();
}
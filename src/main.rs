mod systems;
mod structs;

use bevy::prelude::*;
use systems::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, player_systems::spawn_camera)
    .add_systems(Startup, player_systems::spawn_player)
    .add_systems(PostStartup, enemies_systems::spawn_enemies)
    .add_systems(Update, (enemies_systems::move_enemies, 
        bullet_systems::spawn_bullet, 
        bullet_systems::bullet_movement))
    .run();
}


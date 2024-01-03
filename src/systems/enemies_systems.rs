use bevy::{prelude::*, sprite::Anchor};
use crate::structs::*;
use rand::{self, Rng};

pub fn spawn_enemies(mut commands: Commands, player_query: Query<&Transform, (With<player_structs::Player>, Without<enemy_structs::Enemy>)>) {
    let player_transform = player_query.get_single().unwrap();
    let enemies_count = 100;

    for x in 0..enemies_count {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BISQUE,
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    player_transform.translation.x + rand::thread_rng().gen_range(-1000.0..1000.0),
                    player_transform.translation.y + rand::thread_rng().gen_range(-1000.0..1000.0),
                    0.0,
                ),
                ..Default::default()
            },
            enemy_structs::Enemy {},
        ));
    }
    

    println!("Enemy spawned");
}

pub fn move_enemies(
    mut query: Query<(&mut Transform, &enemy_structs::Enemy)>,
    player_query: Query<&Transform, (With<player_structs::Player>, Without<enemy_structs::Enemy>)>,
) {
    let player_transform = player_query.get_single().unwrap();

    for (mut transform, _enemy) in query.iter_mut() {
        let direction = Vec2::new(
            player_transform.translation.x - transform.translation.x,
            player_transform.translation.y - transform.translation.y,
        );

        let normalized_direction = direction.normalize();

        transform.translation.x += normalized_direction.x;
        transform.translation.y += normalized_direction.y;
    }
}

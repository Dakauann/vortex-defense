use bevy::{prelude::*, sprite::Anchor};
use rand::Rng;
use crate::structs::*;

pub fn spawn_bullet(mut commands: Commands, player: Query<&Transform, (With<player_structs::Player>, Without<enemy_structs::Enemy>)>) {
    let player_transform = player.get_single().unwrap();

    commands.spawn(
        (SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                flip_x: false,
                flip_y: false,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform: Transform::from_xyz(
                player_transform.translation.x,
                player_transform.translation.y,
                0.0,
            ),
            ..Default::default()
        },
        bullet_structs::Bullet {
            direction: Vec2::new(0.0, 0.0),
            speed: 10.0,
        },
    )
    );
}

pub fn bullet_movement(mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform, &mut bullet_structs::Bullet)>,
    enemies_query: Query<&Transform, (With<enemy_structs::Enemy>, Without<player_structs::Player>)>,
    player_query: Query<&Transform, (With<player_structs::Player>, Without<enemy_structs::Enemy>)>,
) {
    // takes nearest enemy and moves the bullet towards it, by setting its direction, if it already has a direction, it will continue moving in that direction
    
    let player_transform = player_query.get_single().unwrap();
    let enemies_transform = enemies_query.iter().collect::<Vec<&Transform>>();

    for (entity, mut transform, mut bullet) in bullet_query.iter_mut() {
        if bullet.direction == Vec2::new(0.0, 0.0) {
            let mut nearest_enemy = enemies_transform[0];
            let mut nearest_distance = Vec2::new(
                nearest_enemy.translation.x - transform.translation.x,
                nearest_enemy.translation.y - transform.translation.y,
            ).length();

            for enemy in enemies_transform.iter() {
                let distance = Vec2::new(
                    enemy.translation.x - transform.translation.x,
                    enemy.translation.y - transform.translation.y,
                ).length();

                if distance < nearest_distance {
                    nearest_enemy = enemy;
                    nearest_distance = distance;
                }
            }

            bullet.direction = Vec2::new(
                nearest_enemy.translation.x - transform.translation.x,
                nearest_enemy.translation.y - transform.translation.y,
            ).normalize();
        }

        transform.translation.x += bullet.direction.x * bullet.speed;
        transform.translation.y += bullet.direction.y * bullet.speed;

        if transform.translation.x > player_transform.translation.x + 1000.0 || transform.translation.x < player_transform.translation.x - 1000.0 || transform.translation.y > player_transform.translation.y + 1000.0 || transform.translation.y < player_transform.translation.y - 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}
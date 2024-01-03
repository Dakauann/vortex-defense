use crate::structs::{self, player_structs, enemy_structs, bullet_structs};
use bevy::{prelude::*, window::PrimaryWindow, sprite::Anchor};

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
            (SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    anchor: Anchor::Center,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    flip_x: false,
                    flip_y: false,
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(50.0, 50.0),
                    })
                },
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..Default::default()
            }, structs::player_structs::Player{})
    );
}

pub fn shoot_bullet(mut commands: Commands, player: Query<&Transform, (With<player_structs::Player>, Without<enemy_structs::Enemy>)>) {}
pub fn bullet_movement(mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &mut bullet_structs::Bullet)>,
    mut enemy_query: Query<(Entity, &Transform, &enemy_structs::Enemy)>,
    player_query: Query<&Transform, (With<player_structs::Player>, Without<enemy_structs::Enemy>)>,
) {}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

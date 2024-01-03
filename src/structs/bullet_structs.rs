use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Vec2,
}
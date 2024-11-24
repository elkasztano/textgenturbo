use crate::cli::Cli;
use bevy::prelude::{Component, Query, Res, Time, Transform, Vec3};
use std::f32::consts::TAU;

#[derive(Component)]
pub struct WaveAble {
    pub speed: f32,
    pub ampli: f32,
    pub base_height: f32,
    pub bias: f32,
}

#[derive(Component)]
pub struct CameraMovement;

// simple up and down movement based on sine function
pub fn wave_waveable(mut waveable: Query<(&mut Transform, &WaveAble)>, t: Res<Time>) {
    for (mut tr, w) in &mut waveable {
        let x = (t.elapsed_seconds_wrapped() * w.speed * TAU + w.bias).sin();
        tr.translation.y = w.base_height + x * w.ampli;
    }
}

// elliptic orbit around center point
pub fn move_camera(
    cli: Res<Cli>,
    t: Res<Time>,
    mut query: Query<(&mut Transform, &CameraMovement)>,
) {
    let neg_threshold = cli.frames as f32 / 100.0;
    for (mut transform, _) in &mut query {
        transform.translation.x =
            ((t.elapsed_seconds_wrapped() - neg_threshold) * 0.25).sin() * 12.0;
        transform.translation.z =
            ((t.elapsed_seconds_wrapped() - neg_threshold) * 0.25).cos() * 10.0;
        transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
    }
}

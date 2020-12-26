use bevy::prelude::*;
use bevy::{
    math::{vec3, Vec3},
    render::camera::Camera,
    window::Window,
};

pub fn screen_to_world_point(window: &Window, camera: &Transform, screen_point: &Vec3) -> Vec3 {
    let center = camera.translation.truncate();
    let half_width = (window.width() / 2.0) * camera.scale.x;
    let half_height = (window.height() / 2.0) * camera.scale.y;
    let left = center.x - half_width;
    let bottom = center.y - half_height;
    vec3(
        left + screen_point.x * camera.scale.x,
        bottom + screen_point.y * camera.scale.y,
        0.0,
    )
}

pub fn world_to_screen_point(window: &Window, camera: &Camera, world_point: &Vec3) -> Vec3 {
    vec3(0.0, 0.0, 0.0)
}

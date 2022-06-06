#![allow(dead_code)]

use bevy::math::{ Vec2, Vec3 };

pub fn evaluate_linear_2d(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a + (b - a) * t
}

pub fn evaluate_quadratic_2d(a: Vec2, b: Vec2, c: Vec2, t: f32) -> Vec2 {
    let p0 = evaluate_linear_2d(a, b, t);
    let p1 = evaluate_linear_2d(b, c, t);
    evaluate_linear_2d(p0, p1, t)
}

pub fn evaluate_cubic_2d(a: Vec2, b: Vec2, c: Vec2, d: Vec2, t: f32) -> Vec2 {
    let p0 = evaluate_quadratic_2d(a, b, c, t);
    let p1 = evaluate_quadratic_2d(b, c, d, t);
    evaluate_linear_2d(p0, p1, t)
}

pub fn evaluate_linear_3d(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

pub fn evaluate_quadratic_3d(a: Vec3, b: Vec3, c: Vec3, t: f32) -> Vec3 {
    let p0 = evaluate_linear_3d(a, b, t);
    let p1 = evaluate_linear_3d(b, c, t);
    evaluate_linear_3d(p0, p1, t)
}

pub fn evaluate_cubic_3d(a: Vec3, b: Vec3, c: Vec3, d: Vec3, t: f32) -> Vec3 {
    let p0 = evaluate_quadratic_3d(a, b, c, t);
    let p1 = evaluate_quadratic_3d(b, c, d, t);
    evaluate_linear_3d(p0, p1, t)
}
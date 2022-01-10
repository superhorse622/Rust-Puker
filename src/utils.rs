use ggez::{
    mint::Point2,
    graphics::{Vertex, Rect},
    GameError,
};
use glam::f32::*;
use std::fmt;

#[derive(Debug)]
pub enum Errors {
    UnknownRoomIndex(usize),
    UnknownGridNum(usize),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<GameError> for Errors {
    fn into(self) -> GameError {
        GameError::CustomError(self.to_string())
    }
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Vec2Wrap(pub Vec2);

impl Into<Point2<f32>> for Vec2Wrap {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.0.x,
            y: self.0.y
        }
    }
}

impl Into<Vec2> for Vec2Wrap {
    fn into(self) -> Vec2 {
        self.0
    }
}

impl From<Vec2> for Vec2Wrap {
    fn from(v: Vec2) -> Self {
        Vec2Wrap(v)
    }
}

impl From<[f32; 2]> for Vec2Wrap {
    fn from(a: [f32; 2]) -> Self {
        Vec2Wrap(Vec2::from_slice(&a))
    }
}

impl From<Point2<f32>> for Vec2Wrap {
    fn from(p: Point2<f32>) -> Self {
        Vec2Wrap(Vec2::from_slice(&[p.x, p.y]))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct MyVertex{
    pub pos: Vec2Wrap,
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl Into<Point2<f32>> for MyVertex {
    fn into(self) -> Point2<f32> {
        Point2 {
            x: self.pos.0.x,
            y: self.pos.0.y,
        }
    }
}

impl Into<Vertex> for MyVertex {
    fn into(self) -> Vertex {
        Vertex {
            pos: self.pos.0.to_array(),
            uv: self.uv,
            color: self.color
        }
    }
}

/// Project Cartesian world coordinates to screen coordinates.
///
pub fn world_to_screen_space(sw: f32, sh: f32, point: Vec2) -> Vec2 {
    Vec2::new(
        point.x + sw / 2.,
        -point.y + sh / 2.,
    )
}

/// Transform screen coordinates to Cartesian world coordinates.
///
pub fn screen_to_world_space(sw: f32, sh: f32, point: Vec2) -> Vec2 {
    Vec2::new(
        point.x - sw / 2.,
        -point.y + sh / 2.,
    )
}

/// Checks if two rectangles overlap
///
pub fn rect_vs_rect(r1: &Rect, r2: &Rect) -> bool {
    if r1.x == r1.x + r1.w || r1.y == r1.y + r1.h || r2.x == r2.x + r2.w || r2.y == r2.y + r2.h { return false; }
    
    if r1.x > r2.x + r2.w || r2.x > r1.x + r1.w { return false; }
    if r1.y - r1.h > r2.y || r2.y - r2.h > r1.y { return false; }

    true
}

/// Detects if a ray is intersecting a given rectangle.
/// Long live OneLoneCoder and his tutorials.
///
pub fn ray_vs_rect(ray_origin: &Vec2, ray_dir: &Vec2, target: &Rect, contact_point: &mut Vec2, contact_normal: &mut Vec2, t_hit_near: &mut f32) -> bool {
    let target_pos = Vec2::new(target.x, target.y);
    let target_size = Vec2::new(target.w, target.h);
    let target_pos2 = Vec2::new(target_pos.x + target_size.x, target_pos.y - target_size.y);

    let mut t_near = (target_pos - *ray_origin) / *ray_dir;
    let mut t_far = (target_pos2 - *ray_origin) / *ray_dir;

    if t_far.x.is_nan() || t_far.y.is_nan() { return false; }
    if t_near.x.is_nan() || t_near.y.is_nan() { return false; }

    if t_near.x > t_far.x { std::mem::swap(&mut t_near.x, &mut t_far.x)}
    if t_near.y > t_far.y { std::mem::swap(&mut t_near.y, &mut t_far.y)}

    *t_hit_near = f32::max(t_near.x, t_near.y);
    let t_hit_far = f32::min(t_far.x, t_far.y);

    if t_hit_far < 0. { return false; }

    *contact_point = *ray_origin + *t_hit_near * *ray_dir;

    if t_near.x > t_near.y {
        if ray_dir.x < 0. { *contact_normal = Vec2::new(1., 0.); }
        else { *contact_normal = Vec2::new(-1., 0.); }
    }
    else if t_near.x < t_near.y {
        if ray_dir.y < 0. { *contact_normal = Vec2::new(0., 1.); }
        else { *contact_normal = Vec2::new(0., -1.); }
    }

    true
}

/// Detects intersection between moving rectangle and stationary rectangle.
/// Long live OneLoneCoder and his tutorials.
///
pub fn dynamic_rect_vs_rect(source: &Rect, source_vel: &Vec2, target: &Rect, contact_point: &mut Vec2, contact_normal: &mut Vec2, contact_time: &mut f32, elapsed_time: f32) -> bool { 
    let source_pos = Vec2::new(source.x, source.y);
    let source_size = Vec2::new(source.w, source.h);

    if source_vel.x == 0. && source_vel.y == 0. { return false; }

    let expanded_target = Rect {
        x: target.x - source_size.x / 2.,
        y: target.y + source_size.y / 2.,
        w: target.w + source_size.x,
        h: target.h + source_size.y,
    };

    let source_ray_origin = Vec2::new(source_pos.x + source_size.x / 2., source_pos.y - source_size.y / 2.);

    if ray_vs_rect(&source_ray_origin, source_vel, &expanded_target, contact_point, contact_normal, contact_time) {
        if *contact_time <= 1. { return true; }
    }

    false
}
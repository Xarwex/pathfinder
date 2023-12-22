use bevy::{asset::Handle, sprite::ColorMaterial};
use std::{f32::consts::PI, sync::OnceLock};

use bevy::prelude::Mesh;
pub const GRID_WIDTH: isize = 9;
pub const GRID_HEIGHT: isize = 5;
pub const HALF_WIDTH: isize = GRID_WIDTH / 2;
pub const HALF_HEIGHT: isize = GRID_HEIGHT / 2;

pub const SCALE: isize = 64;
pub const STRAIGHT_ANGLE: f32 = PI / 4.;

pub static QUAD: OnceLock<Handle<Mesh>> = OnceLock::new();
pub static DEFAULT_COLOR: OnceLock<Handle<ColorMaterial>> = OnceLock::new();

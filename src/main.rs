mod component;
mod constants;
mod laser;
mod level;
mod util;
use crate::component::block::Block;
use crate::level::level::Level;
use crate::level::point::Point;
use std::{f32::consts::PI, path::PathBuf};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::HashSet};
use bevy_mod_picking::prelude::*;
use bevy_rapier2d::prelude::*;
use clap::Parser;
use laser::laser_system;
use level::block_type::BlockType;

#[derive(Debug, Parser)]
struct Args {
    level_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let level = level::level::read_level_from_file(&args.level_path).unwrap();
    // let mut collider_set = ColliderSet::new();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(level)
        .add_event::<RotateEvent>()
        .add_event::<RedrawLaserEvent>()
        .add_startup_system(setup)
        .add_system(click_on_block_system)
        // .add_system(resolve_rotation_system)
        .add_system(laser_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,
    // rapier_context: Res<RapierContext>,
) {
    commands.spawn((Camera2dBundle::default(), RaycastPickCamera::default()));
    constants::DEFAULT_COLOR
        .set(materials.add(ColorMaterial::from(Color::NONE)))
        .unwrap();
    constants::QUAD
        .set(meshes.add(Mesh::from(shape::Quad::default())))
        .unwrap();
    let height = level.get_height() as isize;
    let width = level.get_width() as isize;
    for y in 0..height {
        for x in 0..width {
            level.get_grid()[y as usize][x as usize]
                .spawn_block(&mut commands, Point::new_from_tuple((x, height - 1 - y)));
        }
    }
}

fn click_on_block_system(
    mut ev_rotate: EventWriter<RotateEvent>,
    mut ev_redraw: EventWriter<RedrawLaserEvent>,
    mut clicked: Query<(&Interaction, &mut Transform, &mut Block), Changed<Interaction>>,
) {
    for (interaction, mut transform, block) in &mut clicked {
        if matches!(interaction, Interaction::Clicked) {
            ev_rotate.send(RotateEvent(block.clone()));
            transform.rotate_local_z(constants::STRAIGHT_ANGLE * 2.)
        }
    }
}

fn resolve_rotation_system(
    mut ev_rotate: EventReader<RotateEvent>,
    mut blocks: Query<(&mut Transform, &Block)>,
) {
    let blocks_affected: HashSet<Block> = ev_rotate
        .iter()
        .map(|rotate_event| rotate_event.0.get_adjacent())
        .flatten()
        .collect();
    for (mut transform, block) in &mut blocks {
        if blocks_affected.contains(block) {
            transform.rotate_local_z(constants::STRAIGHT_ANGLE * 2.);
        }
    }
}

#[derive(Component)]
pub struct Goal {}

pub struct RotateEvent(pub Block);
pub struct RedrawLaserEvent();

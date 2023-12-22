use crate::component::block::Block;
use crate::level::point::Point;
use bevy::prelude::{Commands, Resource};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants;

#[derive(Debug, Clone)]
pub enum BlockType {
    Mirror(MirrorState),
    Empty,
    Blocking,
}

#[derive(Debug, Clone)]
pub enum MirrorState {
    /// Forward slash `/` position
    Default,
    /// Backward slash `\` position
    Flipped,
}

impl BlockType {
    pub fn from_char(character: &char) -> BlockType {
        match character {
            'o' => Self::Empty,
            'x' => Self::Blocking,
            '/' => Self::Mirror(MirrorState::Default),
            '\\' => Self::Mirror(MirrorState::Flipped),
            x => panic!("Unrecognized Block Type character {x}"),
        }
    }

    pub fn spawn_block(&self, mut commands: &mut Commands, point: Point) {
        match self {
            BlockType::Mirror(state) => Self::spawn_mirror(commands, point),
            BlockType::Empty => {}
            BlockType::Blocking => Self::spawn_blocking(commands, point),
        }
    }

    fn spawn_blocking(commands: &mut Commands, point: Point) {
        let x = point.x;
        let y = point.y;
        commands.spawn((
            MaterialMesh2dBundle {
                // make this mesh a transparent cube
                mesh: constants::QUAD.get().unwrap().clone().into(),
                transform: Transform::default()
                    .with_scale(Vec3::new(60., 60., 1.))
                    .with_translation(Vec3 {
                        x: ((x - constants::HALF_WIDTH) * constants::SCALE) as f32,
                        y: ((y - constants::HALF_HEIGHT) * constants::SCALE) as f32,
                        z: 0.,
                    }),
                material: constants::DEFAULT_COLOR.get().unwrap().clone(),
                ..Default::default()
            },
            // add colision group and make it collide only with it
            Collider::cuboid(0.5, 0.5),
            Block::new(x, y),
            RaycastPickTarget::default(),
            Pickable,
            Interaction::default(),
        ));
    }

    fn spawn_mirror(commands: &mut Commands, point: Point) {
        let x = point.x;
        let y = point.y;
        commands.spawn((
            MaterialMesh2dBundle {
                // make this mesh a transparent cube
                mesh: constants::QUAD.get().unwrap().clone().into(),
                transform: Transform::default()
                    .with_scale(Vec3::new(60., 60., 1.))
                    .with_translation(Vec3 {
                        x: ((x - constants::HALF_WIDTH) * constants::SCALE) as f32,
                        y: ((y - constants::HALF_HEIGHT) * constants::SCALE) as f32,
                        z: 0.,
                    })
                    .with_rotation(Quat::from_rotation_z(constants::STRAIGHT_ANGLE)),
                material: constants::DEFAULT_COLOR.get().unwrap().clone(),
                ..Default::default()
            },
            // add colision group and make it collide only with it
            Collider::cuboid(0.5, 0.),
            Block::new(x, y),
            RaycastPickTarget::default(),
            Pickable,
            Interaction::default(),
        ));
    }
}

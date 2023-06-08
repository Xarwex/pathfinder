use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::HashSet};
use bevy_mod_picking::prelude::*;

const GRID_WIDTH: isize = 5;
const GRID_HEIGHT: isize = 3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_event::<RotateEvent>()
        .add_startup_system(setup)
        .add_system(click_on_block_system)
        .add_system(resolve_rotation_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), RaycastPickCamera::default()));

    let color = materials.add(ColorMaterial::from(Color::GREEN));
    let quad = meshes.add(Mesh::from(shape::Quad::default()));

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: quad.clone().into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(64.))
                        .with_translation(Vec3 {
                            x: (x * 65) as f32,
                            y: (y * 65) as f32,
                            z: 0.,
                        }),
                    material: color.clone(),
                    ..Default::default()
                },
                Block { x, y },
                RaycastPickTarget::default(),
                Pickable,
                Interaction::default(),
            ));
        }
    }
}

fn click_on_block_system(
    mut ev_rotate: EventWriter<RotateEvent>,
    mut clicked: Query<(&Interaction, &mut Transform, &mut Block), Changed<Interaction>>,
) {
    for (interaction, mut transform, block) in &mut clicked {
        if matches!(interaction, Interaction::Clicked) {
            ev_rotate.send(RotateEvent(block.clone()))
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
            transform.rotate_local_z(1.0)
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
struct Block {
    x: isize,
    y: isize,
}

impl Block {
    fn is_adjacent(self, block: &Block) -> bool {
        [(self.x, block.x), (self.y, block.y)]
            .iter()
            .fold(false, |acc, (a, b)| acc ^ ((a - b).abs() == 1))
    }

    fn get_adjacent(self) -> HashSet<Block> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(x, y)| Block {
                x: self.x + x,
                y: self.y + y,
            })
            .collect()
    }
}

struct RotateEvent(pub Block);

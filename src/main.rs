use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_system(click_on_block_system)
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
    mut clicked: Query<(&Interaction, &mut Transform, &mut Block), Changed<Interaction>>,
) {
    for (interaction, mut transform, block) in &mut clicked {
        if matches!(interaction, Interaction::Clicked) {
            // rotate the block and change the flow
            transform.rotate_local_z(1.0);
        }
    }
}

#[derive(Component)]
struct Block {
    x: usize,
    y: usize,
}

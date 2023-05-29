use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 1;

fn main() {
    App::new()
        .init_resource::<Game>()
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
    mut game: ResMut<Game>,
) {
    commands.spawn(Camera2dBundle::default());

    let color = materials.add(ColorMaterial::from(Color::GREEN));
    let quad = meshes.add(Mesh::from(shape::Quad::default()));

    // let mut grid = vec![];
    for height in 0..GRID_HEIGHT {
        // let mut line = vec![];
        for width in 0..GRID_WIDTH {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: quad.clone().into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(64.))
                        .with_translation(Vec3 {
                            x: (width * 65) as f32,
                            y: (height * 65) as f32,
                            z: 0.,
                        }),
                    material: color.clone(),
                    ..Default::default()
                },
                Block {},
                Interaction::default(),
            ));
            // line.push(Block {
            //     mesh:
            // })
        }
        // grid.push(line);
    }

    // game.grid = grid;
}

fn click_on_block_system(
    mut clicked: Query<(&Interaction, &mut Transform, &mut Block), Changed<Interaction>>,
) {
    for (interaction, mut transform, block) in &mut clicked {
        println!("{:?}", interaction);
    }
}

#[derive(Resource, Default)]
struct Game {
    grid: Vec<Vec<Block>>,
}

#[derive(Component)]
struct Block {}

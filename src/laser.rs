use crate::constants;
use crate::RedrawLaserEvent;
use bevy::{prelude::*, render::render_resource::PrimitiveTopology, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone)]
struct PolygonalChain {
    pub points: Vec<Vec2>,
}

impl PolygonalChain {
    fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}

impl From<PolygonalChain> for Mesh {
    fn from(value: PolygonalChain) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            value
                .points
                .into_iter()
                .map(|vec2| Vec3::from((vec2, 1.)))
                .collect::<Vec<_>>(),
        );
        mesh
    }
}

pub fn spawn_laser(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rapier_context: Res<RapierContext>,
    mut origin: Vect,
    mut direction: Vect,
) {
    let max_toi = 1000.;
    let filter = QueryFilter::new();
    let solid = true;
    let mut points = vec![origin];
    direction = direction.normalize();
    while let Some((entity, intersection)) =
        rapier_context.cast_ray_and_get_normal(origin, direction, max_toi, solid, filter)
    {
        let hit_point = intersection.point;
        let hit_normal = intersection.normal;
        let reflection = direction - 2. * hit_normal.dot(direction) * hit_normal;
        points.push(hit_point);
        direction = reflection.normalize();
        origin = hit_point + direction; // Offset fucked maths
    }
    points.push(origin + (direction * max_toi));
    let polygonal_chain = meshes.add(Mesh::from(PolygonalChain::new(points)));
    let laser_color = materials.add(ColorMaterial::from(Color::RED));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: polygonal_chain.clone().into(),
            material: laser_color,
            ..Default::default()
        },
        Laser,
    ));
}

pub fn laser_system(
    mut ev_redraw: EventReader<RedrawLaserEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut prev_laser: Query<(Entity, &Laser)>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, _) in &mut prev_laser {
        commands.entity(entity).despawn();
    }
    spawn_laser(
        commands,
        meshes,
        materials,
        rapier_context,
        Vec2::new(
            (-constants::SCALE * (constants::HALF_WIDTH + 1)) as f32,
            (-constants::SCALE * constants::HALF_HEIGHT) as f32,
        ),
        Vec2::new(1.0, 0.),
    );
}

#[derive(Component)]
pub struct Laser;

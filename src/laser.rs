use crate::RotateEvent;
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
    origin: Vect,
    direction: Vect,
) {
    let max_toi = 1000.;
    let filter = QueryFilter::default();
    let solid = false;
    let mut points = vec![Vec2::new(0., 0.), origin];
    if let Some((entity, intersection)) =
        rapier_context.cast_ray_and_get_normal(origin, direction, max_toi, solid, filter)
    {
        let hit_point = intersection.point;
        println!("{}", hit_point);
        let hit_normal = intersection.normal;
        points.push(hit_point);
    };
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
        Vec2::new(-60., -100.),
        Vec2::new(1.0, 1.0),
    );
}

#[derive(Component)]
pub struct Laser;

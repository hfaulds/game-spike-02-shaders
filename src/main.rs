pub use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2dPlugin;
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "game-spike-02-shaders".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        },
        ..Default::default()
    }));
    app.add_plugin(Material2dPlugin::<PulseMaterial>::default());

    app.add_startup_system(setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut matpulse: ResMut<Assets<PulseMaterial>>,
    mut matcolor: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        matpulse.add(PulseMaterial {}),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                .into(),
            material: matcolor.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_xyz(0.0, 0.0, 1.5),
            ..default()
        },
    ));
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "b3afc14c-f8b8-4e77-a9e2-2ddd13f92db6"]
pub struct PulseMaterial {}

impl Material2d for PulseMaterial {
    fn vertex_shader() -> ShaderRef {
        "pulse.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "pulse.wgsl".into()
    }
}

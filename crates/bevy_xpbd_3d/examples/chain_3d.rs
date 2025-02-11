#![allow(clippy::unnecessary_cast)]

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use examples_common_3d::XpbdExamplePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(XpbdExamplePlugin)
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(AmbientLight {
            brightness: 2.0,
            ..default()
        })
        .insert_resource(SubstepCount(80))
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 2.0))
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

#[derive(Component)]
struct Controllable;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let particle_count = 100;
    let particle_radius = 0.06;
    let particle_mesh = PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: particle_radius as f32,
                ..default()
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial::from(Color::rgb(0.2, 0.7, 0.9))),
        ..default()
    };

    // Spawn kinematic particle that can follow the mouse
    let mut previous_particle = commands
        .spawn((particle_mesh.clone(), RigidBody::Kinematic, Controllable))
        .id();

    // Spawn the rest of the particles, connecting each one to the previous one with joints
    for i in 1..particle_count {
        let current_particle = commands
            .spawn((
                particle_mesh.clone(),
                RigidBody::Dynamic,
                Position(i as Scalar * Vector::NEG_Y * (particle_radius * 2.2)),
                MassPropertiesBundle::new_computed(&Collider::ball(particle_radius), 1.0),
            ))
            .id();

        commands.spawn(
            SphericalJoint::new(previous_particle, current_particle)
                .with_local_anchor_1(Vector::NEG_Y * particle_radius * 1.1)
                .with_local_anchor_2(Vector::Y * particle_radius * 1.1)
                .with_compliance(0.00001),
        );

        previous_particle = current_particle;
    }

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 5.0, 20.0))
            .looking_at(Vec3::NEG_Y * 5.0, Vec3::Y),
        ..default()
    });
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut marbles: Query<&mut LinearVelocity, With<Controllable>>,
) {
    for mut linear_velocity in &mut marbles {
        if keyboard_input.pressed(KeyCode::Up) {
            linear_velocity.z -= 0.75;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            linear_velocity.z += 0.75;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            linear_velocity.x -= 0.75;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            linear_velocity.x += 0.75;
        }
        linear_velocity.0 *= 0.9;
    }
}

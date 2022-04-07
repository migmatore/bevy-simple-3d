use bevy::prelude::shape;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32
}

#[derive(Component)]
struct Jumper {
    jump_impulse: f32,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 20.0),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Jumper { jump_impulse: 5. })
        .insert(Player { speed: 3.5 });
}

fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Jumper, &mut RigidBodyVelocityComponent), With<Player>>,
) {
    for (jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            velocity.linvel = Vec3::new(0., jumper.jump_impulse, 0.).into();
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut RigidBodyVelocityComponent)>
) {
    for (player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec3::new(-player.speed, velocity.linvel.y, velocity.linvel.z).into();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec3::new(player.speed, velocity.linvel.y, velocity.linvel.z).into();
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.linvel = Vec3::new(velocity.linvel.x, velocity.linvel.y, -player.speed).into();
        }
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.linvel = Vec3::new(velocity.linvel.x, velocity.linvel.y, player.speed).into();
        }
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, -2., 0.).into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }
        .into(),
        body_type: RigidBodyType::Static.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(5. / 2., 0., 5. / 2.).into(),
        ..Default::default()
    };

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}

fn spawn_light(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        
        transform: Transform::from_xyz(5., 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Test".to_string(),
            width: 400.,
            height: 400.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup)
        .add_startup_stage("player_stage", SystemStage::single(spawn_player))
        .add_system(player_jumps.system())
        .add_system(player_movement.system())
        .add_startup_stage("floor_stage", SystemStage::single(spawn_floor))
        .add_startup_stage("light_stage", SystemStage::single(spawn_light))
        .run();
}

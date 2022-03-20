use ::bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};

const TIME_STEP: f32 = 1. / 60.;
const SPEED: f32 = 250.;

#[derive(Component)]
struct Robot;

#[derive(Component, Debug, PartialEq)]
enum JumpState {
    Jumping,
    NotJumping,
}

#[derive(Component, Default)]
struct JumpStartTime(f64);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
    // .insert(FlyCamera2d::default());
    // let texture_handle = asset_server.load("block.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(4., 4.), 1, 1);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // commands
    //     .spawn_bundle(SpriteSheetBundle {
    //         texture_atlas: texture_atlas_handle,
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..Default::default()
    //     });
    // let box_collider = ColliderBundle::default();
    // let box_collider_parent: ColliderParentComponent = ColliderParent {
    //     handle: entity_with_rigid_body.handle(),
    //     pos_wrt_parent: Vec2::ZERO.into(),
    // }
    // .into();

    for i in -10..10 {
        spawn_block(&mut commands, &asset_server, (i as f32) * 32.0, 0.0);
    }
    // commands
    //     .spawn()
    //     .insert_bundle(SpriteBundle {
    //         texture: asset_server.load("32block.png"),
    //         transform: Transform::from_xyz(0.0, 0.0, 1.0),
    //         ..Default::default()
    //     })
    //     .insert_bundle(ColliderBundle {
    //         shape: ColliderShape::cuboid(16.0, 16.0).into(),
    //         position: Vec2::new(0.0, 0.0).into(),
    //         ..Default::default()
    //     })
    // .insert_bundle(
    //     RigidBodyBundle {
    //         position: Vec2::new(0.0, 0.0).into(),
    //         ..Default::default()
    //     }
    // )
    // .insert(ColliderPositionSync::Discrete);
    // commands
    //     .spawn()
    //     .insert_bundle(SpriteBundle {
    //         texture: asset_server.load("32block.png"),
    //         transform: Transform::from_xyz(0.0, 50.0, 1.0),
    //         ..Default::default()
    //     })
    //     .insert_bundle(
    //         ColliderBundle {
    //             shape: ColliderShape::cuboid(16.0, 16.0).into(),
    //             position: Vec2::new(0.0, 50.0).into(),
    //             ..Default::default()
    //         }
    //     )
    //     .insert_bundle(
    //         RigidBodyBundle {
    //             position: Vec2::new(0.0, 50.0).into(),
    //             ..Default::default()
    //         }
    //     )
    //     .insert(ColliderPositionSync::Discrete);
    // let texture_handle = asset_server.load("robot.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(40.6, 39.66), 10, 38);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("robot.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(16.0, 16.0).into(),
            position: Vec2::new(0.0, 50.0).into(),
            ..Default::default()
        })
        // .insert_bundle(SpriteSheetBundle {
        //     texture_atlas: texture_atlas_handle,
        //     transform: Transform::from_xyz(0.0, 50.0, 1.0),
        //     ..Default::default()
        // })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, 50.0).into(),
            forces: RigidBodyForces {
                gravity_scale: 0.0,
                ..Default::default()
            }
            .into(),
            body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
            mass_properties: MassProperties::new(Vec2::new(1.0, 2.0).into(), 10.0, 0.5).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Robot)
        .insert(JumpState::NotJumping)
        .insert(JumpStartTime {
            ..Default::default()
        });
}

fn spawn_block(commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("32block.png"),
            transform: Transform::from_xyz(x, y, 1.0),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(16.0, 16.0).into(),
            position: Vec2::new(x, y).into(),
            material: ColliderMaterial { 
                friction: 0.0,
                friction_combine_rule: CoefficientCombineRule::Min,
                ..Default::default() 
            }.into(),
            ..Default::default()
        })
        // .insert_bundle(
        //     RigidBodyBundle {
        //         position: Vec2::new(0.0, 0.0).into(),
        //         ..Default::default()
        //     }
        // )
        .insert(ColliderPositionSync::Discrete);
}

fn frame_update(mut last_time: Local<f64>, time: Res<Time>) {
    info!("update: {}", time.seconds_since_startup() - *last_time);
    *last_time = time.seconds_since_startup();
}

fn robot_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // mut query: Query<&mut Transform, With<Robot>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Robot>)>,
    mut velocities: Query<&mut RigidBodyVelocityComponent, With<Robot>>,
) {
    // if let Ok(mut transform) = query.get_single_mut() {
    let dir = if keyboard_input.pressed(KeyCode::Left) {
        -1.
    } else if keyboard_input.pressed(KeyCode::Right) {
        1.
    } else {
        0.
    };
    for mut rb_vel in velocities.iter_mut() {
        rb_vel.linvel = Vec2::new(dir * SPEED, rb_vel.linvel.data.0[0][1]).into();
        rb_vel.angvel = 0.0;
    }
    // transform.translation.x += dir * SPEED * TIME_STEP;
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.translation.x += dir * SPEED * TIME_STEP;
    };
}
/* Set the velocities inside of a system. */
// fn modify_body_velocity(mut velocities: Query<&mut RigidBodyVelocityComponent, With<Robot>>) {
//     for mut rb_vel in velocities.iter_mut() {
//         rb_vel.linvel = Vec2::new(1.0, rb_vel.linvel.data.0[0][1]).into();
//         rb_vel.angvel = 0.0;
//         // println!("rb_vel.linvel: {}", rb_vel.linvel)
//     }
// }

/* Set the gravity scale inside of a system. */
// fn modify_body_gravity_scale(mut forces: Query<&mut RigidBodyForcesComponent, With<Robot>>) {
//     for mut rb_forces in forces.iter_mut() {
//         rb_forces.gravity_scale = 2.0;
//     }
// }

fn robot_gravity(
    mut velocities: Query<
        (
            &mut RigidBodyVelocityComponent,
            &mut JumpState,
            &JumpStartTime,
        ),
        With<Robot>,
    >,
    time: Res<Time>,
) {
    if let Ok((mut rb_vel, mut jump_state, jump_start_time)) = velocities.get_single_mut() {
        if (*jump_state == JumpState::Jumping)
            & (time.seconds_since_startup() - jump_start_time.0 > 1.0)
        {
            *jump_state = JumpState::NotJumping;
            rb_vel.linvel = Vec2::new(rb_vel.linvel.data.0[0][0], -SPEED / 2.0).into();
        } 
        else if *jump_state != JumpState::Jumping {
            rb_vel.linvel = Vec2::new(rb_vel.linvel.data.0[0][0], -SPEED / 2.0).into();
        }
    }
}

fn robot_jump(
    keyboard_input: Res<Input<KeyCode>>,
    // mut robot_forces: Query<(&mut RigidBodyForcesComponent, &mut RigidBodyVelocityComponent, &RigidBodyMassPropsComponent), With<Robot>>
    mut velocities: Query<
        (
            &mut RigidBodyVelocityComponent,
            &mut JumpState,
            &mut JumpStartTime,
        ),
        With<Robot>,
    >,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if let Ok((mut rb_vel, mut jump_state, mut jump_start_time)) = velocities.get_single_mut() {
            if *jump_state == JumpState::NotJumping {
                rb_vel.linvel = Vec2::new(rb_vel.linvel.data.0[0][0], SPEED / 2.0).into();
                *jump_state = JumpState::Jumping;
                jump_start_time.0 = time.seconds_since_startup()
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robot!".to_string(),
            width: 480.0,
            height: 320.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::WHITE))
        // .add_plugin(FlyCameraPlugin)
        .add_system(robot_movement)
        .add_system(robot_jump)
        // .add_system(modify_body_velocity)
        // .add_system(modify_body_gravity_scale)
        .add_system(robot_gravity)
        .run();
}

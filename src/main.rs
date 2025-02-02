use bevy::{prelude::*, window::{WindowMode, WindowResolution}};
use rand::Rng;

const EXTENTS: Vec3 = Vec3::new(1920., 1080., 0.);
const BALL_SIZE: f32 = 20.;
const BALL_SPEED: f32 = 250.;
const PLAYER_WIDTH: f32 = 20.;
const PLAYER_HEIGHT: f32 = 160.;
const PLAYER_DIST: f32 = 800.;
const PLAYER_SPEED: f32 = 275.;

#[derive(Component)]
struct Player{
    num: bool,
    score: i32,
    up: KeyCode,
    down: KeyCode 
}

#[derive(Component)]
struct Ball{
    velo: Vec3
}

#[derive(Event)]
struct Goal {
    player: bool
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(EXTENTS.x, EXTENTS.y).with_scale_factor_override(1.0),
                resizable: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default() }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, (game_setup, ball_setup).chain())
        .add_systems(FixedUpdate, (player_movement, ball_movement, collision_checker).chain())
        .add_observer(on_goal)
        .run();
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

    // Spawn Ball
    commands.spawn((
        Ball{velo: Vec3::from((0.0, 0.0, 0.0))},
        Mesh2d(meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0., 0., 1.)
    ));

    // Player 1
    commands.spawn((
        Player{num: false, score: 0, up: KeyCode::KeyW, down: KeyCode::KeyS},
        Mesh2d(meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(-PLAYER_DIST, 0., 0.)
    ));

    // Player 2
    commands.spawn((
        Player{num: true, score: 0, up: KeyCode::ArrowUp, down: KeyCode::ArrowDown},
        Mesh2d(meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(PLAYER_DIST, 0., 0.)
    ));
}

fn ball_setup(
    mut query: Single<&mut Ball>
) {
    let mut rng = rand::rng();
    query.velo.x = BALL_SPEED * if rng.random_bool(0.5) {1.0} else {-1.0};
    query.velo.y = BALL_SPEED * if rng.random_bool(0.5) {1.0} else {-1.0};
}

//fn window_setup(mut window: Single<&mut Window>) {
//    window.set_maximized(true);
//}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    mut app_exit: ResMut<Events<bevy::app::AppExit>>
) {
    for player_bundle in &mut query {
        let (player, mut transform) = player_bundle;

        let mut movement_factor = 0.0;

        // Upwards movement
        if keyboard_input.pressed(player.up) {
            movement_factor += 1.0;
        }

        // Downwards movement
        if keyboard_input.pressed(player.down) {
            movement_factor -= 1.0;
        }

        transform.translation += movement_factor * Vec3::Y * PLAYER_SPEED * time.delta_secs();

        let bounds = Vec3::from(EXTENTS * 0.5);

        transform.translation = transform.translation.min(bounds - 0.5*PLAYER_HEIGHT*Vec3::Y).max(-bounds + 0.5*PLAYER_HEIGHT*Vec3::Y);
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit.send(bevy::app::AppExit::Success);
    }
}

fn ball_movement(
    mut commands: Commands,
    time: Res<Time>,
    query: Single<(&mut Ball, &mut Transform)>
) {
    let (mut ball, mut transform) = query.into_inner();

    transform.translation += ball.velo * time.delta_secs();

    let bounds = Vec3::from(EXTENTS / 2.0);

    if transform.translation.abs().y > (bounds.y - (0.5*BALL_SIZE)) {
        ball.velo.y *= -1.;
    }
    
    if transform.translation.abs().x > (bounds.x - (0.5*BALL_SIZE)) {
        commands.trigger(Goal{ player: if transform.translation.x < 0.0 {true} else {false} } );
    }
}

fn collision_checker(
    ball: Single<(&mut Ball, &Transform)>,
    players: Query<&Transform, With<Player>>
) {
    let (mut ball_ent, ball_transform) = ball.into_inner();
    //let ball_abs = ball_transform.translation.abs();
    for player in players.iter() {
        let x_cond = ((player.translation.x - ball_transform.translation.x).abs() < 0.5*(BALL_SIZE+PLAYER_WIDTH));
        let y_cond = ((player.translation.y - ball_transform.translation.y).abs() < 0.5*(BALL_SIZE+PLAYER_HEIGHT));
        if x_cond && y_cond {
            ball_ent.velo.x *= -1.;
            ball_ent.velo.x += ball_ent.velo.x.signum() * 10.;
        }
    }
}

fn on_goal(
    trigger: Trigger<Goal>,
    mut players: Query<&mut Player>,
    ball: Single<(&mut Ball, &mut Transform)>
) {
    // Ball logic
    let (mut ball_ent, mut ball_transform) = ball.into_inner();
    ball_transform.translation = Vec3::ZERO;
    let mut rng = rand::rng();
    ball_ent.velo.x = BALL_SPEED * if rng.random_bool(0.5) {1.0} else {-1.0};
    ball_ent.velo.y = BALL_SPEED * if rng.random_bool(0.5) {1.0} else {-1.0};

    // Player logic
    for mut player in &mut players {
        if player.num == trigger.player {
            player.score += 1;
        }
        println!("Player {}: {}", player.num as i8, player.score);
    }
}
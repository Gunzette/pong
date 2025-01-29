use bevy::{prelude::*, window::WindowMode};
use rand::Rng;

const EXTENTS: Vec3 = Vec3::new(1920., 1080., 0.);

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
        Mesh2d(meshes.add(Rectangle::new(20., 20.))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0., 0., 1.)
    ));

    // Player 1
    commands.spawn((
        Player{num: false, score: 0, up: KeyCode::KeyW, down: KeyCode::KeyS},
        Mesh2d(meshes.add(Rectangle::new(20., 160.))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(-800., 0., 0.)
    ));

    // Player 2
    commands.spawn((
        Player{num: true, score: 0, up: KeyCode::ArrowUp, down: KeyCode::ArrowDown},
        Mesh2d(meshes.add(Rectangle::new(20., 160.))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(800., 0., 0.)
    ));
}

fn ball_setup(
    mut query: Single<&mut Ball>
) {
    let mut rng = rand::rng();
    query.velo.x = 150. * if rng.random_bool(0.5) {1.0} else {-1.0};
    query.velo.y = 150. * if rng.random_bool(0.5) {1.0} else {-1.0};
}

//fn window_setup(mut window: Single<&mut Window>) {
//    window.set_maximized(true);
//}

fn player_movement() {}

fn ball_movement(
    mut commands: Commands,
    time: Res<Time>,
    query: Single<(&mut Ball, &mut Transform)>
) {
    let (mut ball, mut transform) = query.into_inner();

    transform.translation += ball.velo * time.delta_secs();

    let bounds = Vec3::from(EXTENTS / 2.0);

    if transform.translation.abs().y > (bounds.y - 10.) {
        ball.velo.y *= -1.;
    }
    
    if transform.translation.abs().x > (bounds.x - 10.) {
        commands.trigger(Goal{ player: if transform.translation.x < 0.0 {true} else {false} } );
    }
}

// TODO
fn collision_checker(
    
)

fn on_goal(
    trigger: Trigger<Goal>,
    mut players: Query<&mut Player>,
    ball: Single<(&mut Ball, &mut Transform)>
) {
    // Ball logic
    let (mut ball_ent, mut ball_transform) = ball.into_inner();
    ball_transform.translation = Vec3::ZERO;
    let mut rng = rand::rng();
    ball_ent.velo.x = 150. * if rng.random_bool(0.5) {1.0} else {-1.0};
    ball_ent.velo.y = 150. * if rng.random_bool(0.5) {1.0} else {-1.0};

    // Player logic
    for mut player in &mut players {
        if player.num == trigger.player {
            player.score += 1;
        }
        println!("Player {}: {}", player.num as i8, player.score);
    }
}
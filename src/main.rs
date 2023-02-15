use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    time::FixedTimestep,
};

const WALL_COLOR: Color = Color::rgb(50., 0., 0.);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const TIME_STEP: f32 = 1.0 / 120.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;

const LEFT_BOUND: f32 = LEFT_WALL + WALL_THICKNESS / 2.0 + BALL_SIZE.x / 2.0;
const RIGHT_BOUND: f32 = RIGHT_WALL - WALL_THICKNESS / 2.0 - BALL_SIZE.x / 2.0;
const BOTTOM_BOUND: f32 = BOTTOM_WALL + WALL_THICKNESS / 2.0 + BALL_SIZE.y / 2.0;
const TOP_BOUND: f32 = TOP_WALL - WALL_THICKNESS / 2.0 - BALL_SIZE.y / 2.0;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_ball)
                .with_system(apply_velocity),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
    ));

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

fn move_ball(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Ball>>) {
    let mut ball_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction_x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction_y += 1.0;
    }

    let new_ball_position_x = ball_transform.translation.x + direction_x * BALL_SPEED * TIME_STEP;
    let new_ball_position_y = ball_transform.translation.y + direction_y * BALL_SPEED * TIME_STEP;

    ball_transform.translation.x = new_ball_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
    ball_transform.translation.y = new_ball_position_y.clamp(BOTTOM_BOUND, TOP_BOUND);
}


fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

use bevy::{prelude::*,window::PrimaryWindow};
use rand::{rngs::ThreadRng,thread_rng,Rng};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin{
                    primary_window: Some(Window{
                        title : String::from("Flappy Bird"),
                        position : WindowPosition::Centered(MonitorSelection::Primary),
                        resolution : Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup_level)
        .add_systems(Update, update_bird)
        .run();
}

// Bird
const PIXEL_RATIO : f32 = 4.0;
const FLAP_FORCE : f32 = 500.0;
const GRAVITY : f32 = 2000.0;
const VELOCITY_TO_ROTATION_RATIO : f32 = 7.5;

// Obstacle
const OBSTACLE_AMOUNT : i32 = 5;
const OBSTACLE_WIDTH : f32 = 32.0;
const OBSTACLE_HEIGHT : f32 = 144.0;
const OBSTACLE_VERTICAL_OFFSET : f32 = 30.0;
const OBSTACLE_GAP_SIZE : f32 = 15.0;
const OBSTACLE_SPACING : f32 = 60.0;
const OBSTACLE_SCROLL_SPEED : f32 = 150.0;

#[derive(Resource)]
pub struct GameManager {
    pub pipe_image : Handle<Image>,
    pub window_dimensions : Vec2,
}

#[derive(Component)]
struct Bird {
    pub velocity : f32,
}

#[derive(Component)]
pub struct Obstacle {
    pub pipe_direction : f32,
}

fn setup_level(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    window_query : Query<&Window,With<PrimaryWindow>>,
) {
    let pipe_image = asset_server.load("pipe.png");
    let window = window_query.single().unwrap();

    commands.insert_resource(GameManager{
        pipe_image : pipe_image.clone(),
        window_dimensions : Vec2::new(window.width(), window.height()),
    });

    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));

    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite{
            image : asset_server.load("bird.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird{ velocity: 0.0 },
    ));

    let mut rand = thread_rng();

    spawn_obstacles(&mut commands, &mut rand, window.width(), &pipe_image)
}

fn get_centered_pipe_position() -> f32 {
    return (OBSTACLE_HEIGHT / 2.0 + OBSTACLE_GAP_SIZE) * PIXEL_RATIO;
}

fn spawn_obstacles(
    mut commands : &mut Commands,
    mut rand : &mut ThreadRng,
    window_width : f32,
    pipe_image : &Handle<Image>,
) {
    for i in 0..OBSTACLE_AMOUNT {
        let y_offset = generate_offset(rand);
        let x_pos = window_width / 2.0 + (OBSTACLE_SPACING * PIXEL_RATIO * i as f32);
        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (get_centered_pipe_position() + y_offset),
            1.0,
            commands,
            pipe_image,
        );
        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (-get_centered_pipe_position() + y_offset),
            -1.0,
            commands,
            pipe_image,
        );
    }
}

fn spawn_obstacle(
    translation : Vec3,
    pipe_direction : f32, // Top or bottom of screen
    commands : &mut Commands,
    pipe_image : &Handle<Image>,
) {
    commands.spawn((
        Sprite {
            image : pipe_image.clone(),
            ..Default::default()
        },
        Transform::from_translation(translation).with_scale(Vec3::new(
            PIXEL_RATIO,
            PIXEL_RATIO * -pipe_direction,
            PIXEL_RATIO,
        )),
        Obstacle{pipe_direction},
    ));
}

fn generate_offset(rand : &mut ThreadRng) -> f32 {
    return rand.gen_range(-OBSTACLE_VERTICAL_OFFSET..OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO;
}

fn update_bird(
    mut bird_query : Query<(&mut Bird, &mut Transform)>,
    time: Res<Time>,
    keys : Res<ButtonInput<KeyCode>>
) {
    if let Ok((mut bird, mut transform)) = bird_query.single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = FLAP_FORCE;
        }

        bird.velocity -= time.delta_secs() * GRAVITY;
        transform.translation.y += bird.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90.0, 90.0).to_radians()
        );
    }
}

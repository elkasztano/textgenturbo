use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::RenderPlugin,
    time::TimeUpdateStrategy,
    winit::WinitPlugin,
};
use bevy_capture::{
    encoder::mp4_openh264::Mp4Openh264Encoder, CameraTargetHeadless, Capture, CaptureBundle,
};
use clap::Parser;
use colorgrad::Gradient;
use rand::prelude::*;
use std::{fs::File, time::Duration};
use xorwowgen::xorwow64::WrapB;

use textgenturbo::{
    animation::{move_camera, wave_waveable, CameraMovement, WaveAble},
    cli::Cli,
    text2mesh::generate_text,
    texturegen::gen_normal_basic_multi,
};

fn main() {
    let cli = Cli::parse();

    let my_plugins = (
        DefaultPlugins
            .build()
            .disable::<WinitPlugin>()
            .set(RenderPlugin {
                synchronous_pipeline_compilation: true,
                ..default()
            }),
        ScheduleRunnerPlugin {
            run_mode: RunMode::Loop { wait: None },
        },
        bevy_capture::CapturePlugin,
    );

    App::new()
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 50.0,
        )))
        .insert_resource(Msaa::Sample4)
        .insert_resource(cli)
        .add_plugins(my_plugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (wave_waveable, move_camera, update_capture))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    cli: Res<Cli>,
) {
    let font_data = include_bytes!("../assets/fonts/FiraCode-Medium.ttf");

    let (bevy_text_mesh, size_x) = generate_text(font_data, &cli.text);

    commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(bevy_text_mesh),
                material: materials.add(StandardMaterial {
                    base_color: Srgba::rgb(1.0, 1.0, 1.0).into(),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(size_x / -2f32, 0f32, 0f32)),
                ..default()
            });
        });

    let colorgrad = colorgrad::preset::turbo();
    let mut rng = WrapB::seed_from_u64(12345);

    // circular sphere array
    for i in 0..50 {
        for j in 0..50 {
            let f = 0.025;
            let spawn_x = i as f32 * 0.2 - 5.0 + (rng.gen::<f32>() - 0.5) * f;
            let spawn_y = j as f32 * 0.2 - 5.0 + (rng.gen::<f32>() - 0.5) * f;
            // add just a little bit of variation
            let height_bias = (rng.gen::<f32>() - 0.5) * f;
            let c = colorgrad.reflect_at(i as f32 * 0.02 + height_bias);
            let transl = Vec3::new(spawn_x, 0.5 + height_bias, spawn_y);
            // make object spawn dependent on distance
            // in order to get circular pattern
            let dist_bias = transl.distance(Vec3::ZERO);
            if dist_bias < 5.0 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.08).mesh().ico(2).unwrap()),
                        transform: Transform::from_translation(transl),
                        material: materials.add(StandardMaterial {
                            base_color: Srgba::rgb(c.r, c.g, c.b).into(),
                            ..default()
                        }),
                        ..default()
                    },
                    WaveAble {
                        ampli: 0.5,
                        // bias is dependent on distance to center
                        // to get a concentric pattern
                        bias: dist_bias * 1.5 + height_bias * 2.5,
                        base_height: 0.5,
                        speed: 0.25,
                    },
                ));
            }
        }
    }

    // procedurally generate texture using the 'noise' crate
    let ground_texture_handle = images.add(gen_normal_basic_multi(0.005, 123456, (2048, 2048)));

    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Circle::new(500.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                base_color_texture: Some(ground_texture_handle.clone()),
                unlit: true,
                cull_mode: None,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -25.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 3.5, 7.5),
        ..default()
    });

    // camera
    commands.spawn((
        (Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.0)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        })
        .target_headless(1920, 1080, &mut images),
        CaptureBundle::default(),
        CameraMovement,
    ));
}

fn update_capture(
    mut app_exit: EventWriter<AppExit>,
    mut capture: Query<&mut Capture>,
    mut frame: Local<u32>,
    cli: Res<Cli>,
) {
    let mut capture = capture.single_mut();
    if !capture.is_capturing() {
        capture.start(
            Mp4Openh264Encoder::new(
                File::create(&cli.output).expect("could not create file"),
                1920,
                1080,
            )
            .unwrap(),
        );
    }
    *frame += 1;
    // show progress
    eprint!("\rFRAME: {:03}/{:03}", *frame, cli.frames);
    if *frame >= cli.frames {
        eprintln!(" - FINISH");
        app_exit.send(AppExit::Success);
    }
}

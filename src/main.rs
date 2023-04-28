use std::{path::PathBuf, f32::consts::PI};
use bevy::{prelude::*};
use components::{charge::*,point::*};
use bevy_inspector_egui::prelude::*;
use bevy_flycam::prelude::*;

mod components;
mod util;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const DELTA_TIME: f64 = 0.001;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct StateVariables {
    initialized: bool,
    running: bool,
    path: PathBuf,
    speed: f32,
    elapsed: f64,
}

#[derive(Resource, Default)]
struct FieldInfo {
    efield: Vec<f64>,
    bfield: Vec<f64>,
}

#[derive(Component)]
struct Shape;

#[derive(Component)]
struct FieldText;

#[derive(Component)]
struct ChargeText;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Dist(f64);

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Factor(f64);

fn ui_setup(
    mut commands: Commands, asset_server: Res<AssetServer>, 
    mut field: ResMut<FieldInfo>, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    field.efield.extend(vec![0.0,0.0,0.0,0.0,0.0,0.0]);
    field.bfield.extend(vec![0.0,0.0,0.0,0.0,0.0,0.0]);

    // x-axis
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cylinder::default().into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.5, 0.0, 0.0)
            .with_scale(Vec3 { x: 0.1, y: 1., z: 0.1 })
            .with_rotation(Quat::from_rotation_z(PI/2.)),
            ..default()
        },
        Shape,
    ));

    // y-axis
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cylinder::default().into()),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0)
            .with_scale(Vec3 { x: 0.1, y: 1., z: 0.1 })
            .with_rotation(Quat::from_rotation_y(PI/2.)),
            ..default()
        },
        Shape,
    ));


    // z-axis
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cylinder::default().into()),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.5)
            .with_scale(Vec3 { x: 0.1, y: 1., z: 0.1 })
            .with_rotation(Quat::from_rotation_x(PI/2.)),
            ..default()
        },
        Shape,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Unpause",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 26.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        
                    ));
                });
        });

    commands.spawn(PointLightBundle { transform: Transform::from_translation(Vec3::ONE * 3.0),..default()});

    commands.spawn((
        TextBundle::from_section("Charges",
            TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 10.0, color: Color::WHITE,},
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect { bottom: Val::Px(5.0), right: Val::Px(15.0),..default()
            },..default()
        }),
        FieldText,
    ));

    info!("Move camera around by using WASD for lateral movement");
    info!("Use Left Shift and Spacebar for vertical movement");
    info!("Use the mouse to look around");
    info!("Press Esc to hide or show the mouse cursor");
        
}


fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text, Without<FieldText>>,
    mut button: ResMut<StateVariables>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                button.running = !button.running;
                if button.running {
                    text.sections[0].value = "Pause".to_string();
                } else {
                    text.sections[0].value = "Unpause".to_string();
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(StateVariables {running: false, initialized: true, path: "".into(), speed: 0.0, elapsed: 0.0})
        .insert_resource(Dist {0: 0.35})
        .init_resource::<FieldInfo>()
        .insert_resource(FixedTime::new_from_secs(DELTA_TIME as f32))
        .add_startup_system(ui_setup)
        .add_systems((file_drag_and_drop_system,button_system))
        .add_system(setup.run_if(|i: Res<StateVariables>| !i.initialized))
        .add_system(simulate.run_if(|x: Res<StateVariables>| x.running).in_schedule(CoreSchedule::FixedUpdate))
        .run();
}

// Startup system to setup the scene and spawn all relevant entities.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<StateVariables>,
    mut query1: Query<&mut Text, With<FieldText>>, 
    query2: Query<Entity, With<Charge>>,
    query3: Query<Entity, With<ChargeText>>, 
    mut field_info: ResMut<FieldInfo>,
) {
    query2.for_each(|entity| {
        println!("{:?}", entity);
        commands.entity(entity).despawn();
    });
    query3.for_each(|entity| {
        println!("{:?}", entity);
        commands.entity(entity).despawn();
    });

    let true_path = state.path.clone().into_os_string().into_string().unwrap();
    let (particles, e, b) = util::setup(true_path);
    field_info.efield = e;
    field_info.bfield = b;

    for mut entity in &mut query1 {
        entity.sections[0].value = format!("E-Field:({},{},{}),({},{},{})\nB-Field:({},{},{}),({},{},{})",
        field_info.efield[0],field_info.efield[1],field_info.efield[2],
        field_info.efield[3],field_info.efield[4],field_info.efield[4],
        field_info.bfield[0],field_info.bfield[1],field_info.bfield[2],
        field_info.bfield[3],field_info.bfield[4],field_info.bfield[4]);
    }

    let mut s = shape::Icosphere::default();
    s.radius = 0.25;
    for i in particles.clone() {
        let mut c = Color::BLUE;
        if i.get_magnitude() > 0.0 {
            c = Color::RED;
        } else if i.get_magnitude() == 0.0 {
            c = Color::GRAY;
        }

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(s.try_into().unwrap()),
                material: materials.add(c.into()),
                transform: Transform::from_translation(i.to_vec()),
                ..default()
            },
            i,
        ));
    }

    let mut texts: Vec<TextSection> = vec![];
    texts.push(
        TextSection::new(
            format!("Time: \n"),
            TextStyle { 
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 10.0, 
                color: Color::WHITE, 
            },
        ),
    );
    for i in particles {
        texts.push(
            TextSection::new(
                format!("{}\n", i.display_pos()),
                TextStyle { 
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 10.0, 
                    color: Color::WHITE, 
                },
            ),
        );
    }

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections(texts),
        ChargeText,
    ));
    state.initialized = true;
    state.elapsed = 0.0;
}

// This system will move all Movable entities with a Transform
fn simulate(
    mut query: Query<(&mut Transform, &mut Charge)>, 
    //e: Res<EField>, b: Res<BField>, 
    field: Res<FieldInfo>,
    mut query1: Query<&mut Text, With<ChargeText>>, 
    mut state: ResMut<StateVariables>, dist: Res<Dist>,
) {
    let mut iter: bevy::ecs::query::QueryCombinationIter<(&mut Transform, &mut Charge), (), 2> = query.iter_combinations_mut();

    while let Some([(_, mut c1), (_, mut c2)]) = iter.fetch_next() {
        let mut coulomb = Charge::coulomb(*c1, *c2);
        
        if Point::dist(c1.get_pos(),c2.get_pos()) <= dist.0 {
            coulomb = Point::new();
        }

        c1.add_force(coulomb);
        c2.add_force(coulomb.neg());
    }
    
    let mut iter = query1.iter_mut().nth(0).unwrap();
    
    iter.sections[0].value = format!("Time: {:.2}\n", state.elapsed);
    let mut n = 1;
    for (mut transform, mut particle) in &mut query {
        let lor = Charge::lorentz(*particle, &field.efield, &field.bfield);
        let _ab_lor = Charge::abraham_lorentz(*particle, DELTA_TIME);
        particle.add_force(lor);
        
        if !particle.is_fixed() {
            particle.update(DELTA_TIME);
            transform.translation = particle.to_vec();
        }

        iter.sections[n].value = format!("{}\n", particle.display_pos());
        n += 1;
        particle.reset();
    }
    state.elapsed += DELTA_TIME;
}


fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>,mut state: ResMut<StateVariables>) {
    for event in events.iter() {
        info!("{:?}", event);
        if let FileDragAndDrop::DroppedFile { path_buf, window } = event {
            println!("Dropped file with path: {:?}, in window id: {:?}", path_buf, window);
            state.path = path_buf.clone();
            state.initialized = false;
        }
    }
}
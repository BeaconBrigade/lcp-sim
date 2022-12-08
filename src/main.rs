#![cfg_attr(not(debug_assertions), windows_subsytem = "windows")]

mod components;
mod concentration;
mod error;
mod ui;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::EguiPlugin;
use chem_eq::{Direction, Equation};
use rand::{seq::SliceRandom, Rng};

use crate::{
    components::{Particle, ParticleBundle},
    error::Error,
    ui::{PauseSimulation, StartSimulation, UiState},
};

fn main() {
    App::new()
        .add_event::<StartSimulation>()
        .add_event::<PauseSimulation>()
        .init_resource::<UiState>()
        .init_resource::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Le Chatelier's Principle".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(update_equation)
        .add_system(start)
        .add_system(pause)
        .add_system_set(
            SystemSet::new()
                .label("ui")
                .with_system(ui::menu.before("graph"))
                .with_system(ui::help)
                .with_system(ui::eq_edit)
                .with_system(ui::graph.label("graph")),
        )
        .run();
}

#[derive(Debug, Default, Resource)]
pub struct AppState {
    pub is_running: bool,
}

/// try to balance the equation and update the output
fn update_equation(mut ui_state: ResMut<UiState>) {
    // don't rerun if input hasn't changed
    if ui_state.input == ui_state.last_input {
        return;
    }
    ui_state.last_input = ui_state.input.clone();
    if ui_state.input.is_empty() {
        ui_state.eq_res = Err(Error::WaitingForEquation);
        return;
    }

    let res = Equation::new(ui_state.input.as_str());
    let Ok(eq) = res else {
        ui_state.eq_res = res.map_err(Into::into);
        return;
    };
    let res = eq.to_balancer().balance();
    let Ok(eq) = res else {
        ui_state.eq_res = res.map_err(Into::into);
        return;
    };

    // must be reversible to be an equilibrium
    ui_state.eq_res = if *eq.direction() == Direction::Reversible {
        Ok(eq)
    } else {
        Err(Error::NotEquilibrium)
    }
}

fn setup(mut commands: Commands) {
    info!("setting up");
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("ben-griffiths-background.jpg"),
    //     ..Default::default()
    // });
}

fn start(
    start_sim: EventReader<StartSimulation>,
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ui_state: Res<UiState>,
) {
    if start_sim.is_empty() {
        return;
    }
    let Ok(eq) = ui_state.eq_res.as_ref() else {
        return;
    };
    info!("starting simulation");
    start_sim.clear();
    app_state.is_running = true;
    const COLOURS: [Color; 11] = [
        Color::BLUE,
        Color::CRIMSON,
        Color::CYAN,
        Color::LIME_GREEN,
        Color::ORANGE,
        Color::PINK,
        Color::PURPLE,
        Color::RED,
        Color::VIOLET,
        Color::WHITE,
        Color::YELLOW,
    ];
    let mut rng = &mut rand::thread_rng();
    let clrs = COLOURS.choose_multiple(
        &mut rng,
        ui_state
            .eq_res
            .as_ref()
            .expect("equation is valid")
            .num_compounds(),
    );

    for ((i, cmp), clr) in eq.iter_compounds().enumerate().zip(clrs) {
        // scaled units because... we don't have "2415919104"+ bytes of memory...
        let units = cmp.get_units(eq.volume().unwrap()) * 1.0e-19;
        info!("units = {}", units);
        for _ in 0..units as u64 {
            let translation = Vec3::new(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
            );
            commands.spawn(ParticleBundle {
                particle: Particle {
                    compound_index: i,
                    colour: *clr,
                    ..Default::default()
                },
                material: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(1.).into()).into(),
                    material: materials.add(ColorMaterial::from(*clr)),
                    transform: Transform::from_translation(translation),
                    ..Default::default()
                },
            });
        }
    }
}

fn pause(
    pause_sim: EventReader<PauseSimulation>,
    // mut commands: Commands,
    mut app_state: ResMut<AppState>,
) {
    if pause_sim.is_empty() {
        return;
    }
    info!("pausing simulation");
    pause_sim.clear();
    app_state.is_running = false;
}

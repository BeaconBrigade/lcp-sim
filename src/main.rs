#![cfg_attr(not(debug_assertions), windows_subsytem = "windows")]

mod components;
mod error;
mod reaction;
mod ui;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};
use chem_eq::{Direction, Equation};
use rand::{seq::SliceRandom, Rng};

use crate::{
    components::{Particle, ParticleBundle},
    error::Error,
    ui::{PauseSimulation, StartSimulation, StopSimulation, UiState},
};

fn main() {
    App::new()
        .add_event::<StartSimulation>()
        .add_event::<PauseSimulation>()
        .add_event::<StopSimulation>()
        .init_resource::<UiState>()
        .init_resource::<AppState>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Le Chatelier's Principle".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(InspectorPlugin::<UiState>::new())
        .add_plugin(InspectorPlugin::<AppState>::new())
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Particle>()
        .add_startup_system(setup)
        .add_system(update_equation)
        .add_system(start)
        .add_system(pause)
        .add_system(stop)
        .add_system(reaction::reaction_continuation.with_run_criteria(reaction::system_is_running))
        .add_system(reaction::start_reaction.with_run_criteria(reaction::system_is_setting_up))
        .add_system_set(
            SystemSet::new()
                .label("ui")
                .with_system(ui::menu.before("graph"))
                .with_system(ui::help)
                .with_system(ui::eq_edit)
                .with_system(ui::graph.label("graph")),
        )
        .add_system_set(
            SystemSet::new()
                .label("reactions")
                .with_run_criteria(reaction::system_is_running)
                .with_system(reaction::adjust_equilibrium)
                .with_system(reaction::reaction_continuation),
        )
        .run();
}

#[derive(Debug, Default, Resource, Inspectable)]
pub struct AppState {
    pub running: SimulationState,
}

#[derive(Debug, Default, Clone, Copy, Inspectable)]
pub enum SimulationState {
    /// The equilibrium has reached equilibrium and is mostly idle
    Running,
    /// The simulation has started but hasn't reached equilibrium
    Starting,
    /// A change to the system has occured, the system is adjusting
    Adjusting,
    /// The simulation is paused, but running
    Paused,
    /// The simulation is stopped, nothing has been started
    #[default]
    Stopped,
}

impl SimulationState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running | Self::Starting | Self::Adjusting)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped)
    }

    pub fn is_setting_up(&self) -> bool {
        matches!(self, Self::Starting)
    }

    pub fn pause(&mut self) {
        *self = Self::Paused
    }

    pub fn start(&mut self) {
        *self = Self::Running
    }

    pub fn stop(&mut self) {
        *self = Self::Stopped
    }
}

/// try to balance the equation and update the output
fn update_equation(mut ui_state: ResMut<UiState>) {
    // don't rerun if input hasn't changed
    if ui_state.input == ui_state.last_input {
        return;
    }
    ui_state.last_input = ui_state.input.clone();
    if ui_state.input.is_empty() {
        ui_state.eq_res = Err(Error::WaitingForEquation).into();
        return;
    }

    let res = Equation::new(ui_state.input.as_str());
    let Ok(eq) = res else {
        ui_state.eq_res = res.map_err(Into::into).into();
        return;
    };
    let res = eq.to_balancer().balance();
    let Ok(mut eq) = res else {
        ui_state.eq_res = res.map_err(Into::into).into();
        return;
    };

    // only give the left side of the equation initial concentrations
    let range = 0..eq.left().len();
    for (cmp, _) in eq.iter_compounds_mut().zip(range).fuse() {
        cmp.concentration = 0.001;
    }

    // must be reversible to be an equilibrium
    ui_state.eq_res = if *eq.direction() == Direction::Reversible {
        Ok(eq).into()
    } else {
        Err(Error::NotEquilibrium).into()
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

    // if the game has already started (it was paused, not stopped), we don't have to spawn more
    // entities
    if app_state.running.is_paused() {
        app_state.running.start();
        return;
    }
    app_state.running.start();

    // spawn initial particles

    // the colours to randomly pick from
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
                rng.gen_range(-500.0..500.0),
                rng.gen_range(-500.0..500.0),
                0.0,
            );
            commands.spawn(ParticleBundle {
                particle: Particle { compound_index: i },
                material: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.).into()).into(),
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
    app_state.running.pause();
}

fn stop(
    stop_sim: EventReader<StopSimulation>,
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    query: Query<(Entity, With<Particle>)>,
) {
    if stop_sim.is_empty() {
        return;
    }
    info!("stopping simulation");
    stop_sim.clear();
    app_state.running.stop();

    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}

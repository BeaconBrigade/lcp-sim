use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

/// A particle to be displayed
#[derive(Default, Bundle)]
pub struct ParticleBundle {
    #[bundle]
    pub particle: Particle,
    #[bundle]
    pub material: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct Particle {
    /// index into UiState::eq_res.unwrap().iter_compounds().collect::<Vec<_>>()
    pub compound_index: usize,
}

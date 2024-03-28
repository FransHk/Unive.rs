/// Configuration that contains all planet
/// bounds that it must adhere to when randomly
/// generated
#[derive(Debug, Clone)]
pub struct PlanetConfig {
    pub lower_pos_bound: f32,
    pub upper_pos_bound: f32,
    pub velocity_bound: f32,
    pub mass_mean: f32,
    pub mass_std: f32,
    pub mass_to_size: f32,
}

impl PlanetConfig {
    pub fn new(
        lower_pos_bound: f32,
        upper_pos_bound: f32,
        velocity_bound: f32,
        mass_mean: f32,
        mass_std: f32,
        mass_to_size: f32,
    ) -> PlanetConfig {
        PlanetConfig {
            lower_pos_bound,
            upper_pos_bound,
            velocity_bound,
            mass_mean,
            mass_std,
            mass_to_size,
        }
    }
}

// /// Trait for all bodies that have mass and a position
// pub trait CelestialBody {
//     fn mass(&self) -> f32;
//     fn pos(&self) -> [f32; 2];
// }

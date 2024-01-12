/// Configuration that contains all planet
/// bounds that it must adhere to when randomly
/// generated
#[derive(Debug, Clone)]
pub struct PlanetConfig {
    pub lower_pos_bound: f64,
    pub upper_pos_bound: f64,
    pub velocity_bound: f64,
    pub mass_mean: f64,
    pub mass_std: f64,
    pub mass_to_size: f64,
}

impl PlanetConfig {
    pub fn new(
        lower_pos_bound: f64,
        upper_pos_bound: f64,
        velocity_bound: f64,
        mass_mean: f64,
        mass_std: f64,
        mass_to_size: f64,
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

/// Trait for all bodies that have mass and a position
pub trait CelestialBody {
    fn mass(&self) -> f64;
    fn pos(&self) -> [f64; 2];
}

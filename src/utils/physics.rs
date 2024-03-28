use crate::celestial_bodies::planet::Planet;

use super::array_logic::{dot_product, scalar_mult, subtract_arrays, Normalise};
// use crate::celestial_bodies::body_config::CelestialBody;

/// Calculate the gravitational force between two bodies,
/// takes two structs that implement the CelestialBody trait
/// this means that the body:
/// - Has mass
/// - Has pos
pub fn grav_force(planet_1: &Planet, planet_2: &Planet, g: f32) -> ([f32; 2], [f32; 2]) {
    let dist = subtract_arrays(planet_1.position, planet_2.position);
    let sqr_dist = dot_product(dist, dist); // dist.x^2 + dist.y^2
    let force_dir = dist.normalise();
    let force = scalar_mult(force_dir, g * planet_2.mass * planet_1.mass);
    let force = scalar_mult(force, 1.0 / sqr_dist); // pull on m1 by m2
    let force_inv = scalar_mult(force, -1.0); // equally, pull on m2 by m1
    (force, force_inv)
}

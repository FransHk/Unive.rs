use super::array_logic::{dot_product, scalar_mult, subtract_arrays, Normalise};
use crate::celestial_bodies::body_config::CelestialBody;

/// Calculate the gravitational force between two bodies,
/// takes two structs that implement the CelestialBody trait
/// this means that the body:
/// - Has mass
/// - Has pos
pub fn grav_force<C: CelestialBody>(mass1: &C, mass2: &C, g: f64) -> ([f64; 2], [f64; 2]) {
    let dist = subtract_arrays(mass1.pos(), mass2.pos());
    let sqr_dist = dot_product(dist, dist); // dist.x^2 + dist.y^2
    let force_dir = dist.normalise();
    let force = scalar_mult(force_dir, g * mass2.mass() * mass1.mass());
    let force = scalar_mult(force, 1.0 / sqr_dist); // pull on m1 by m2
    let force_inv = scalar_mult(force, -1.0); // equally, pull on m2 by m1
    (force, force_inv)
}

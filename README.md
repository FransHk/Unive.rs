# Unive.rs 🚀

## Description
"Unive.rs" is a minimalistic, Newtonian simulation of celestial bodies in space for which I am developing a set of minimalist 2D physics utilities. All utilities are intentionally simplistic, pure Rust. 

 
## General components
 <a href="src/celestial_bodies/planet.rs"> Planet logic </a>

## Computational components
 <a href="src/utils/array_logic.rs"> Linear algebra utilities </a>
 <a href="src/utils/physics.rs"> Newtonian gravity </a>

We calculate the gravitational force between two celestial bodies in the following method. As per Newton's second law, gravitational
force is opposite and equal between two bodies. 
```rust
pub fn grav_force<C: CelestialBody>(mass1: &C, mass2: &C, g: f64) -> ([f64; 2], [f64; 2]) {
    let dist = subtract_arrays(mass1.pos(), mass2.pos());
    let sqr_dist = dot_product(dist, dist); 
    let force_dir = normalise_vector(dist);
    let force = scalar_mult(force_dir, g * mass2.mass() * mass1.mass());
    let force = scalar_mult(force, 1.0 / sqr_dist); 
    let force_inv = scalar_mult(force, -1.0); 
    (force, force_inv)
}
```
However, two bodies are not always equally affected. The force exerted between the earth and moon is equal and opposite, yet the moon is accelerated more than the earth. This reflected 
in how we handle adding forces to celestial bodies. More specifically, applied force is inversely proportional to the body's mass. 
```rust
    /// Adds a 2-dimensional force to the body,
    /// it is scaled by the body's mass before being
    /// applied
    pub fn add_force(&mut self, force: [f64; 2]) {
        let scaled_force = al::scalar_mult(force, 1.0 / self.mass); // i.e. force/self.mass
        self.velocity = al::add_arrays(self.velocity, scaled_force);
    }
```



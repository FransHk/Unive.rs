# Unive.rs 🚀

## Description
"Unive.rs" is a minimalistic Newtonian simulation of celestial bodies in space for which I am developing a set of minimalist 2D physics utilities. All utilities are intentionally simplistic, pure Rust. 

## Dependencies
External crates intentionally are intentionally kept to a minimum. The Piston engine and OpenGL are used only to draw the 2D graphics to screen. 
 
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
    let force_dir = dist.normalise();
    let force = scalar_mult(force_dir, g * mass2.mass() * mass1.mass());
    let force = scalar_mult(force, 1.0 / sqr_dist); 
    let force_inv = scalar_mult(force, -1.0); 
    (force, force_inv)
}
```
As an optimisation, we leverage the concept that Newtonian gravitational forces between two bodies are exactly equal and opposite to each other,
Thus, instead of calculating all possible celestial body pairs, we only look at unique pairs. For example, with planets [1,2,3], we get 3 pairs: (1,2), (1,3) and (2,3) instead of 3x2=6 non-unique pairs. 
For each unique pair, we apply the force and opposing force, respectively.

```rust
            let mut bottom: usize = 1;
            for i in 0..planets.len() {
                for j in bottom..planets.len() {
                    if i != j {
                        let (force, force_inv) = grav_force(&planets[i], &planets[j], GRAV_CONST);
                        planets[i].add_force(force_inv);
                        planets[j].add_force(force);
                    }
                }
                bottom += 1;
            }
```
However, two bodies are not always equally affected. For example, the force exerted between the earth and moon is equal and opposite, yet the moon is accelerated more than the earth. This reflected 
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


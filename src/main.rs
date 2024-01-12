mod celestial_bodies;
mod utils;

use celestial_bodies::body_config::*;
use celestial_bodies::planet::Planet;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::WindowSettings;
use utils::colour::Colour;
use utils::physics::grav_force;

const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const GRAV_CONST: f64 = 2.0;

// Some constants used throughout the code
// Create a set of planets according to
// a normal distribution
fn create_planets(amt_planet: u32, bounds: f64) -> Vec<Planet> {
    //(Planet, Vec<Planet>) {
    let mut planets = Vec::<Planet>::new();
    let planet_const = PlanetConfig::new(0.0, bounds, 1.3, 5.0, 10.0, 0.2);
    for i in 0..amt_planet {
        planets.push(Planet::new(&planet_const, i, WHITE));
    }

    planets
}

fn main() {
    // SET UP THE MAIN CONFIG DATA
    let bounds: f64 = 1028.0; // window size
    let centre: [f64; 2] = [bounds * 0.5, bounds * 0.5];
    let mut planets = create_planets(100, bounds);

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Window", [bounds; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    // Game loop. First, render every object (planet),
    // then, update each planet's position and check
    // for collisions.
    while let Some(e) = events.next(&mut window) {
        // Render step, all planetary bodies
        //
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c: graphics::Context, g: &mut GlGraphics| {
                graphics::clear(BLACK, g);
                for planet in planets.iter() {
                    planet.draw(c, g);
                }
            });
        }

        // Update step, each planet moves by its vel
        // scaled by dt for a frame-independent movement
        if let Some(args) = e.update_args() {
            // Handle step-wise update of each planet
            for planet in planets.iter_mut() {
                planet.update(&args); // pass update args for 'dt' value to scale movement
                planet.check_dist_from_centre(centre);
            }

            let mut bottom: usize = 1;
            // Handle gravitational force for unique planet pair
            // e.g. for 5 planets we have 5+4+3+2+1=15 force calcs
            for i in 0..planets.len() {
                // e.g. i = 1
                for j in bottom..planets.len() {
                    if i != j {
                        // Obtain force, it is always equal and opposite,
                        // the .add_force method scales the force by the mass
                        // of the body
                        let (force, force_inv) = grav_force(&planets[i], &planets[j], GRAV_CONST);
                        planets[i].add_force(force_inv);
                        planets[j].add_force(force);
                    }
                }

                bottom += 1;
            }
        }
    }
}

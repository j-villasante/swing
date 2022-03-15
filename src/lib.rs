use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
pub struct Simulation {
    rod_length: f64,
    i_velocity: f64,
    i_angle: f64,
    omega: f64,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(
        gravity: f64,
        rod_length: f64,
        i_velocity: f64,
        i_angle: f64,
    ) -> Simulation {
        Simulation {
            rod_length,
            i_velocity,
            i_angle,
            omega: f64::sqrt(gravity / rod_length),
        }
    }

    pub fn theta(&self, t: f64) -> Position {
        let theta = self.i_angle * f64::cos(self.omega * t)
            + self.i_velocity * f64::sin(self.omega * t) / self.omega;

        Position {
            x: f64::sin(theta) * self.rod_length,
            y: f64::cos(theta) * self.rod_length,
        }
    }
}

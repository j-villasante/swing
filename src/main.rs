use std;

pub struct Parameters {
    pub gravity: f64,
    pub rod_length: f64,
    pub i_velocity: f64,
    pub i_position: f64,
}

pub struct Variables {
    pub omega: f64,
    pub phi: f64,
    pub a: f64,
}

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub fn get_variables(p: Parameters) -> Variables {
    let omega = f64::sqrt(p.gravity / p.rod_length);
    let a = f64::sqrt(
        p.i_position.powf(2.0) + p.i_velocity.powf(2.0) / omega.powf(2.0),
    );
    let phi = f64::acos(p.i_position / a);
    Variables { omega, phi, a }
}

pub fn theta(v: &Variables, t: f64) -> Position {
    let theta = v.a * f64::cos(v.omega * t + v.phi);

    Position {
        x: f64::cos(theta),
        y: f64::sin(theta),
    }
}

fn main() {
    let parameters = Parameters {
        gravity: -9.8,
        rod_length: 1.0,
        i_velocity: 0.0,
        i_position: std::f64::consts::PI / 2.0,
    };
    let variables = get_variables(parameters);
    for t in 0..10 {
        let position = theta(&variables, t as f64);
        println!("{},{},{}", t as f64, position.x, position.y);
    }
}

#![allow(dead_code)]
use std::f64;
struct Clock {
    time: f64,
    dt: f64,
}
pub struct Corde {
    clock: Clock,
    mu: f64,
    l: f64,
    pub n: f64,
    dx: f64,
    beta: f64,
    tension: f64,
    f: f64,
    pub y: Vec<f64>,
    y_p: Vec<f64>,
    y_pp: Vec<f64>,
    force: Vec<f64>,
}
// trait Action {
//     fn next(&mut self);
//     fn init(&mut self);
// }
impl Clock {
    pub fn next(&mut self) {
        self.time += self.dt;
    }
}
impl Corde {
    pub fn compute_single(&mut self) {
        for i in 1..(self.n - 2.0) as usize {
            let pos = self.y_p[i + 1] - 2 as f64 * self.y_p[i] + self.y_p[i - 1];
            let first = self.clock.dt.powf(2.0) / (self.mu + self.beta * self.clock.dt);
            self.y[i] = first
                * (self.tension * pos / self.dx.powi(2)
                    + self.mu * (2.0 * self.y_p[i] - self.y_pp[i]) / self.clock.dt.powi(2)
                    + self.beta * self.y_p[i] / self.clock.dt
                    + self.force[i]);
        }
        self.replace();
        self.clock.next();
    }
    pub fn pluck(&mut self, position: usize, this_force: f64) {
        self.force[position] = this_force;
        self.compute_single();
        self.force[position] = 0.0;
        self.replace();
        self.clock.next();
    }
    fn show(&mut self) {
        for i in self.y.clone().into_iter() {
            print!("{i} ");
        }
        println!("");
    }
    pub fn replace(&mut self) {
        self.y_pp = self.y_p.clone();
        self.y_p = self.y.clone();
    }
}

fn build_clock() -> Clock {
    Clock {
        time: 0.0,
        dt: 0.000125,
    }
}
pub fn build_corde() -> Corde {
    let l: f64 = 0.44;
    let n: usize = 21;
    let mu: f64 = 0.0077;
    let f: f64 = 100.0;
    Corde {
        clock: build_clock(),
        n: n as f64,
        l,
        dx: l / (n as f64 - 1.0),
        beta: 0.325,
        mu,
        f, // en Hz
        tension: mu * f.powi(2) * 4.0 * l.powi(2),
        y: vec![0.0; n],
        y_p: vec![0.0; n],
        y_pp: vec![0.0; n],
        force: vec![0.0; n],
    }
}

fn main() {
    let mut c = build_corde();
    c.pluck(5, 0.01);
    c.show();
    while c.clock.time < 1.0 {
        c.compute_single();
        c.show();
    }
}

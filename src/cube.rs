use crate::{GROUND_Y, WALL_X};
use raylib::prelude::*;

pub const MAX_CUBE_SIZE: f64 = 200.;
pub const MIN_CUBE_SIZE: f64 = 50.;

pub struct Cube {
    pub x: f64,
    mass: f64,
    velocity: f64,
    pub size: f64,
}

impl Cube {
    pub fn new(x: f64, mass: f64, velocity: f64) -> Cube {
        Self {
            x,
            mass,
            velocity,
            size: mass.max(MIN_CUBE_SIZE).min(MAX_CUBE_SIZE),
        }
    }

    pub fn next_x(&self, delta: f64) -> f64 {
        return self.x + delta * self.velocity;
    }

    pub fn update(&mut self, delta: f64, minimum_x: f64) {
        self.x = {
            let this = self.next_x(delta);
            if this > minimum_x {
                this
            } else if minimum_x > this {
                minimum_x
            } else if this == minimum_x {
                if this.is_sign_positive() && minimum_x.is_sign_negative() {
                    this
                } else {
                    minimum_x
                }
            } else {
                this + minimum_x
            }
        };
    }

    pub fn collide_wall(&mut self, delta: f64) -> bool {
        if self.next_x(delta) <= WALL_X && self.velocity < 0. {
            self.velocity *= -1.;
            true
        } else {
            false
        }

        // if self.next_x(delta) + self.size >= WIDTH && self.velocity > 0. {
        //     self.velocity *= -1.;
        // }
    }

    pub fn collide_cube_on_right(&mut self, other: &mut Cube, delta: f64) -> bool {
        if self.next_x(delta) + self.size >= other.next_x(delta) {
            let m1 = self.mass;
            let m2 = other.mass;
            let v1 = self.velocity;
            let v2 = other.velocity;

            self.velocity = ((m1 - m2) * v1 + 2. * m2 * v2) / (m1 + m2);
            other.velocity = ((m2 - m1) * v2 + 2. * m1 * v1) / (m2 + m1);

            true
        } else {
            false
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, scale_x: f64) {
        d.draw_rectangle(
            (self.x / scale_x) as i32,
            (GROUND_Y - (self.size / scale_x)) as i32,
            (self.size / scale_x) as i32,
            (self.size / scale_x) as i32,
            rcolor(self.mass as _, (self.mass / 2.) as _, 100, 255),
        );

        let label: &str = &format!("{} kg", self.mass);
        let label_size = (label.chars().count() * 11) as f64;
        d.draw_text(
            &label,
            ((self.x + self.size / 2. - label_size / 2.
                + if self.size < label_size {
                    label_size / 6.
                } else {
                    0.
                })
                / scale_x) as i32,
            (GROUND_Y - self.size / scale_x / 2. - 10.
                + if self.size < label_size { 2. } else { 0. }) as i32,
            if self.size < label_size { 15 } else { 20 },
            Color::WHITE,
        );
    }
}

mod cube;

use std::{env, ffi::CString, os::unix::prelude::OsStrExt, path::Path};

// use std::time;
use cube::Cube;
use raylib::{
    ffi::{CloseAudioDevice, InitAudioDevice, LoadSound, PlaySound, UnloadSound},
    prelude::*,
};

pub const HEIGHT: f64 = 480.;
pub const WIDTH: f64 = 640.;
pub const GROUND_Y: f64 = HEIGHT * 0.75;
pub const WALL_X: f64 = WIDTH / 6.;
pub const DIGITS: u32 = 2;
pub const TIME_STEPS: usize = 2usize.pow(DIGITS * 4);

fn main() {
    unsafe {
        let (mut rl, thread) = raylib::init()
            .size(WIDTH as i32, HEIGHT as i32)
            .title("Bouncing PI")
            .build();

        InitAudioDevice();

        let path = Path::new(&env::current_dir().unwrap()).join("resources/clack.wav");
        let path_str_c = CString::new(path.as_os_str().as_bytes()).unwrap();
        let clack_sound = LoadSound(path_str_c.as_ptr());

        let mut cube_a = Cube::new(WIDTH * 0.25, 1., 0.);
        let mut cube_b = Cube::new(WIDTH * 0.50, 100_i64.pow(DIGITS) as f64, -100.);
        // let mut cube_c = Cube::new(WIDTH * 0.75, 200., 100.);

        let mut bounces = 0;

        // let mut last_update = time::Instant::now();
        while !rl.window_should_close() {
            // let delta = (time::Instant::now() - last_update).as_secs_f32();
            let delta = 0.001;
            // last_update = time::Instant::now();

            let time_steps = if DIGITS > 2 { TIME_STEPS } else { 1 };
            let mut bounced = false;

            for _ in 0..time_steps {
                if cube_a.collide_cube_on_right(&mut cube_b, delta) {
                    bounces += 1;
                    bounced = true;
                }
                // cube_b.collide_cube_on_right(&mut cube_c, delta);

                if cube_a.collide_wall(delta) {
                    bounces += 1;
                    bounced = true;
                }
                // cube_b.collide_wall(delta);
                // cube_c.collide_wall(delta);

                cube_a.update(delta, WALL_X);
                cube_b.update(delta, WALL_X + cube_a.size);
                // cube_c.update(delta);
            }

            let scale_x = {
                let other = cube_b.x + cube_b.size + 20.;
                if WIDTH > other {
                    WIDTH
                } else if other > WIDTH {
                    other
                } else if WIDTH == other {
                    if WIDTH.is_sign_positive() && other.is_sign_negative() {
                        WIDTH
                    } else {
                        other
                    }
                } else {
                    WIDTH + other
                }
            } / WIDTH;

            if bounced {
                PlaySound(clack_sound);
            }

            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::BLACK);

            cube_a.draw(&mut d, scale_x);
            cube_b.draw(&mut d, scale_x);
            // cube_c.draw(&mut d);

            d.draw_line(
                (WALL_X / scale_x) as i32,
                0,
                (WALL_X / scale_x) as i32,
                GROUND_Y as i32,
                Color::WHITE,
            );
            d.draw_line(
                0,
                GROUND_Y as i32,
                WIDTH as i32,
                GROUND_Y as i32,
                Color::WHITE,
            );

            d.draw_text(
                &format!(
                    "{} rebonds\nPI = {}",
                    bounces,
                    (bounces as f64 / 10_i32.pow(DIGITS) as f64)
                ),
                10,
                20,
                20,
                Color::WHITE,
            );
        }

        UnloadSound(clack_sound);
        CloseAudioDevice();
    }
}

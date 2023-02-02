extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use smumerix_core::edg::{self, get_moved_particles, Particle};

pub struct App {
    gl: GlGraphics,                // OpenGL drawing backend.
    prev_particles: Vec<Particle>, // prev state of the gas
    cur_edg: edg::EventDrivenGas,  // State of the gas
    anim_time: f64,
    timestep_time: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_GRAY: [f32; 4] = [0.21, 0.21, 0.21, 0.0];
        const BALL_GREEN: [f32; 4] = [0.20, 0.46, 0.42, 1.0];
        const VECTOR_PINK: [f32; 4] = [0.74, 0.53, 0.55, 1.0];
        let ctx = self.gl.draw_begin(args.viewport());
        clear(BG_GRAY, &mut self.gl);
        let particles = if self.prev_particles.len() > 40 {
            self.prev_particles.clone()
        } else {
            get_moved_particles(&self.prev_particles, self.timestep_time)
        };
        for (_idx, particle) in particles.iter().enumerate() {
            let (x, y) = (particle.x.x, particle.x.y);
            let (x, y) = (args.window_size[0] * x, args.window_size[1] * y);
            let square = rectangle::square(0.0, 0.0, 2.0 * particle.r * args.window_size[0]);

            let transform = ctx.transform.trans(x, y).trans(
                -particle.r * args.window_size[0],
                -particle.r * args.window_size[1],
            );
            // Draw a box rotating around the middle of the screen.
            ellipse(BALL_GREEN, square, transform, &mut self.gl);
            let (line_to_x, line_to_y) = (
                particle.v.x * args.window_size[0],
                particle.v.y * args.window_size[1],
            );
            line(
                VECTOR_PINK,
                1.0,
                [0.0, 0.0, line_to_x, line_to_y],
                transform.trans(
                    particle.r * args.window_size[0],
                    particle.r * args.window_size[1],
                ),
                &mut self.gl,
            );
        }
        self.gl.draw_end();
    }

    fn update(&mut self, args: &UpdateArgs) {
        let ts = args.dt * 2.0;
        self.timestep_time += ts;
        self.anim_time += ts;
        let collision_time = self.cur_edg.cur_time;
        if self.anim_time >= collision_time {
            self.timestep_time = self.anim_time - collision_time;
            self.prev_particles = self.cur_edg.particles.clone();
            self.cur_edg.step();
        }
    }
}

fn main() {
    // let mut edg = edg::EventDrivenGas::new_uniform_v(100, 0.04, 0.03).unwrap();
    let mut edg = edg::EventDrivenGas::new_uniform_v(15, 0.04, 0.05).unwrap();
    // let mut edg = edg::EventDrivenGas::new_for_test_4(-0.1);
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("event_driven_gas", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let prev_particles = edg.particles.clone();
    edg.step();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        prev_particles,
        cur_edg: edg,
        anim_time: 0.0,
        timestep_time: 0.0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

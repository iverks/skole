extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use smumerix_core::edg;

pub struct App {
    gl: GlGraphics,           // OpenGL drawing backend.
    edg: edg::EventDrivenGas, // State of the gas
    anim_time: f64,
    timestep_time: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        let ctx = self.gl.draw_begin(args.viewport());
        clear(GREEN, &mut self.gl);
        for (_idx, particle) in self.edg.particles.iter().enumerate() {
            let particle = particle.borrow();
            let (x, y) = (
                particle.x.x, // + particle.v.x * self.timestep_time,
                particle.x.y, // + particle.v.x * self.timestep_time,
            );
            let (x, y) = (args.window_size[0] * x, args.window_size[1] * y);
            let square = rectangle::square(0.0, 0.0, 2.0 * particle.r * args.window_size[0]);

            let transform = ctx.transform.trans(x, y).trans(
                -particle.r * args.window_size[0],
                -particle.r * args.window_size[1],
            );
            // Draw a box rotating around the middle of the screen.
            ellipse(RED, square, transform, &mut self.gl);
            let (line_to_x, line_to_y) = (
                particle.v.x * args.window_size[0],
                particle.v.y * args.window_size[1],
            );
            line(
                BLUE,
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
        if self.anim_time >= self.edg.pq.peek().unwrap().time {
            self.timestep_time = 0.0;
            self.edg.step();
        }
    }
}

fn main() {
    let edg = edg::EventDrivenGas::new_uniform_v(5, 0.04, 0.13).unwrap();
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("event_driven_gas", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        edg,
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

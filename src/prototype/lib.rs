extern crate piston;
extern crate graphics;
extern crate image;
extern crate glutin_window;
extern crate opengl_graphics;

mod simulation;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture, TextureSettings };

use simulation::Simulation;

pub fn run() {
	// Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "grid_city",
            [640, 480]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let img = Texture::from_path("../assets/landscapeTiles_sheet.png", &TextureSettings::new()).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        sim: Simulation::new(),
        img: img
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    sim: Simulation,
    img: Texture
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.2, 0.5, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        let img = &self.img;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            let transform = c.transform.trans(x, y);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

            image(img, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}

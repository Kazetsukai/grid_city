extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod simulation;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture, TextureSettings };

use simulation::{ Simulation, Building, grid_coords };

pub fn run() {
	// Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "grid_city",
            [1024, 768]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let imgs = [
        Texture::from_path("../assets/landscapeTiles_sheet.png", &TextureSettings::new()).unwrap(),
        Texture::from_path("../assets/buildingTiles_sheet.png", &TextureSettings::new()).unwrap(),
    ];

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        sim: Simulation::new(),
        spritesheets: imgs
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.button_args() {
            app.button(&b);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    sim: Simulation,
    spritesheets: [Texture; 2]
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.2, 0.5, 1.0];
        const GRASS_TILE: [f64; 4] = [398.0, 265.0, 132.0, 99.0];
        const ROAD_TILE:  [f64; 4] = [266.0, 723.0, 132.0, 99.0];
        const HOUSE_TILE: [f64; 4] = [265.0, 254.0, 132.0, 127.0];
        const HOUSE_ROOF: [f64; 4] = [397.0, 1149.0, 99.0, 60.0];

        let img = Image::new();
        let (gl, sim, spritesheets) = (&mut self.gl, &self.sim, &self.spritesheets);

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            let grass_tile = img.src_rect(GRASS_TILE);
            let road_tile = img.src_rect(ROAD_TILE);
            let house_tile = img.src_rect(HOUSE_TILE);
            let house_roof = img.src_rect(HOUSE_ROOF);

            let mut elem = 0;

            for (ref cell, coord) in sim.grid.iter().zip(grid_coords()) {
                let (xa, ya) = coord;
                let (x, y) = (xa as f64, ya as f64);

                let transform = c.transform.trans(
                    5.0 * GRASS_TILE[2] / 2.0,
                    0.0
                ).trans(
                    x * GRASS_TILE[2] / 2.0 - y * GRASS_TILE[2] / 2.0, 
                    y * GRASS_TILE[3] / 3.0 + x * GRASS_TILE[3] / 3.0
                );

                    
                elem += 1;
                if elem > 500000 { break; }

                match &cell.building {
                    &Some(Building::Residential) => {
                        house_tile.draw(
                            &spritesheets[1],
                            &DrawState::default(),
                            transform.trans(0.0, -28.0),
                            gl,
                        );
                        house_roof.draw(
                            &spritesheets[1],
                            &DrawState::default(),
                            transform.trans(17.0, -36.0),
                            gl,
                        );
                    },
                    &Some(Building::Road) => {
                        road_tile.draw(
                            &spritesheets[0],
                            &DrawState::default(),
                            transform,
                            gl,
                        );  
                    }
                    &None => {
                        grass_tile.draw(
                            &spritesheets[0],
                            &DrawState::default(),
                            transform,
                            gl,
                        );  
                    }
                    _ => ()
                };
            }
        });
    }

    fn textRender(&mut self) {
        let mut row = 0;
        let mut elem = 0;
        let sim = &self.sim;

        for (ref cell, coord) in sim.grid.iter().zip(grid_coords()) {
            let (x, y) = coord;

            if y > row {
                println!();
                row = y;
            }
            elem += 1;
            if elem > 10000 { break; }

            /*match &cell.building {
                &Some(Building::Residential) => print!("H"),
                &Some(Building::Road) => print!("#"),
                &Some(Building::Office) => print!("O"),
                &None => print!("."),
                _ => print!("?")
            };*/

            if cell.nav.distWork > 9 {
               print!(">");
            }
            else {
               print!("{}", cell.nav.distWork);
            }
        }

        println!();
    }

    fn update(&mut self, args: &UpdateArgs) {
    }

    fn button(&mut self, args: &ButtonArgs) {
        if args.state == ButtonState::Release {
            match args.button {
                Button::Keyboard(Key::Space) => {
                    self.sim.tick();
                },
                _ => ()
            }
        }
    }
}

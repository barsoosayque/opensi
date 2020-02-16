extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use opensi::Package;
use opensi::state::main_menu::MainMenu;
use opensi::state::StateManager;

struct App {
    gl: GlGraphics,
    window: GlutinWindow,

    event_settings: EventSettings,
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let window_settings = WindowSettings::new("opensi", [640, 480])
            .graphics_api(opengl)
            .exit_on_esc(true);
        let window: GlutinWindow = window_settings.build().unwrap();
        let event_settings = EventSettings::new();
        let gl = GlGraphics::new(opengl);

        App { gl, window, event_settings }
    }

    fn run(&mut self) {
        let mut events = Events::new(self.event_settings);
        let mut state_manager = StateManager::new();
        state_manager.forward::<MainMenu>();

        while let Some(event) = events.next(&mut self.window) {
            if let Some(args) = event.update_args() {
                state_manager.update(args.dt as f32);
            }

            if let Some(_) = event.render_args() {
                state_manager.render();
            }
        }
    }
}


fn main() {
    // let mut app = App::new(OpenGL::V2_1);
    // app.run();
    
    let package = Package::open("tests/data/slamjam2.siq").expect("can't open file");
    println!("{}", package.name.unwrap());
}
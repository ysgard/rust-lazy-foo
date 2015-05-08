#![feature(step_by)]
extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

const WIDTH:  i32 = 640;
const HEIGHT: i32 = 480;

    
/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window)  {
    let sdl = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    let win = match Window::new(&sdl, "SDL Tutorial",
                      WindowPos::PosCentered,
                      WindowPos::PosCentered,
                      WIDTH, HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("Failed to create Window!: {}", err)
    };

    (sdl, win)
}


fn main() {

    // Initialize SDL2
    let (sdl_context, window) = init();

    // Set texture filtering to linear

    
    let mut renderer = match Renderer::from_window(window, RenderDriverIndex::Auto,
                                                   ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };

            
    let mut context = renderer.drawer();

    // running is 'mut' because we will want to 'flip' it to false when we're ready
    // to exit the game loop.
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump();
    
    // game loop
    while running {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit {..} => {
                    running = false
                },
                _ => {}
            }
        }
        
        // Set renderer color using the context
        context.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        // Clear the screen
        context.clear();

        // Render red filled quad
        let fill_rect = Rect{ x: WIDTH / 4,
                             y: HEIGHT / 4,
                             w: WIDTH / 2,
                             h: HEIGHT / 2 };
        context.set_draw_color(Color::RGB(0xff, 0, 0));
        context.fill_rect(fill_rect);

        // Render green outlined quad
        let outline_rect = Rect{ x: WIDTH / 6,
                                y: HEIGHT / 6,
                                w: (WIDTH * 2) / 3,
                                h: (HEIGHT * 2) / 3 };
        context.set_draw_color(Color::RGB(0, 0xff, 0));
        context.draw_rect(outline_rect);

        // Draw Blue horizontal line
        context.set_draw_color(Color::RGB(0, 0, 0xff));
        context.draw_line(Point{ x: 0, y: HEIGHT / 2 },
                          Point{ x: WIDTH, y: HEIGHT / 2 });

        // Draw vertical line of yellow dots
        context.set_draw_color(Color::RGB(0xff, 0xff, 0));
        for i in (0..HEIGHT).step_by(4) {
            context.draw_point(Point{ x: WIDTH / 2, y: i });
        }

        // Update the screen
        context.present();
    }
}

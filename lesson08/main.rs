#![feature(iterator_step_by)]
#![feature(step_by)]
extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

    
/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window)  {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let win = match video.window("SDL Tutorial 08", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build() {
        Ok(window) => window,
        Err(err)   => panic!("Failed to create Window!: {}", err)
    };

    (sdl, win)
}


fn main() {

    // Initialize SDL2
    let (sdl_context, window) = init();

    // Set texture filtering to linear
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };

    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump().unwrap();
    
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
        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        // Clear the screen
        renderer.clear();

        // Render red filled quad
        // Here's a bit of a dumb thing - Rect::new expects its
        // coords to be i32 - because you can have negative
        // positions - but its width and height to be u32, because
        // negative size isn't logical (though it can be useful).
        // Because HEIGHT and WIDTH are u32, we have to cast them
        // as i32 for the positions.  Note that casting is
        // unsafe if your HEIGHT/WIDTH fall in the value range
        // between i32 and u32 (greater than 2,147,483,647)
        // but to date nobody's made a screen with a resolution
        // that large.
        let fill_rect = Rect::new(WIDTH as i32 / 4,
                                  HEIGHT as i32 / 4,
                                  WIDTH / 2,
                                  HEIGHT / 2);
        renderer.set_draw_color(Color::RGB(0xff, 0, 0));
        renderer.fill_rect(fill_rect).unwrap();

        // Render green outlined quad
        let outline_rect = Rect::new(WIDTH as i32 / 6,
                                     HEIGHT as i32 / 6,
                                     (WIDTH * 2) / 3,
                                     (HEIGHT * 2) / 3);
        renderer.set_draw_color(Color::RGB(0, 0xff, 0));
        renderer.draw_rect(outline_rect).unwrap();

        // Draw Blue horizontal line
        renderer.set_draw_color(Color::RGB(0, 0, 0xff));
        renderer.draw_line(Point::new(0, HEIGHT as i32 / 2),
                           Point::new(WIDTH as i32, HEIGHT as i32 / 2)).unwrap();

        // Draw vertical line of yellow dots
        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0));
        for i in (0..HEIGHT as i32).step_by(4) {
            renderer.draw_point(Point::new(WIDTH as i32 / 2, i as i32)).unwrap();
        }

        // Update the screen
        renderer.present();
    }
}

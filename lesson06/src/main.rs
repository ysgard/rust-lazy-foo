extern crate sdl2;
extern crate sdl2_image;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::event::Event;

use sdl2_image::{INIT_PNG, INIT_JPG, LoadTexture};

use std::path::Path;

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

    // Initialize SDL2_Image.  It doesn't return anything.
    // We're not using a jpg, but init it anyway to show flag joining.
    sdl2_image::init(INIT_PNG | INIT_JPG);
    
    (sdl, win)
}




fn main() {

    // Initialize SDL2
    let (sdl_context, window) = init();
    
    let mut renderer = match Renderer::from_window(window,
                                                   RenderDriverIndex::Auto,
                                                   ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };
    
    // Load the image
    let image_texture = match renderer.load_texture(Path::new("loaded.png")) {
        Ok(texture) => texture,
        Err(err)    => panic!("Could not load texture: {}", err)
    };

    // Blit the image to the window.  Note that in this example this happens outside
    // the game loop, in a real game this would happen inside.
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
        // Clear and render the texture each pass through the loop
        context.clear();
        context.copy(&image_texture, None, None);
        context.present();
    }
}

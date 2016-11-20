extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::{Renderer, Texture};
use sdl2::event::Event;
use sdl2::surface::{Surface};

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

const X_IMAGE: &'static str = "resources/x.bmp";

/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the video context) 
fn init() -> (Sdl, Window)  {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    // Create the window
    let win = match video.window("SDL Tutorial 03", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("Failed to create Window!: {}", err)
        };
    (sdl, win)
}

/// Take a string describing a path and use it to load
/// an image, and return its surface.
fn load_image(path: &'static str) -> Surface {
    use std::path::Path;
    match Surface::load_bmp(&Path::new(path)) {
        Ok(surface) => surface,
        Err(err)    => panic!("Could not load image: {}", err)
    }
}

/// Take a string describing a path and use it to
/// load an image, and return its texture
fn load_texture(path: &'static str, renderer: &Renderer) -> Texture {
    let image = load_image(path);
    match renderer.create_texture_from_surface(&image) {
        Ok(tex)    => tex,
        Err(err)   => panic!("Could not load texture: {}", err)
    }
}


fn main() {

    // Initialize SDL2
    let (sdl_context, window) = init();
    
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };
    
    // Load the image
    let image_texture = load_texture(X_IMAGE, &renderer);

    // running is 'mut' because we will want to 'flip' it to false when we're ready
    // to exit the game loop.
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err)      => panic!("Could not obtain event pump: {}", err)
    };
    
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
        renderer.clear();
        renderer.copy(&image_texture, None, None).unwrap();
        renderer.present();
    }
}

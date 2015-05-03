extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, Texture};
use sdl2::event::Event;
use sdl2::surface::{Surface};

const WIDTH:  i32 = 640;
const HEIGHT: i32 = 480;

const X_IMAGE: &'static str = "x.bmp";

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

/// Take a string describing a path and use it to load
/// an image, and return its surface.
fn load_image(path: &'static str) -> Surface {
    use std::path::Path;
    match Surface::from_bmp(&Path::new(path)) {
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
    
    let mut renderer = match Renderer::from_window(window, RenderDriverIndex::Auto,
                                                   ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };
    
    // Load the image
    let image_texture = load_texture(X_IMAGE, &renderer);


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

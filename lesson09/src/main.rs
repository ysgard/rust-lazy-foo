extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2_image::{LoadTexture, INIT_PNG};

const WIDTH:  i32 = 640;
const HEIGHT: i32 = 480;

const IMG_NAME: &'static str = "viewport.png";

// Note that 'renderer.load_texture' makes this example trivial.  See lesson03
// to show how we can manually load a surface and convert it to a texture.
    
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

    sdl2_image::init(INIT_PNG);
    
    (sdl, win)
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
    let image_texture = match renderer.load_texture(Path::new(IMG_NAME)) {
        Ok(texture) => texture,
        Err(err)    => panic!("Could not load texture: {}", err)
    };
            
    let mut context = renderer.drawer();

    // Set renderer color using the context
    context.set_draw_color(Color::RGB(0, 0, 0));
    
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

        // Create the top left viewport
        let top_left_viewport = Rect { x: 0,
                                       y: 0,
                                       w: WIDTH / 2,
                                       h: HEIGHT / 2
        };
        // Set the viewport.  Note that set_viewport takes an Option<Rect>,
        // so we have to pass either Some(rect) or None (whole window)
        context.set_viewport(Some(top_left_viewport));
        // Now render a texture to that viewport
        context.copy(&image_texture, None, None);

        // Top right viewport.  Same process
        let top_right_viewport = Rect { x: WIDTH / 2,
                                        y: 0,
                                        w: WIDTH / 2,
                                        h: HEIGHT / 2
        };
        context.set_viewport(Some(top_right_viewport));
        context.copy(&image_texture, None, None);

        // Bottom viewport
        let bottom_viewport = Rect { x: 0,
                                     y: HEIGHT / 2,
                                     w: WIDTH,
                                     h: HEIGHT / 2
        };
        context.set_viewport(Some(bottom_viewport));
        context.copy(&image_texture, None, None);
                                      
        // Update the screen
        context.present();
    }
}

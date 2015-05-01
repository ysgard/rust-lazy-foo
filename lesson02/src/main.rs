extern crate sdl2;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::timer;
use sdl2::surface::{Surface};

use std::path::Path;


// Screen dimensions
const WIDTH:  i32 = 640;
const HEIGHT: i32 = 480;

fn main() {

    // Initialize SDL
    // We'll just unwrap this
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    // Create the window
    let window = match Window::new(&sdl_context, "SDL Tutorial",
                                   WindowPos::PosCentered, WindowPos::PosCentered,
                                   WIDTH, HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("Failed to create window: {}", err)
    };

    // Obtain a renderer and context
    let mut renderer = match Renderer::from_window(window, RenderDriverIndex::Auto,
                                                   ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Failed to create renderer: {}", err)
    };

    // Load the image as a surface
    let image_surface = match Surface::from_bmp(&Path::new("hello_world.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("Could not load image: {}", err)
    };

    // At this point, we could do any number of transformations on the surface, and
    // then when we're ready, we convert it to a texture for quick blitting
    let image_texture = match renderer.create_texture_from_surface(&image_surface) {
        Ok(tex)  => tex,
        Err(err) => panic!("Could not convert image to texture: {}", err)
    };

    // Note that we couldn't grab the context before here, because Rust prevents us from
    // borrowing a mutable value while it's still in use.
    let mut ctx = renderer.drawer();
    // Clear the current window
    ctx.clear();
    // Blit the texture to the window.  We specify None, None because we're blitting
    // the entire image to the origin of the window.  If we just wanted to blit a subset
    // of the image, or to a particular section of the window, we would specify Some(rect),
    // where rect is a Rect representing the area to blit from/to.
    ctx.copy(&image_texture, None, None);
    // Flip the screen buffer.
    ctx.present();

    // Pause for two seconds
    timer::delay(2000);
}

    

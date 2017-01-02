extern crate sdl2;

use sdl2::surface::Surface;

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;


// Screen dimensions
const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

fn main() {

    // Initialize SDL
    // We'll just unwrap these - See lesson01 for an example of how to properly handle SDL errors
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("SDL Tutorial 02", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Obtain a renderer and context
    let mut renderer = window.renderer().build().unwrap();

    // Load the image as a surface - if we can't load the image, we want to know why
    let image_surface = match Surface::load_bmp(&Path::new("resources/hello_world.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("Could not load image: {}", err)
    };

    // At this point, we could do any number of transformations on the surface, and
    // then when we're ready, we convert it to a texture for quick blitting
    let image_texture = match renderer.create_texture_from_surface(&image_surface) {
        Ok(tex)  => tex,
        Err(err) => panic!("Could not convert image to texture: {}", err)
    };

    // Clear the current window
    renderer.clear();
    // Blit the texture to the window.  We specify None, None because we're blitting
    // the entire image to the origin of the window.  If we just wanted to blit a subset
    // of the image, or to a particular section of the window, we would specify Some(rect),
    // where rect is a Rect representing the area to blit from/to.
    // We match on the result because it could return an error.  Note that we return (), which
    // is Rust's 'nothing' type.
    match renderer.copy(&image_texture, None, None) {
        Ok(()) => (),
        Err(err) => panic!("Could not render texture: {}", err),
    };
    // Flip the screen buffer.
    renderer.present();

    // Pause for two seconds
    sleep(Duration::new(2, 0));
}

    

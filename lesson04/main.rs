extern crate sdl2;

use std::collections::HashMap;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::{Renderer, Texture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window)  {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    // Create the window
    let win = match video.window("SDL Tutorial 04", WIDTH, HEIGHT)
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

/// Load the textures we're going to use into an
/// easily indexable HashMap.
fn load_media(renderer: &Renderer) -> HashMap<&'static str, Box<Texture>> {
    let mut map: HashMap<&'static str, Box<Texture>> = HashMap::new();
    map.insert("up", Box::new(load_texture("resources/up.bmp", renderer)));
    map.insert("down", Box::new(load_texture("resources/down.bmp", renderer)));
    map.insert("left", Box::new(load_texture("resources/left.bmp", renderer)));
    map.insert("right", Box::new(load_texture("resources/right.bmp", renderer)));
    map.insert("press", Box::new(load_texture("resources/press.bmp", renderer)));
    map
}


fn main() {

    // Initialize SDL2
    let (sdl_context, window) = init();

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };

    // Load the sprite textures into an hashmap
    let sprites: HashMap<&'static str, Box<Texture>> = load_media(&renderer);

    // Start up the game loop
    let mut running: bool = true;
    let mut current_image: &str = "press";

    // Obtain the event pump
    let mut event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err)       => panic!("Could not obtain event pump: {}", err)
    };

    while running {
        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit {..} |
        //         Event::KeyDown { keycode: Keycode::Escape, .. } |
        //         Event::KeyDown { keycode: Keycode::Q, .. } => {
        //             running = false
        //         },
        //         Event::KeyDown { keycode: Keycode::Up, .. } => {
        //             current_image = "up"
        //         },
        //         Event::KeyDown { keycode: Keycode::Down, .. } => {
        //             current_image = "down"
        //         },
        //         Event::KeyDown { keycode: Keycode::Left, .. } => {
        //             current_image = "left"
        //         },
        //         Event::KeyDown { keycode: Keycode::Right, .. } => {
        //             current_image = "right"
        //         },
        //         Event::KeyDown { .. } => {
        //             current_image = "press"
        //         },
        //         _ => {}
        //     }
        // }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false
                },
                Event::KeyDown { keycode: k, ..} => match k {
                    
                    Some(Keycode::Escape) | Some(Keycode::Q) => {
                        running = false
                    },
                    Some(Keycode::Up) => {
                        current_image = "up"
                    },
                    Some(Keycode::Down) => {
                        current_image = "down"
                    },
                    Some(Keycode::Right) => {
                        current_image = "right"
                    },
                    Some(Keycode::Left) => {
                        current_image = "left"
                    },
                    Some(_) => {
                        current_image = "press"
                    },
                    None => {}
                },
                _ => {}
            }
        }

        // Clear and render the currently selected image
        renderer.clear();
        // sprites[current_image] yields a Box<Texture>, so we use
        // a '&' to reference it.
        renderer.copy(&sprites[current_image], None, None).unwrap();
        renderer.present();
    }
}

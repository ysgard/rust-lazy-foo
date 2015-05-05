extern crate sdl2;

use std::collections::HashMap;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, Texture};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::surface::{Surface};

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

/// Load the textures we're going to use into an
/// easily indexable HashMap.
fn load_media(renderer: &Renderer) -> HashMap<&'static str, Box<Texture>> {
    let mut map: HashMap<&'static str, Box<Texture>> = HashMap::new();
    map.insert("up", Box::new(load_texture("up.bmp", renderer)));
    map.insert("down", Box::new(load_texture("down.bmp", renderer)));
    map.insert("left", Box::new(load_texture("left.bmp", renderer)));
    map.insert("right", Box::new(load_texture("right.bmp", renderer)));
    map.insert("press", Box::new(load_texture("press.bmp", renderer)));
    map
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

    // Load the sprite textures into an hashmap
    let sprites: HashMap<&'static str, Box<Texture>> = load_media(&renderer);

    // Blit the initial image to the window.
    let mut context = renderer.drawer();

    // Start up the game loop
    let mut running: bool = true;
    let mut current_image: &str = "press";
    let mut event_pump = sdl_context.event_pump();

    while running {
        // We blit the image to the screen corresponding to the keypress,
        // or 'press' otherwise.  Using 'Esc' or 'q' will quit the program.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: KeyCode::Escape, .. } |
                Event::KeyDown { keycode: KeyCode::Q, .. } => {
                    running = false
                },
                Event::KeyDown { keycode: KeyCode::Up, .. } => {
                    current_image = "up"
                },
                Event::KeyDown { keycode: KeyCode::Down, .. } => {
                    current_image = "down"
                },
                Event::KeyDown { keycode: KeyCode::Left, .. } => {
                    current_image = "left"
                },
                Event::KeyDown { keycode: KeyCode::Right, .. } => {
                    current_image = "right"
                },
                Event::KeyDown { .. } => {
                    current_image = "press"
                },
                _ => {}
            }
        }

        // Clear and render the currently selected image
        context.clear();
        // sprites[current_image] yields a Box<Texture>, so we use
        // a '&' to reference it.
        context.copy(&sprites[current_image], None, None);
        context.present();
    }
}

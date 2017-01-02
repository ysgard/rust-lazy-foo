extern crate sdl2;
extern crate sdl2_image;

use std::collections::HashMap;
use std::path::Path;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, Texture};
use sdl2::event::Event;
use sdl2::scancode::ScanCode;
use sdl2::keyboard::get_keyboard_state;

use sdl2_image::{LoadTexture, INIT_PNG};


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

    sdl2_image::init(INIT_PNG);

    (sdl, win)
}

/// Take a string describing a path and use it to
/// load an image, and return its texture
fn load_texture(path: &'static str, renderer: &Renderer) -> Texture {
    match renderer.load_texture(Path::new(path)) {
        Ok(tex)    => tex,
        Err(err)   => panic!("Could not load texture: {}", err)
    }
}

/// Load the textures we're going to use into an
/// easily indexable HashMap.
fn load_media(renderer: &Renderer) -> HashMap<&'static str, Box<Texture>> {
    let mut map: HashMap<&'static str, Box<Texture>> = HashMap::new();
    map.insert("up", Box::new(load_texture("up.png", renderer)));
    map.insert("down", Box::new(load_texture("down.png", renderer)));
    map.insert("left", Box::new(load_texture("left.png", renderer)));
    map.insert("right", Box::new(load_texture("right.png", renderer)));
    map.insert("press", Box::new(load_texture("press.png", renderer)));
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
                Event::Quit {..} => {
                    running = false
                },
                _ => {}
            }
        }

        // Instead of using keyboard events to toggle the images,
        // use keyboard state.  Note the type is not necessary here,
        // it's just a reminder as to what we're dealing with.
        let keycode: HashMap<ScanCode, bool> = get_keyboard_state();

        // This strange-looking syntax is necessary because HashMap::get takes
        // a &K and returns a &V - so you have to pass in a reference, and
        // dereference the result.
        // We can get away with just using unwrap() here because a bool ('true' for
        // pressed, 'false' for not) is defined for every valid ScanCode - if we
        // pass in an invalid ScanCode, we'll get back 'None' and unwrap will
        // panic, which is surely what we want.

        if *keycode.get(&ScanCode::Up).unwrap() == true { current_image = "up"; }
        if *keycode.get(&ScanCode::Down).unwrap() == true { current_image = "down"; }
        if *keycode.get(&ScanCode::Right).unwrap() == true { current_image = "right"; }
        if *keycode.get(&ScanCode::Left).unwrap() == true { current_image = "left"; }
        
        // Clear and render the currently selected image
        context.clear();
        // sprites[current_image] yields a Box<Texture>, so we use
        // a '&' to reference it.
        context.copy(&sprites[current_image], None, None);
        context.present();
    }
}

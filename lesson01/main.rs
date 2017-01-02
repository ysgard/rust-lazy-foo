extern crate sdl2;

use sdl2::pixels::Color;
use std::time::Duration;
use std::thread::sleep;

// Set Screen dimensions
const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

fn main() {

    // Initialize SDL
    // Note that we can just call:
    // let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    // because 'unwrap' will panic if the result of the init call is not Ok, otherwise
    // it will unwrap Ok and assign the result to 'sdl_context'
    // We eschew .unwrap() here so we can output the exact SDL error that caused the failure.
    let sdl_context = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(err) => panic!("SDL could not initialize!  SDL_Error: {}", err),
    };

    // Get a handle to the underlying video subsystem
    let video = match sdl_context.video() {
        Ok(video) => video,
        Err(err) => panic!("Could not obtain handle to the video subsystem! SDL_Error: {}", err),
    };

    // Create a Window
    // Window::new and sdl2::init (and other funcs return an SdlResult, which
    // is just a wrapper around Result<T, string>.  Result can return one
    // of two values: Ok(T), or Err(string).  Use match to unwrap them.
    let window = match video.window("SDL Tutorial 1", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err) => panic!("SDL could not create a window! SDL_Error: {}", err),
        };

    // There's a major deviation from Lazy Foo's Lesson 1 here,
    // because rust-sdl2 (presumably for safety reasons) doesn't let you access
    // the window's Surface without going through a properties
    // function, which in turn requires that you pass it an event
    // pump so it can verify it's not running.  :-/
    // Instead, we'll obtain a renderer, and use that to update
    // the main window.
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("SDL could not create a renderer! SDL_Error: {}", err),
    };

    // Use the renderer it to clear and render the screen
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    // Pause for two seconds
    sleep(Duration::new(2,0));
    
    // Quit SDL Subsystems
    // Note that we don't have to explicitly call SDL_Quit, as `sdl_context`
    // calls it as it gets dropped
}
    
            
    

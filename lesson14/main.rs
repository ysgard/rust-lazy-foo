extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::{Renderer, Texture};
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use sdl2_image::{LoadSurface, INIT_PNG};

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

const FOO_IMG: &'static str = "resources/foo2.png";

// usize so we can mod the array index with it
// without having to cast.
const WALKING_FRAMES: usize = 4;

// Note: Starting with this tutorial we will eschew using either
// .unwrap() or matching on errors, instead we will use
// .expect(error_string), which essentially works like an unwrap,
// but will display the error_string as well as panicking.

// Create a struct that will track texture data
struct LTexture {
    // The actual texture.
    texture: Texture,
    // Image dimensions
    width: u32,
    height: u32
}

// Implement a few functions for the Texture struct
// Note that Rust doesn't put much focus on data hiding
// or other OOP concepts so we don't care about having
// getters and setters or the like.
//
// Instead, since Rust treats values as immutable by
// default, we don't have to worry about accidentally
// setting a struct field unless we create an LTexture
// using 'mut', in which case we take on the responsibility
// of ensuring the fields don't get messed with.
//
// This 'hands off' by default approach helps eliminate
// a lot of problems that, in OOP, are handled by boilerplate code.
// The result is cleaner, more consise and ultimately more safe.
#[allow(dead_code)]
impl LTexture {

    // create a new texture
    fn new(tex: Texture) -> LTexture {
        let w = tex.query().width;
        let h = tex.query().height;
        LTexture {
            texture: tex,
            width: w,
            height: h,
        }
    }

    // Load a texture from a file
    fn new_from_file(renderer: &Renderer, path: &Path) -> LTexture {
        // Load the surface first, so we can set the color key
        let mut surface = Surface::from_file(path)
            .expect("Could not load surface from path!");
        // Now set the color key on the surface
        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .expect("Could not set color key on the surface!");

        // Convert the surface to a texture and pass it to
        // LTexture::new to be wrapped
        let tex = renderer.create_texture_from_surface(&surface)
            .expect("Could not create texture from the surface!");
        LTexture::new(tex)
    }

    // Renders a texture to a given point using a provided renderer
    fn render_to(&self,
                 renderer: &mut Renderer,
                 x: i32,
                 y: i32,
                 clip: Option<Rect>) {
        let clip_rect = match clip {
            Some(rect) => rect,
            None       => Rect::new(0, 0, self.width, self.height) 
        };
        renderer.copy(&self.texture,
                     Some(clip_rect),
                      Some(Rect::new(x, y,
                                     clip_rect.width(),
                                     clip_rect.height())))
            .expect("Could not copy texture to the render target!");            
    }

    // Modulate the LTexture using a Color - this will 'tint' the texture
    // Note that LTextures are immutable, so we have to create a new one
    // and return it - we can't mutate ourselves.
    fn set_color(&mut self, color: Color) {
        let (r, g, b) = color.rgb();
        self.texture.set_color_mod(r, g, b);
    }

    // Set the alpha channel of the texture, controlling its transparency
    fn set_alpha(&mut self, alpha: u8) {
        self.texture.set_alpha_mod(alpha);
    }
}

// Note that 'renderer.load_texture' makes this example trivial.  See lesson03
// to show how we can manually load a surface and convert it to a texture.
    
/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window)  {
    let sdl = sdl2::init().expect("Could not initialize SDL!");
    let video = sdl.video().expect("Could not acquire the video context!");
    let win = video.window("SDL Tutorial 14", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create window!");

    sdl2_image::init(INIT_PNG).expect("Could not initialize sdl2_image!");
    
    (sdl, win)
}

// LoadMedia function
//
// We want to avoid the use of global variables (it's not really
// a Rust, or functional, idiom) so we return a tuple containing
// the data
fn load_media(renderer: &Renderer) ->
    (LTexture, [Rect; 4]) {
        // Return the teuple
        ( LTexture::new_from_file(renderer, Path::new(FOO_IMG)),
          [ Rect::new(0, 0, 64, 205),
            Rect::new(64, 0, 64, 205),
            Rect::new(128, 0, 64, 205),
            Rect::new(196, 0, 64, 205) ] )
}

fn main() {

    // Initialize SDL2
    let (sdl_context, window) = init();

    // obtain the renderer
    let mut renderer = window.renderer().build()
        .expect("Could not obtain renderer from window!");

    let (sprite_sheet, clips) = load_media(&renderer);
            
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump()
        .expect("Could not obtain event_pump!");

    // Set current frame to 0
    let mut frame: usize = 0;
   
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
        // Clear and render the texture each ass through the loop
        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();

        // Render the current frame
        let current_clip: Rect = clips[ frame % WALKING_FRAMES ];
        sprite_sheet.render_to(&mut renderer,
                               ((WIDTH - current_clip.width()) / 2) as i32,
                               ((HEIGHT - current_clip.height()) / 2) as i32,
                               Some(current_clip));
        
        // Update the screen
        renderer.present();

        // Increment the frame
        frame += 1;
        // Set the frame to 0 if we're at 4
        if frame == WALKING_FRAMES { frame = 0 };

        // This isn't in the tutorial, but we're going to pause 100ms
		// to give the animation a more graceful, smooth cadence
		std::thread::sleep(Duration::from_millis(100));
    }
}

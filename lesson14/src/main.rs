extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, RenderDrawer, Texture};
use sdl2::surface::{Surface};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect};

use sdl2_image::{LoadSurface, INIT_PNG};

const WIDTH:  i32 = 640;
const HEIGHT: i32 = 480;

// usize so we can mod the array index with it
// without having to cast.
const WALKING_FRAMES: usize = 4;

// Create a struct that will track texture data
struct LTexture {
    // The actual texture.
    texture: Texture,
    // Image dimensions
    width: i32,
    height: i32
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

    // create a copy of 

    // Load a texture from a file
    fn new_from_file(ren: &Renderer, path: &Path) -> LTexture {
        // Load the surface first, so we can set the color key
        let surface = match Surface::from_file(path) {
            Ok(surface) => surface,
            Err(err)    => panic!("Could not load surface: {}", err)
        };

        // Now set the color key on the surface
        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff)).unwrap();

        // Convert the surface to a texture and pass it to
        // LTexture::new to be wrapped
        let tex = match ren.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(err)    => panic!("Could not convert surface to texture: {}", err)
        };
        LTexture::new(tex)
    }

    // Renders a texture to a given point using a provided renderer
    fn render_to(&self,
                 context: &mut RenderDrawer,
                 x: i32,
                 y: i32,
                 clip: Option<Rect>) {
        let clip_rect = match clip {
            Some(rect) => rect,
            None       => Rect {x: 0, y: 0, w: self.width, h: self.height }
        };
        context.copy(&self.texture,
                     Some(clip_rect),
                     Some(Rect { x: x, y: y, w: clip_rect.w, h: clip_rect.h}) );            
    }

    // Modulate the LTexture using a Color - this will 'tint' the texture
    // Note that LTextures are immutable, so we have to create a new one
    // and return it - we can't mutate ourselves.
    fn set_color(&mut self, color: Color) {
        let (r, g, b) = color.get_rgb();
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

// LoadMedia function
//
// We want to avoid the use of global variables (it's not really
// a Rust, or functional, idiom) so we return a tuple containing
// the data
fn load_media(ren: &Renderer) ->
    (LTexture, [Rect; 4]) {
        // Return the teuple
        ( LTexture::new_from_file(ren, Path::new("foo.png")),
          [ Rect { x: 0, y: 0, w: 64, h: 205 },
            Rect { x: 64, y: 0, w: 64, h: 205 },
            Rect { x: 128, y: 0, w: 64, h: 205 },
            Rect { x: 196, y: 0, w: 64, h: 205 } ] )
}

fn main() {

    // Initialize SDL2
    let (sdl_context, window) = init();

    // obtain the renderer
    let mut renderer = match Renderer::from_window(window,
                                                   RenderDriverIndex::Auto,
                                                   ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };

    let (sprite_sheet, clips) = load_media(&renderer);
            
    let mut context = renderer.drawer();
    
    // running is 'mut' because we will want to 'flip' it to false when
    // we're ready to exit the game loop.
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump();

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
        // Clear and render the texture each pass through the loop
        context.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        context.clear();

        // Render the current frame
        let current_clip: Rect = clips[ frame % WALKING_FRAMES ];
        sprite_sheet.render_to(&mut context,
                               (WIDTH - current_clip.w) / 2,
                               (HEIGHT - current_clip.h) / 2,
                               Some(current_clip));
        
        // Update the screen
        context.present();

        // Increment the frame
        frame += 1;
        // Set the frame to 0 if we're at 4
        if frame == WALKING_FRAMES { frame = 0 };

        // This isn't in the tutorial, but we're going to pause 100ms
		// to give the animation a more graceful, smooth cadence
		sdl2::timer::delay(100);
    }
}

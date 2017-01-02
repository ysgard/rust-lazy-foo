extern crate sdl2;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::{Renderer, Texture};
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use sdl2::image::{LoadSurface, INIT_PNG, Sdl2ImageContext};

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

const IMG_ARROW: &'static str = "resources/arrow.png";

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
            .expect("Could not load surface from file!");
        
        // Now set the color key on the surface
        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .expect("Could not set color_key on surface!");

        // Convert the surface to a texture and pass it to
        // LTexture::new to be wrapped
        let tex = renderer.create_texture_from_surface(&surface)
            .expect("Could not create texture from surface!");

        LTexture::new(tex)
    }

    // Renders a texture to a given point using a provided renderer
    // provide additional arguments for rotation and flipping
    // Rust doesn't provide default arguments, and it seems overkill
    // to provide additional function signatures for this, so we're
    // going to wrap rotation and flipping args in Option<> so we can
    // provide None when we don't care about it.
    fn render_to(&self,
                 renderer: &mut Renderer,
                 x: i32,
                 y: i32,
                 clip: Option<Rect>,
                 rotation: Option<f64>,
                 center: Option<Point>,
                 flip_h: bool,
                 flip_v: bool
                 ) {
        let clip_rect = match clip {
            Some(rect) => rect,
            None       => Rect::new(0, 0, self.width, self.height),
        };
        let rot: f64 = match rotation {
            Some(rot) => rot,
            None      => 0.0
        };
        
        renderer.copy_ex(&self.texture,
                         Some(clip_rect),
                         Some(Rect::new(x, y,
                                        clip_rect.width(),
                                        clip_rect.height())),
                         rot,
                         center,
                         flip_h,
                         flip_v)
            .expect("Could not blit texture to render target!");
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
fn init() -> (Sdl, Window, Sdl2ImageContext)  {
    let sdl = sdl2::init()
        .expect("Could not initialize SDL!");
    let video = sdl.video()
        .expect("Could not obtain video from sdl context!");
    let win = video.window("SDL Tutorial 15", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create Window!");
   

    let image = sdl2::image::init(INIT_PNG)
        .expect("Could not initialize sdl2_image!");
    
    (sdl, win, image)
}

fn main() {

    // Initialize SDL2
    let (sdl_context, window, _image) = init();

    // obtain the renderer
    let mut renderer = window.renderer().build()
        .expect("Could not obtain renderer from window!");

    let arrow = LTexture::new_from_file(&renderer, Path::new(IMG_ARROW));
            
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump()
        .expect("Could not obtain event pump!");
    
    // Track current rotation and flips
    let mut degrees: f64 = 0.0;
    let mut flip_vertical: bool = false;
    let mut flip_horizontal: bool = false;
   
    // game loop
    while running {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit {..} => {
                    running = false
                },
                Event::KeyDown { keycode: k, .. } => match k {
                    Some(Keycode::A) => {
                        degrees -= 60.0;
                    },
                    Some(Keycode::D) => {
                        degrees += 60.0;
                    },
                    Some(Keycode::Q) => {
                        flip_horizontal = !flip_horizontal;
                    },
                    Some(Keycode::W) => {
                        flip_horizontal = false;
                        flip_vertical = false;
                    },
                    Some(Keycode::E) => {
                        flip_vertical = !flip_vertical;
                    },
                    Some(Keycode::Escape) => {
                        running = false;
                    },
                    Some(_) => {},
                    None => {},
                },
                _ => {},
            }
        }
        // Clear and render the texture each pass through the loop
        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();

        // Render the arrow
        arrow.render_to(&mut renderer,
                        (WIDTH - arrow.width) as i32 / 2,
                        (HEIGHT - arrow.height) as i32 / 2,
                        None,
                        Some(degrees),
                        None,
                        flip_horizontal,
                        flip_vertical);
        
        // Update the screen
        renderer.present();
    }
}

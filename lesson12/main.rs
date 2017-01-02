extern crate sdl2;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::{Renderer, Texture};
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::image::{LoadSurface, INIT_PNG, Sdl2ImageContext};

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

const IMG_COLORS: &'static str = "resources/colors.png";

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
        let mut surface = match Surface::from_file(path) {
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
            .unwrap();            
    }

    // Modulate the LTexture using a Color - this will 'tint' the texture
    // Note that LTextures are immutable, so we have to create a new one
    // and return it - we can't mutate ourselves.
    fn set_color(&mut self, color: Color) {
        let (r, g, b) = color.rgb();
        self.texture.set_color_mod(r, g, b);
    }
}

// Note that 'renderer.load_texture' makes this example trivial.  See lesson03
// to show how we can manually load a surface and convert it to a texture.
    
/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window, Sdl2ImageContext)  {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let win = match video.window("SDL Tutorial 12", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("Failed to create Window!: {}", err)
        };

    let image = sdl2::image::init(INIT_PNG).unwrap();
    
    (sdl, win, image)
}

fn main() {

    // Initialize SDL2
    let (sdl_context, window, _image) = init();

    // obtain the renderer
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Could not obtain renderer: {}", err)
    };

    // Create the textures we are going to use.
    let mut texture = LTexture::new_from_file(&renderer,
                                              Path::new(IMG_COLORS));
            
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create the Color we're going to use to modulate the texture
    // As we're allowing the user to alter this tint, we will make
    // it mutable
    let mut red_tint: u8 = 0xff;
    let mut green_tint: u8 = 0xff;
    let mut blue_tint: u8 = 0xff;
   
    // game loop
    while running {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit {..} => {
                    running = false
                },
                Event::KeyDown { keycode: k, ..} => match k {
                // The keys 'q', 'w' and 'e' increase the red, green and blue of the tint,
                // the keys 'a', 's' and 'd' decrease them.  We check to make sure we don't
                    // overflow the 1-byte value for each color channel.
                    Some(Keycode::Escape) => {
                        running = false
                    },
                    Some(Keycode::Q) => {
                        if red_tint < 224 { 
                            red_tint += 32;
                        }
                    },
                    Some(Keycode::W) => {
                        if green_tint < 224 {
                            green_tint += 32;
                        }
                    },
                    Some(Keycode::E) => {
                        if blue_tint < 224 {  
                            blue_tint += 32;
                        }
                    },
                    Some(Keycode::A) => {
                        if red_tint > 32 {
                            red_tint -= 32;
                        }
                    },
                    Some(Keycode::S) => {
                        if green_tint > 32 {
                            green_tint -= 32;
                        }
                    },
                    Some(Keycode::D) => {
                        if blue_tint > 32 {
                            blue_tint -= 32;
                        }
                    },
                    Some(_) => {},
                    None => {}
                },
                _ => {}
            }
        }
        // Clear and render the texture each pass through the loop
        renderer.set_draw_color(Color::RGB(0x0, 0x0, 0x0));
        renderer.clear();

        // Tint the texture
        texture.set_color(Color::RGB(red_tint, green_tint, blue_tint));
        // Blit the texture
        texture.render_to(&mut renderer, 0, 0, None);

        // Update the screen
        renderer.present();
    }
}

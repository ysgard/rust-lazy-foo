extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, RenderDrawer, Texture};
use sdl2::surface::{Surface};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use sdl2_image::{LoadSurface, INIT_PNG};

use sdl2_ttf::Font;

const WIDTH:  i32 = 640;
const HEIGHT: i32 = 480;

const FONT_FILE: &'static str = "lazy.ttf";
const FONT_SIZE: i32 = 28;

// Create a struct that will track texture data
struct LTexture {
    // The actual texture.
    texture: Texture,
    // Image dimensions
    width: i32,
    height: i32
}

// Note the use of the #[allow(dead_code)] which turns off
// warnings about functions we don't use in this lesson.
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
    // provide additional arguments for rotation and flipping
    // Rust doesn't provide default arguments, and it seems overkill
    // to provide additional function signatures for this, so we're
    // going to wrap rotation and flipping args in Option<> so we can
    // provide None when we don't care about it.
    fn render_to(&self,
                 context: &mut RenderDrawer,
                 x: i32,
                 y: i32,
                 clip: Option<Rect>,
                 rotation: Option<f64>,
                 center: Option<Point>,
                 flip: Option<(bool, bool)>
                 ) {
        let clip_rect = match clip {
            Some(rect) => rect,
            None       => Rect {x: 0, y: 0, w: self.width, h: self.height }
        };
        let rot: f64 = match rotation {
            Some(rot) => rot,
            None      => 0.0
        };
        let flip = match flip {
            Some((flip_v, flip_h)) => (flip_v, flip_h),
            None                 => (false, false)
        };
        
        context.copy_ex(&self.texture,
                        Some(clip_rect),
                        Some(Rect { x: x, y: y, w: clip_rect.w, h: clip_rect.h}),
                        rot,
                        center,
                        flip);            
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

    fn load_from_rendered_text(renderer: &Renderer, font: &Font, text: &str, color: Color) -> LTexture {
        let text_surface: Surface = match font.render_str_solid(text, color) {
            Ok(surface) => surface,
            Err(e)      => panic!("Error! Could not create text surface: {}", e)
        };
        // Now create a texture from the surface using the supplied renderer
        let text_texture = match renderer.create_texture_from_surface(&text_surface) {
            Ok(texture) => texture,
            Err(e)      => panic!("Could not texturize text surface: {}", e)
        };
        // Return an LTexture using the given text_texture
        LTexture::new(text_texture)
    }
}

/// Load the font, and use it to create and return a new texture with
/// the rendered string
fn load_media(renderer: &Renderer) -> LTexture {
    // Load the font, using the font and size specified by the global constants
    let font = match Font::from_file(Path::new(FONT_FILE), FONT_SIZE) {
        Ok(font) => font,
        Err(e)   => panic!("Failed to load TTF font: {}", e)
    };

    // Now return a new LTexture using the supplied font and renderer
    LTexture::load_from_rendered_text(renderer, &font, "The quick brown fox jumps over the lazy dog", Color::RGB(0, 0, 0))
}
    


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
    sdl2_ttf::init();
    
    (sdl, win)
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

    let text = load_media(&renderer);
            
    let mut context = renderer.drawer();
    
    // running is 'mut' because we will want to 'flip' it to false when
    // we're ready to exit the game loop.
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump();

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

        // Render the text
        text.render_to(&mut context,
                       (WIDTH - text.width) / 2,
                       (HEIGHT - text.height) / 2,
                       None,
                       None,
                       None,
                       None);
        
        // Update the screen
        context.present();
    }
}

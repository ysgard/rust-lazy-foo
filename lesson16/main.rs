extern crate sdl2;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::{Renderer, Texture};
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use sdl2::image::{LoadSurface, INIT_PNG, Sdl2ImageContext};

use sdl2::ttf::{Sdl2TtfContext, Font};

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

const FONT_FILE: &'static str = "resources/lazy.ttf";
const FONT_SIZE: u16 = 28;

// Create a struct that will track texture data
struct LTexture {
    // The actual texture.
    texture: Texture,
    // Image dimensions
    width: u32,
    height: u32
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

    // Load a texture from a file
    fn new_from_file(renderer: &Renderer, path: &Path) -> LTexture {
        // Load the surface first, so we can set the color key
        let mut surface = Surface::from_file(path)
            .expect("Could not load surface from file!");
        // Now set the color key on the surface
        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff)).unwrap();

        // Convert the surface to a texture and pass it to
        // LTexture::new to be wrapped
        let tex = renderer.create_texture_from_surface(&surface)
            .expect("Could not convert surface to texture!");
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
            None       => Rect::new(0, 0, self.width, self.height)
        };
        let rot: f64 = match rotation {
            Some(rot) => rot,
            None      => 0.0
        };
        
        renderer.copy_ex(&self.texture,
                         Some(clip_rect),
                         Some(Rect::new(x, y, clip_rect.width(),
                                        clip_rect.height())),
                         rot,
                         center,
                         flip_h,
                         flip_v)
            .expect("Unable to blit texture to render target!");            
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

    fn load_from_rendered_text(renderer: &Renderer, font: &Font, text: &str, color: Color) -> LTexture {
        let text_surface: Surface = font.render(text)
            .solid(color)
            .expect("Could not create text surface!");
        // Now create a texture from the surface using the supplied renderer
        let text_texture = renderer.create_texture_from_surface(&text_surface)
            .expect("Could not convert text surface to texture!");
        // Return an LTexture using the given text_texture
        LTexture::new(text_texture)
    }
}

/// Load the font, and use it to create and return a new texture with
/// the rendered string
fn load_media(renderer: &Renderer, ttf: &Sdl2TtfContext) -> LTexture {
    // Load the font, using the font and size specified by the global constants
    let font = ttf.load_font(Path::new(FONT_FILE), FONT_SIZE)
        .expect("Could not load font from file!");

    // Now return a new LTexture using the supplied font and renderer
    LTexture::load_from_rendered_text(renderer, &font, "The quick brown fox jumps over the lazy dog", Color::RGB(0, 0, 0))
}
    
/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
// Ugh, the SDL font context name!
fn init() -> (Sdl, Window, Sdl2ImageContext, Sdl2TtfContext)  {
    let sdl = sdl2::init().expect("Could not initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let win = video.window("SDL Tutorial 16", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");

    let image = sdl2::image::init(INIT_PNG).expect("Could not initialize sdl2_image!");
    let ttf = sdl2::ttf::init().expect("Could not initialize sdl2_ttf!");
    
    (sdl, win, image, ttf)
}

fn main() {

    // Initialize SDL2
    let (sdl_context, window, _image, ttf_context) = init();

    // obtain the renderer
    let mut renderer = window.renderer().build()
        .expect("Could not create renderer!");
    
    let text = load_media(&renderer, &ttf_context);
            
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump()
        .expect("Could not obtain handle to event pump!");

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
        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();

        // Render the text
        text.render_to(&mut renderer,
                       (WIDTH - text.width) as i32 / 2,
                       (HEIGHT - text.height) as i32 / 2,
                       None,
                       None,
                       None,
                       false,
                       false);
        
        // Update the screen
        renderer.present();
    }
}

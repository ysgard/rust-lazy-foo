extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer, RenderDrawer, Texture, BlendMode};
use sdl2::surface::{Surface};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use sdl2_image::{LoadSurface, INIT_PNG};

const WIDTH:  i32 = 640;
const HEIGHT: i32 = 480;

const BUTTON_WIDTH: i32 = 300;
const BUTTON_HEIGHT: i32 = 200;
const TOTAL_BUTTONS: i32 = 4;

const BUTTON_SPRITESHEET: &'static str = "button.png"; 

// Rust enums are powerful, allowing you to create algebraic data types,
// but in the simplest case they can be used like C enums.
// We derive Copy and Clone for this enum so so we can pull the value out
// of the struct for conversion into an array index.  If we don't, Rust
// will try to pull the enum out, which leads to the common "cannot
// move out of borrowed content" error.
#[derive(Copy, Clone)]
enum LButtonSprite {
    ButtonSpriteMouseOut = 0,
    ButtonSpriteMouseOverMotion,
    ButtonSpriteMouseDown,
    ButtonSpriteMouseUp,
    ButtonSpriteTotal,
}

    
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

    // Didn't see it in earlier lessons, doesn't seem to be used here, including for completeness
    fn set_blend_mode(&mut self, blend: BlendMode) {
        self.texture.set_blend_mode(blend);
    }

    // We only include this function if sdl2_ttf is used
    #[cfg(sdl2_ttf)]
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


// Create a struct that will be used to track mouse data
struct LButton {
    // Current position of the mouse
    position: Point,
    // Currently used sprite
    current_sprite: LButtonSprite, 
}


// Implement some functions for our mouse tracker struct
#[allow(dead_code)]
impl LButton {

    // Return a newly initialized LButton (not really needed)
    fn new() -> LButton {
        LButton{ position: Point{ x: 0, y: 0 },
                 current_sprite: LButtonSprite::ButtonSpriteMouseOut}
    }

    // Create a new LButton with an initial point
    fn new_from_point(p: Point) -> LButton {
        LButton{ position: p, current_sprite:  LButtonSprite::ButtonSpriteMouseOut }
    }

    // Set the position
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = Point{ x: x, y: y };
    }

    // Handle a mouse event. 
    fn handle_event(&mut self, e: &Event) {
        // The LazyFoo tutorial uses the 'SDL_GetMouseState()' function to
        // obtain the x, y coordinates.  You may ask why we just don't access the
        // x & y coords inside the event itself.  The problem is that we'd have to
        // unwrap the event first, and pattern match on three different types of
        // events: MouseMotion, MouseButtonUp and MouseButtonDown.  It's definitely
        // doable, but simpler just to call get_mouse_state.
        let (_, x, y) = sdl2::mouse::get_mouse_state();
 
        // Check to see if the mouse is inside the button
        if (x < self.position.x) ||
            (x > self.position.x + BUTTON_WIDTH) ||
            (y < self.position.y) ||
            (y > self.position.y + BUTTON_HEIGHT) {
                self.current_sprite = LButtonSprite::ButtonSpriteMouseOut;
            }
        else {
            self.current_sprite = match *e {
                Event::MouseButtonDown {..} => LButtonSprite::ButtonSpriteMouseDown,
                Event::MouseButtonUp {..} => LButtonSprite::ButtonSpriteMouseUp,
                // If it's not an up or down button, it's just motion
                _ => LButtonSprite::ButtonSpriteMouseOverMotion
            }
        }
    }

    // Render a button.  In order to do this, we need the SDL context
    // as well as the LTexture for the button.
    fn render(&self, context: &mut RenderDrawer, texture: &LTexture, clips: &Vec<Rect>) {
        // This is why we need to derive the Copy trait for the enum.
        let indx = self.current_sprite as usize;
        texture.render_to(context,
                          self.position.x,
                          self.position.y,
                          Some(clips[indx]),
                          None,
                          None,
                          None);
    }
}
    

/// We take a deviation from the Lazy Foo tutorial here. In the tutorial, load_media
/// initializes a lot of global variables, which we try and avoid.  Instead load_media
/// just takes care of loading the image and creating its clip rectangles, we will
/// initialize the LButton array in the main loop itself.  That way load_media takes
/// care of only loading the media and its direct data structures (the clip rects),
/// while other initialization takes place elsewhere.
fn load_media(renderer: &Renderer) -> (LTexture, Vec<Rect>) {
    // Load the button sprite
    let button_sprite = LTexture::new_from_file(renderer, Path::new(BUTTON_SPRITESHEET));

    // Create an array.  We use a vector because if we created an array, we'd have to
    // modify its contents in the following loop, and therefore would need to make
    // it mutable.  It's simpler (and safer) to create a Vec and push immutable Rects onto it.
    let mut clip_rects: Vec<Rect> = Vec::new();
    // Create an array of clip rects
    for i in 0..TOTAL_BUTTONS {
        clip_rects.push(Rect{ x: 0, y: i * 200, w: BUTTON_WIDTH, h: BUTTON_HEIGHT });
    }
    (button_sprite, clip_rects)
}


/// We will create the buttons here.  We will use an array instead of a vec because
/// it makes no difference to the program, but is illustrative.
fn initialize_buttons() -> [LButton; 4] {
    // We don't use set_position, because that would require making the buttons mutable
    // Instead, just initialize them from scratch.
    [
        LButton::new(),
        LButton::new_from_point(Point{ x: WIDTH - BUTTON_WIDTH, y: 0 }),
        LButton::new_from_point(Point{ x: 0, y: HEIGHT - BUTTON_HEIGHT }),
        LButton::new_from_point(Point{ x: WIDTH - BUTTON_WIDTH, y: HEIGHT - BUTTON_HEIGHT }),
        ]
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
    // sdl2_ttf::init();
    
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

    let (button_texture, clip_rects) = load_media(&renderer);
    let mut buttons = initialize_buttons();
            
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

                // Pass off the events to the buttons for handling.
                // Note that unlike the tutorial, we actually check it's
                // a mouse event before handing it off.  Otherwise in an
                // actual program we'd be sending non-mouse events into limbo.
                Event::MouseMotion {..} |
                Event::MouseButtonDown {..} |
                Event::MouseButtonUp {..} => {
                    for i in 0..TOTAL_BUTTONS {
                        buttons[i as usize].handle_event(&event);
                    }
                },      
                Event::Quit {..} => {
                    running = false
                },
                _ => {}
            }
        }
        // Clear and render the texture each pass through the loop
        context.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        context.clear();

        // Render the buttons
        // We don't have globals and LButton does not store the button texture,
        // so we need to pass it and the context.
        for i in 0..TOTAL_BUTTONS {
            buttons[i as usize].render(&mut context,
                                     &button_texture,
                                     &clip_rects);
        }

        // Update the screen
        context.present();
    }
}

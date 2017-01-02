extern crate sdl2;

use std::path::Path;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::{Renderer, Texture};
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::mouse::{MouseState};

use sdl2::image::{LoadSurface, INIT_PNG, Sdl2ImageContext};

const WIDTH:  u32 = 640;
const HEIGHT: u32 = 480;

const BUTTON_WIDTH: u32 = 300;
const BUTTON_HEIGHT: u32 = 200;
const TOTAL_BUTTONS: u32 = 4;

const BUTTON_SPRITESHEET: &'static str = "resources/button.png"; 

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
    //ButtonSpriteTotal,
}

    
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
        surface.set_color_key(true, Color::RGB(0, 0xff, 0xff))
            .expect("Could not set color key on surface!");

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
            None       => Rect::new(0, 0, self.width, self.height)
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
                        flip_v).unwrap();            
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

    // We only include this function if sdl2_ttf is used
    #[cfg(sdl2_ttf)]
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



// Create a struct that will be used to track mouse data
struct LButton {
    // Current position of the mouse
    position: Point,
    // Currently used sprite
    current_sprite: LButtonSprite,
    pressed: bool,
}


// Implement some functions for our mouse tracker struct
#[allow(dead_code)]
impl LButton {

    // Return a newly initialized LButton (not really needed)
    fn new() -> LButton {
        LButton{ position: Point::new(0, 0),
                 current_sprite: LButtonSprite::ButtonSpriteMouseOut,
                 pressed: false }
    }

    // Create a new LButton with an initial point
    fn new_from_point(p: Point) -> LButton {
        LButton{ position: p,
                 current_sprite:  LButtonSprite::ButtonSpriteMouseOut,
                 pressed: false }
    }

    // Set the position
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = Point::new(x, y);
    }

    // Handle a mouse event. 
    fn handle_event(&mut self, s: &MouseState) {
        // The LazyFoo tutorial uses the 'SDL_GetMouseState()' function to
        // obtain the x, y coordinates.  That would require passing the event pump
        // into the function, so we simplify things a little bit by acquiring the
        // mouse state in the main game loop and passing in the state to handle_event
 
        // Check to see if the mouse is inside the button
        if (s.x() < self.position.x()) ||
            (s.x() > self.position.x() + BUTTON_WIDTH as i32) ||
            (s.y() < self.position.y()) ||
            (s.y() > self.position.y() + BUTTON_HEIGHT as i32) {
                self.current_sprite = LButtonSprite::ButtonSpriteMouseOut;
            }
        else {
            self.current_sprite = match s.left() {
                true => {
                    self.pressed = true;
                    LButtonSprite::ButtonSpriteMouseDown
                },
                false => {
                    if self.pressed == true {
                        LButtonSprite::ButtonSpriteMouseUp
                    } else {
                        LButtonSprite::ButtonSpriteMouseOverMotion
                    }
                }
            }
        }
    }

    // Render a button.  In order to do this, we need the SDL context
    // as well as the LTexture for the button.
    fn render(&self, renderer: &mut Renderer, texture: &LTexture, clips: &Vec<Rect>) {
        // This is why we need to derive the Copy trait for the enum.
        let indx = self.current_sprite as usize;
        texture.render_to(renderer,
                          self.position.x(),
                          self.position.y(),
                          Some(clips[indx]),
                          None,
                          None,
                          false,
                          false);
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
        clip_rects.push(Rect::new(0, i as i32 * 200, BUTTON_WIDTH, BUTTON_HEIGHT));
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
        LButton::new_from_point(Point::new((WIDTH - BUTTON_WIDTH) as i32, 0)),
        LButton::new_from_point(Point::new(0, (HEIGHT - BUTTON_HEIGHT) as i32)),
        LButton::new_from_point(Point::new((WIDTH - BUTTON_WIDTH) as i32,
                                           (HEIGHT - BUTTON_HEIGHT) as i32))
    ]
}

/// Break out initialization into a separate function, which
/// returns only the Window (we don't need the sdl_context)
fn init() -> (Sdl, Window, Sdl2ImageContext)  {

    let sdl = sdl2::init().expect("Unable to initialize SDL!");
    let video = sdl.video().expect("Could not acquire video context!");
    let win = video.window("SDL Tutorial 17", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not create SDL window!");
                      

    let image = sdl2::image::init(INIT_PNG).expect("Unable to initialize sdl2_image!");
    
    (sdl, win, image)
}

fn main() {

    // Initialize SDL2
    let (sdl_context, window, _image) = init();

    // obtain the renderer
    let mut renderer = window.renderer().build()
        .expect("Unable to obtain renderer!");

    let (button_texture, clip_rects) = load_media(&renderer);
    let mut buttons = initialize_buttons();
            
    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump()
        .expect("Unable to obtain event pump handle!");

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
                // UPDATE: This is the old way of doing it.  Now mouse events
                // are not generated events - instead you pull the status of
                // the mouse right from the event pump (see below).
                
                // Event::MouseMotion {..} |
                // Event::MouseButtonDown {..} |
                // Event::MouseButtonUp {..} => {
                //     for i in 0..TOTAL_BUTTONS {
                //         buttons[i as usize].handle_event(&event);
                //     }
                // },      
                Event::Quit {..} => {
                    running = false
                },
                _ => {}
            }
        }

        // Check the mouse state, & dispatch it to the buttons
        let state = event_pump.mouse_state();
        for i in 0..TOTAL_BUTTONS {
            buttons[i as usize].handle_event(&state);
        }

        // Check the 
        // Clear and render the texture each pass through the loop
        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();

        // Render the buttons
        // We don't have globals and LButton does not store the button texture,
        // so we need to pass it and the context.
        for i in 0..TOTAL_BUTTONS {
            buttons[i as usize].render(&mut renderer,
                                     &button_texture,
                                     &clip_rects);
        }

        // Update the screen
        renderer.present();
    }
}

# Lazy Foo's SDL2 Tutorials, in Rust #

This repo contains ports of the
[Lazy Foo SDL2](http://lazyfoo.net/tutorials/SDL/index.php) tutorials
to the [Rust](http://www.rust-lang.org) programming language, using
the Rust wrappers kindly provided by the following awesome people:

+ SDL2:
[AngryLawyer's rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
+ SDL2_image:
[xsleonard's rust-sdl2_image](https://github.com/xsleonard/rust-sdl2_image)
+ SDL2_ttf:
[andelf's rust-sdl2_ttf](https://github.com/andelf/rust-sdl2_ttf)

## Tutorial Index

I plan to port most of the provided tutorials, though for some I can't
create a port I lack the resourced - for example, I don't have a
gamepad or forcefeedback device, and at this point I can't create
anything for mobile.  

* Lesson 01 - Hello SDL
* Lesson 02 - Getting an Image on the Screen
* Lesson 03 - Event Driven Programming
* Lesson 04 - Key Presses
* Lesson 05 - Optimized Surface Loading and Soft Stretching
* Lesson 06 - Extension Libraries and Loading Other Image Formats
* Lesson 07 - Texture Loading and Rendering
* Lesson 08 - Geometry Rendering
* Lesson 09 - The Viewport
* Lesson 10 - Color Keying
* Lesson 11 - Clip Rendering and Sprite Sheets
* Lesson 12 - Color Modulation
* Lesson 13 - Alpha Blending
* Lesson 14 - Animated Sprites and Vsync
* Lesson 15 - Rotation and Flipping
* Lesson 16 - True Type Fonts
* Lesson 17 - Mouse Events
* Lesson 18 - Key States
* (NOT IMPLEMENTED) Lesson 19 - Gamepads and Joysticks
* (NOT IMPLEMENTED) Lesson 20 - Force Feedback
* (TODO) Lesson 21 - Sound Effects and Music
* (TODO) Lesson 22 - Timing
* (TODO) Lesson 23 - Advanced Timers
* (TODO) Lesson 24 - Calculating Frame Rate
* (TODO) Lesson 25 - Capping Frame Rate
* (TODO) Lesson 26 - Motion
* (TODO) Lesson 27 - Collision Detection
* (TODO) Lesson 28 - Per-pixel Collision Detection
* (TODO) Lesson 29 - Circular Collision Detection
* (TODO) Lesson 30 - Scrolling
* (TODO) Lesson 31 - Scrolling Backgrounds
* (TODO) Lesson 32 - Text Input and Clipboard Handling
* (TODO) Lesson 33 - File Reading and Writing
* (NOT IMPLEMENTED) Lesson 34 - Audio Recording
* (TODO) Lesson 35 - Window Events
* (TODO) Lesson 36 - Multiple Windows
* (TODO) Lesson 37 - Multiple Displays
* (TODO) Lesson 38 - Particle Engines
* (TODO) Lesson 39 - Tiling
* (TODO) Lesson 40 - Texture Manipulation
* (TODO) Lesson 41 - Bitmap Fonts
* (TODO) Lesson 42 - Texture Streaming
* (TODO) Lesson 43 - Render to Texture
* (TODO) Lesson 44 - Frame Independent Movement
* (TODO) Lesson 45 - Timer Callbacks
* (TODO) Lesson 46 - Multithreading
* (TODO) Lesson 47 - Semaphores
* (TODO) Lesson 48 - Atomic Operations
* (TODO) Lesson 49 - Mutexes and Conditions
* (TODO) Lesson 50 - SDL and OpenGL 2
* (TODO) Lesson 51 - SDL and Modern OpenGL
* (NOT IMPLEMENTED) Lesson 52 - Hello Mobile
* (NOT IMPLEMENTED) Lesson 53 - Extensions and Changing Orientation
* (NOT IMPLEMENTED) Lesson 54 - Touches
* (NOT IMPLEMENTED) Lesson 55 - Multitouch

## Requirements

To run any of these examples, you will need two things:

1. The [nightly build](http://www.rust-lang.org/install.html) of Rust.  The
stable 1.0 build will likely work for the majority of these examples,
but I think I used a few unstable features here and there.

2. [The SDL2 Development libraries](https://www.libsdl.org/download-2.0.php).  
You will also need the image library, [SDL_Image 2.0](https://www.libsdl.org/projects/SDL_image/);
and the truetype library, [SDL_TTF 2.0](https://www.libsdl.org/projects/SDL_ttf/).

On OS X, you can use Homebrew:

	brew install sdl2
    brew install sdl2_image
    brew install sdl2_ttf

For other platforms, refer to your existing package documentation.

## Compiling and Running the Examples

Once you have a version of rust installed, just go into any of the
lesson folders and run

    cargo run




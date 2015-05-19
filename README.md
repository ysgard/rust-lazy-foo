# Lazy Foo's SDL2 Tutorials, in Rust #

This repo contains ports of the
[Lazy Foo SDL2](http://lazyfoo.net/tutorials/SDL/index.php) tutorials
to the [Rust](http://www.rust-lang.org) programming language.

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




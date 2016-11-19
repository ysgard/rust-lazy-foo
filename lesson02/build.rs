// build.rs

use std::env;
use std::fs::{copy, File};
use std::path::Path;


fn main() {
    let files = ["hello_world.bmp"];
    let out_dir = env::var("OUT_DIR").unwrap();
    for f in files {
        let dest_path = Path::new(&out_dir);
        fs::copy(&dest_path, f).unwrap();
    }
}

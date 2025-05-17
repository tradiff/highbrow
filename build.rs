use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let source_svg = Path::new(&manifest_dir).join("src/highbrow.svg");

    let out_dir = env::var("OUT_DIR").unwrap();

    // Find the target directory (go up 3 levels from OUT_DIR)
    let mut target_dir = Path::new(&out_dir).to_path_buf();
    for _ in 0..3 {
        target_dir = target_dir.parent().unwrap().to_path_buf();
    }

    let dest_svg = target_dir.join("highbrow.svg");

    println!("Copying SVG icon from: {:?} to: {:?}", source_svg, dest_svg);
    if let Err(e) = fs::copy(source_svg, dest_svg) {
        println!("cargo:warning=Failed to copy SVG file: {}", e);
    }

    // Tell Cargo to rerun this script if the SVG file changes
    println!("cargo:rerun-if-changed=src/highbrow.svg");
}

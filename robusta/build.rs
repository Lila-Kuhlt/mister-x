use specta;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const FRONTEND_DIR: &str = "liberica";

fn main() {
    // Get the project directory
    let project_dir = env::current_dir().unwrap();

    // Build the path to the `liberica` directory
    let liberica_dir = std::env::var("LIBERICA_DIR")
        .map(PathBuf::from)
        .unwrap_or(project_dir.parent().unwrap().join("liberica"));

    for path in ["package.json", "src", "tsconfig.json", "index.html"] {
        println!("cargo:rerun-if-changed={}/{}", liberica_dir.to_string_lossy(), path);
    }
    println!("cargo:warning=Building Liberica");

    // Change into the `liberica` directory
    env::set_current_dir(liberica_dir).unwrap();

    // Run `npm install`
    let npm_install = Command::new("npm")
        .arg("install")
        .output()
        .expect("Failed to run `npm install`");

    // Check for errors in `npm install`
    if !npm_install.status.success() {
        panic!(
            "`npm install` failed: {}",
            String::from_utf8_lossy(&npm_install.stderr)
        );
    }

    // Run `npm run build`
    let npm_build = Command::new("npm")
        .arg("run")
        .arg("build:vite")
        .output()
        .expect("Failed to run `npm run build`");

    // Check for errors in `npm run build`
    if !npm_build.status.success() {
        panic!(
            "`npm run build` failed: {}",
            String::from_utf8_lossy(&npm_build.stderr)
        );
    }

    // Optionally, change back to the original directory
    env::set_current_dir(project_dir).unwrap();
}

use std::env;
use std::process::Command;

fn main() {
    // Get the project directory
    let project_dir = env::current_dir().unwrap();

    // Build the path to the `liberica` directory
    let liberica_dir = project_dir.parent().unwrap().join("liberica");

    // Change into the `liberica` directory
    env::set_current_dir(&liberica_dir).unwrap();

    // Tell Cargo to only rerun this script if something in `../liberica` changes
    println!("cargo:rerun-if-changed={}", liberica_dir.display());

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
        .arg("build")
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

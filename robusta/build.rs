use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the project directory
    let project_dir = env::current_dir().unwrap();

    // Build the path to the `liberica` directory
    let liberica_dir = std::env::var("LIBERICA_DIR")
        .map(PathBuf::from)
        .unwrap_or(project_dir.parent().unwrap().join("liberica"));

    for path in ["package.json", "src", "tsconfig.json", "index.html"] {
        println!(
            "cargo:rerun-if-changed={}/{}",
            liberica_dir.to_string_lossy(),
            path
        );
    }
    if std::env::var("BUILD_FRONTEND").is_err() {
        println!("cargo:warning=Skipping Liberica build");
        println!("cargo:warning=Set `BUILD_FRONTED` to enable automatic liberica rebuilds");
        return;
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
        println!(
            "cargo:warning=`npm install` failed: {}",
            String::from_utf8_lossy(&npm_install.stderr).replace('\n', "\\n")
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
        println!(
            "cargo:warning=`npm run build` failed: {}",
            String::from_utf8_lossy(&npm_build.stderr).replace('\n', "\\n")
        );
    }

    // Optionally, change back to the original directory
    env::set_current_dir(project_dir).unwrap();
}

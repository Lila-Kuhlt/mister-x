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
        println!("cargo:rerun-if-changed={}/{}", liberica_dir.to_string_lossy(), path);
    }
    println!("cargo:rerun-if-env-changed=BUILD_FRONTEND");
    if std::env::var("BUILD_FRONTEND").is_err() {
        println!("cargo:warning=Skipping Liberica build");
        println!("cargo:warning=Set `BUILD_FRONTEND` to enable automatic liberica rebuilds");
        return;
    }

    println!("cargo:warning=Building Liberica");

    // Change into the `liberica` directory
    env::set_current_dir(liberica_dir).unwrap();

    // Run `bun install`
    let bun_install = Command::new("bun")
        .arg("install")
        .output()
        .expect("Failed to run `bun install`, is bun installed?");

    // Check for errors in `bun install`
    if !bun_install.status.success() {
        println!(
            "cargo:warning=`bun install` failed: {}",
            String::from_utf8_lossy(&bun_install.stderr).replace('\n', "\ncargo:warning="),
        );
    }

    // Run `bun run build`
    let bun_build = Command::new("bun")
        .arg("run")
        .arg("build")
        .output()
        .expect("Failed to run `bun run build`");

    // Check for errors in `bun run build`
    if !bun_build.status.success() {
        println!(
            "cargo:warning=`bun run build` failed: {}",
            String::from_utf8_lossy(&bun_build.stderr).replace('\n', "\ncargo:warning="),
        );
    }

    // Optionally, change back to the original directory
    env::set_current_dir(project_dir).unwrap();
}

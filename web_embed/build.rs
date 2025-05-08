use std::{env, path::Path, process::Command};

fn main() {
    let frontend_dir = Path::new("frontend");

    // Check if the frontend build should be skipped
    let skip_frontend = env::var("SKIP_FRONTEND_BUILD").unwrap_or("0".to_string());
    if skip_frontend != "0" {
        println!("Skipping frontend build.");
        return;
    }

    // Execute the "bun run build" command in the "frontend" directory.
    let status = Command::new("bun")
        .arg("run")
        .arg("build")
        .current_dir(frontend_dir)
        .status()
        .expect("Failed to execute bun command");

    // If the frontend build failed, exit with an error code so that Cargo build fails.
    if !status.success() {
        panic!("Frontend build failed.");
    }
}

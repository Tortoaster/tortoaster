use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");

    Command::new("tailwind")
        .arg("-i")
        .arg("./input.css")
        .arg("-o")
        .arg("./static/style.css")
        .arg("--minify")
        .output()
        .expect("failed to run tailwind");
}

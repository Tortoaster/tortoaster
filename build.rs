use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates");

    Command::new("tailwind")
        .arg("-i")
        .arg("./input.css")
        .arg("-o")
        .arg("./static/style.css")
        .arg("--minify")
        .output()
        .expect("failed to run tailwind");
}

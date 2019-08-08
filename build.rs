use std::process::Command;

fn main() {
    let mut cmd = Command::new("go")
        .arg("build")
        .arg("-o")
        .arg("libivy.a")
        .arg("-buildmode=c-archive")
        .arg("-ldflags=-s -w")
        .arg("libivy.go")
        .current_dir("go")
        .env("GO111MODULE", "on")
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
    println!("cargo:rustc-link-lib=static=ivy");
    println!("cargo:rustc-link-search=native=./go/");
    println!("cargo:rerun-if-changed=./go/");
}

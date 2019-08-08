use std::process::Command;

fn main() {
	Command::new("/bin/bash")
                .arg("-c")
                .arg("cd go; GO111MODULE=on go build -o libivy.a -buildmode=c-archive -ldflags='-s -w' libivy.go")
                .output()
                .unwrap();
	println!("cargo:rustc-link-lib=static=ivy");
	println!("cargo:rustc-link-search=native=./go/");
	println!("cargo:rerun-if-changed=./go/");
}

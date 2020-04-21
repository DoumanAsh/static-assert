use std::process::Command;
use std::env;

fn main() {
    let cmd = env::var_os("RUSTC").unwrap_or_else(|| std::ffi::OsString::from("rustc"));
    let output = Command::new(cmd).arg("-V").output().expect("Successfully execute command");
    let output = core::str::from_utf8(&output.stdout).expect("Cannot read stdout as UTF-8").trim();

    if output.contains("nightly") {
        println!("cargo:rustc-cfg=NIGHTLY");
    }
}

use std::env;
use std::process::Command;
use std::str;

fn main() {
    let compiler = match rustc_version() {
        Some(compiler) => compiler,
        None => return,
    };

    if compiler.minor < 33 {
        println!("cargo:rustc-cfg=proc_macro_hack_no_crate_as_underscore");
    }
}

struct Compiler {
    minor: u32,
}

fn rustc_version() -> Option<Compiler> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    let minor = pieces.next()?.parse().ok()?;
    Some(Compiler { minor })
}

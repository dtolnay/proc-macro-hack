use std::env;
use std::fs;
use std::iter;
use std::path::Path;
use std::process::Command;
use std::str;

/*
#[doc(hidden)]
#[macro_export]
macro_rules! count {
    () => { proc_macro_call_0!() };
    (!) => { proc_macro_call_1!() };
    (!!) => { proc_macro_call_2!() };
    ...
}
*/

// The rustc-cfg strings below are *not* public API. Please let us know by
// opening a GitHub issue if your build environment requires some way to enable
// these cfgs other than by executing our build script.
fn main() {
    // Tell Cargo not to rerun on src/lib.rs changes.
    println!("cargo:rerun-if-changed=build.rs");

    let minor = match rustc_minor_version() {
        Some(minor) => minor,
        None => return,
    };

    // Function like procedural macros in expressions patterns statements stabilized in Rust 1.45:
    // https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#stabilizing-function-like-procedural-macros-in-expressions-patterns-and-statements
    if minor >= 45 {
        println!("cargo:rustc-cfg=fn_like_proc_macro");
        // proc-macro-hack does nothing in these versions, so proc-macro-nested
        // doesn't need to do anything either.
        return;
    }

    let mut content = String::new();
    content += "#[doc(hidden)]\n";
    content += "#[macro_export]\n";
    content += "macro_rules! count {\n";
    for i in 0..=64 {
        let bangs = iter::repeat("!").take(i).collect::<String>();
        content += &format!("    ({}) => {{ proc_macro_call_{}!() }};\n", bangs, i);
    }
    content += "    ($(!)+) => {\n";
    content += "        compile_error! {\n";
    content += "            \"this macro does not support >64 nested macro invocations\"\n";
    content += "        }\n";
    content += "    };\n";
    content += "}\n";

    let content = content.as_bytes();
    let out_dir = env::var("OUT_DIR").unwrap();
    let ref dest_path = Path::new(&out_dir).join("count.rs");

    // Avoid bumping filetime if content is up to date. Possibly related to
    // https://github.com/dtolnay/proc-macro-hack/issues/56 ...?
    if fs::read(dest_path)
        .map(|existing| existing != content)
        .unwrap_or(true)
    {
        fs::write(dest_path, content).unwrap();
    }
}

fn rustc_minor_version() -> Option<u32> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    pieces.next()?.parse().ok()
}

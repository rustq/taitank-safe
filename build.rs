use std::process::Command;

fn main() {

    Command::new("git")
        .args(["submodule", "init"])
        .status()
        .expect("Unable to initialize git submodules");
    Command::new("git")
        .args(["submodule", "update"])
        .status()
        .expect("Unable to update the submodule repositories");

    cxx_build::bridge("src/lib.rs")
        .file("src/taitank_safe.cc")
        .file("include/taitank/src/taitank.cc")
        .file("include/taitank/src/taitank_style.cc")
        .file("include/taitank/src/taitank_node.cc")
        .file("include/taitank/src/taitank_flexline.cc")
        .file("include/taitank/src/taitank_flex.cc")
        .file("include/taitank/src/taitank_config.cc")
        .file("include/taitank/src/taitank_cache.cc")
        .file("include/taitank/src/taitank_util.cc")
        .std("c++17")
        .compile("taitank-safe");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/taitank_safe.cc");
    println!("cargo:rerun-if-changed=include/taitank_safe.h");
}
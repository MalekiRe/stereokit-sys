extern crate bindgen;

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        match name {
            "FP_NAN" => MacroParsingBehavior::Ignore,
            "FP_INFINITE" => MacroParsingBehavior::Ignore,
            "FP_ZERO" => MacroParsingBehavior::Ignore,
            "FP_SUBNORMAL" => MacroParsingBehavior::Ignore,
            "FP_NORMAL" => MacroParsingBehavior::Ignore,
            _ => MacroParsingBehavior::Default,
        }
    }
}

macro_rules! cargo_cmake_feat {
    ($feature:literal) => {
        if cfg!(feature = $feature) {
            "ON"
        } else {
            "OFF"
        }
    };
}
macro_rules! cargo_link {
    ($feature:expr) => {
        println!("cargo:rustc-link-lib={}", $feature);
    };
}
fn main() {
    // Build StereoKit, and tell rustc to link it.
    let mut cmake_config = cmake::Config::new("StereoKit");
    cmake_config.define("SK_BUILD_SHARED_LIBS", "OFF");
    cmake_config.define("SK_BUILD_TESTS", "OFF");
    cmake_config.define("SK_LINUX_EGL", cargo_cmake_feat!("linux-egl"));
    cmake_config.define("SK_PHYSICS", cargo_cmake_feat!("physics"));
    let dst = cmake_config.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    cargo_link!("static=StereoKitC");

    cargo_link!("stdc++");
    cargo_link!("X11");
    cargo_link!("GLX");
    cargo_link!("GL");
    cargo_link!("GLEW");
    cargo_link!("EGL");
    cargo_link!("openxr_loader");
    cargo_link!("fontconfig");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/static-wrapper.h");

    // Generate bindings to StereoKitC.
    let macros = Arc::new(RwLock::new(HashSet::new()));
    let bindings = bindgen::Builder::default()
        .header("src/static-wrapper.h")
        .blocklist_type("FP_NAN")
        .blocklist_type("FP_INFINITE")
        .blocklist_type("FP_ZERO")
        .blocklist_type("FP_SUBNORMAL")
        .blocklist_type("FP_NORMAL")
        .parse_callbacks(Box::new(MacroCallback { macros }))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

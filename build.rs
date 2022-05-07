extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};

#[derive(Debug)]
struct MacroCallback {
	macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
	fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
		self.macros.write().unwrap().insert(name.into());

		match name {
			"FP_NAN" =>{return MacroParsingBehavior::Ignore},
			"FP_INFINITE" =>{return MacroParsingBehavior::Ignore},
			"FP_ZERO" =>{return MacroParsingBehavior::Ignore}
			"FP_SUBNORMAL" =>{return MacroParsingBehavior::Ignore},
			"FP_NORMAL" =>{return MacroParsingBehavior::Ignore}
			&_ => {}
		}

		MacroParsingBehavior::Default
	}
}
fn main() {
	// Tell cargo to tell rustc to link the system bzip2
	// shared library.
	println!("cargo:rustc-link-lib=StereoKitC");

	// Tell cargo to invalidate the built crate whenever the wrapper changes
	println!("cargo:rerun-if-changed=stereokit.h");
	let macros = Arc::new(RwLock::new(HashSet::new()));
	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		// The input header we would like to generate
		// bindings for.
		.header("stereokit.h")
		.blocklist_type("FP_NAN")
		.blocklist_type("FP_INFINITE")
		.blocklist_type("FP_ZERO")
		.blocklist_type("FP_SUBNORMAL")
		.blocklist_type("FP_NORMAL")
		.parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
		// Tell cargo to invalidate the built crate whenever any of the
		// included header files changed.
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings");
	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
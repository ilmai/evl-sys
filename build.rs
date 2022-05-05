use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=evl");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Yocto integration: if some SDK env was sourced, make sure
    // bindgen tells Clang about the right sysroot location and target
    // triple.
    let mut target_triple = String::new();
    if let Ok(sysroot) = env::var("OECORE_TARGET_SYSROOT") {
	let mut opt: String = "--sysroot=".to_owned();
	opt.push_str(&sysroot);
	let key = "BINDGEN_EXTRA_CLANG_ARGS";
	env::set_var(key, opt);
	let arch = env::var("OECORE_TARGET_ARCH").unwrap();
	// If building for ARM, turn on hard-float support if the GCC
	// tunables mention this for the target, otherwise we may have
	// the GNU stubs spuriously looking for missing soft-fp
	// headers. We do that by switching the target abi to
	// -gnueabihf.
	if arch == "arm" {
	    if let Ok(ccargs) = env::var("OECORE_TUNE_CCARGS") {
		if ccargs.contains("mfloat-abi=hard") {
		    target_triple = "--target=arm-poky-linux-gnueabihf".to_string()
		}
	    }
	}
    }

    let bindings = bindgen::Builder::default()
        .allowlist_function("evl_.*")
        .allowlist_type("evl_.*")
        .allowlist_var("evl_.*")
        .blocklist_function("evl_sigdebug_handler")
        .blocklist_type("siginfo_.*")
        .clang_arg("-Ivendor")
        .clang_arg("-Ivendor/evl")
        .clang_arg("-D_GNU_SOURCE")
        .clang_arg(target_triple)
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

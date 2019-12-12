fn main() {

    //
    // Settings
    // ========
    //

    // the architecture targetted
    let mut arch_target = "generic64";
    
    if cfg!(feature = "haswell") {
        arch_target = "Haswell";
    } else if cfg!(feature = "generic64lc") {
        arch_target = "generic64lc";
    } else if         cfg!(feature = "nehalem") {
            arch_target = "Nehalem";
        } else if         cfg!(feature = "sandybridge") {
            arch_target = "SandyBridge";
        } else if         cfg!(feature = "bulldozer") {
            arch_target = "Bulldozer";
        } else if         cfg!(feature = "haswell") {
            arch_target = "Haswell";
        } else if         cfg!(feature = "skylakex") {
            arch_target = "SkylakeX";
        } 

    // obtain version of XKCP from .xkcp_version
    let xkcp_version = std::fs::read_to_string(".xkcp_version").unwrap();

    // current dir
    let main_dir = std::env::current_dir().unwrap();
    // the OUT_DIR where we clone and build XKCP
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // shortcut to the XKCP dir in OUT_DIR
    let xkcp_dir = out_dir.clone().join("XKCP");

    //
    // Make libkeccak
    // ==============
    //

    // cd OUT_DIR
    assert!(std::env::set_current_dir(&out_dir).is_ok());

    // remove any XKCP that is already lying there
    std::fs::remove_dir_all(&xkcp_dir).ok();

    // git clone https://github.com/XKCP/XKCP.git
    // git checkout e8e6b1d45861ea6fc0957ab27a401f42aef0d033
    std::process::Command::new("git")
        .args(&["clone", "https://github.com/XKCP/XKCP.git"])
        .status()
        .expect("failed to git clone XKCP");

    std::process::Command::new("git")
        .args(&["checkout", &xkcp_version])
        .status()
        .expect("failed to git checkout a specific commit");

    // cd OUT_DIR/XKCP    
    assert!(std::env::set_current_dir(&xkcp_dir).is_ok());

    // make <arch>libkeccak.a
    let to_make = format!("{}/libkeccak.a", arch_target);
    std::process::Command::new("make")
        .args(&[to_make])
        .env("CFLAGS", "-fPIC") // needed by rust
        .status()
        .expect("failed to make!");

    //
    // Create bindings with bindgen
    // ============================
    //

    assert!(std::env::set_current_dir(&main_dir).is_ok());

    // bindgen magic using wrapper.h
    let include_path = format!("-I {}/XKCP/bin/{}/libkeccak.a.headers", out_dir.display(), arch_target);
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(include_path)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    //
    // Cargo flags
    //

    // tell cargo to re-build if our .xkcp_version changes
    println!("cargo:rerun-if-changed=.xkcp_version");
    println!("cargo:rerun-if-changed=wrapper.h");
    // tell cargo to use the library path OUT_DIR/XKCP/bin/<arch>
    println!("cargo:rustc-flags=-L {}/XKCP/bin/{}", out_dir.display(), arch_target);
}

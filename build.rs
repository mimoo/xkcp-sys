fn main() {

    //
    // Settings
    // ========
    //

    // the architecture targetted
    let arch_target = "generic64";

    // obtain version of XKCP from .xkcp_version
    let xkcp_version = std::fs::read_to_string(".xkcp_version").unwrap();

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
        .status()
        .expect("failed to make!");

    //
    // Cargo flags
    //

    // tell cargo to re-build if our .xkcp_version changes
    println!("cargo:rerun-if-changed=.xkcp_version");
    // tell cargo to use the library path OUT_DIR/XKCP/bin/<arch>
    println!("cargo:rustc-flags=-L {}/XKCP/bin/{}", out_dir.display(), arch_target);
}

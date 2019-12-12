fn main() {
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
    std::fs::remove_dir_all(&xkcp_dir);

    // git clone https://github.com/XKCP/XKCP.git
    // git checkout e8e6b1d45861ea6fc0957ab27a401f42aef0d033
    std::process::Command::new("git")
        .args(&["clone", "https://github.com/XKCP/XKCP.git"])
        .status()
        .expect("failed to git clone XKCP");

    std::process::Command::new("git")
        .args(&["checkout", "e8e6b1d45861ea6fc0957ab27a401f42aef0d033"])
        .status()
        .expect("failed to git checkout a specific commit");

    // cd OUT_DIR/XKCP    
    assert!(std::env::set_current_dir(&xkcp_dir).is_ok());

    // make generic64/libkeccak.a
    std::process::Command::new("make")
        .args(&["generic64/libkeccak.a"])
        .status()
        .expect("failed to make!");

    //
    // Cargo flags
    //

    // tell cargo to re-build if our wrapper.h changes
    println!("cargo:rerun-if-changed=wrapper.h");
    // tell cargo to use the library path OUT_DIR/XKCP/bin/generic64
    println!("cargo:rustc-flags=-L {}/XKCP/bin/generic64", out_dir.display());
}

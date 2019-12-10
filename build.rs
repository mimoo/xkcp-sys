fn main() {
    let xkcp_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("XKCP");

    std::process::Command::new("git")
        .args(&["submodule", "init"])
        .status()
        .expect("failed to init git submodule XKCP");

    std::process::Command::new("git")
        .args(&["submodule", "update"])
        .status()
        .expect("failed to update git submodule XKCP");

    assert!(std::env::set_current_dir(&xkcp_dir).is_ok());

    std::process::Command::new("make")
        .args(&["generic64/libkeccak.a"])
        .status()
        .expect("failed to make!");

    println!("cargo:rustc-flags=-L XKCP/bin/generic64");
}

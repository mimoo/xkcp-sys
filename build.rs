fn main() {
    //    let XKCP_dir = env!("CARGO_MANIFEST_DIR").join("XKCP");

    std::process::Command::new("make")
        .args(&["generic64/libkeccak.a"])
        .status()
        .expect("failed to make!");

    println!("cargo:rustc-flags=-L XKCP/bin/generic64");
}

These are rust bindings to the C reference implementations of the different keccak-based algorithms.

Currently the build script:

* clones the last version of the full [XKCP library](https://github.com/XKCP/XKCP)
* `make`s the static `libkeccak.a` library for `generic64`
* uses this as bindings

I'm not sure what the best way to offer support for different CPUs would be, perhaps manually through feature flags, or dynamically?

## TODO:

* dynamically compiles the best implementation that the host architecture supports
    - use [target_feature](https://github.com/RustCrypto/universal-hashes/blob/master/polyval/src/field.rs) flag
* use bindgen
* clone XKCP and compile in OUT_DIR instead of in /XKCP directly
* publish as a crate
**OBVIOUSLY DONT USE THIS AT THE MOMENT**

These are rust bindings to the C reference implementations of the different keccak-based algorithms.

Currently the build script:

* clones the last version of the full [XKCP library](https://github.com/XKCP/XKCP)
* `make`s the static `libkeccak.a` library for `generic64`
* uses this as bindings

I'm not sure what the best way to offer support for different CPUs would be, perhaps manually through feature flags, or dynamically?

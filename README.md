# firefly-sudo

A Rust crate for accessing internal API of [Firefly Zero](https://fireflyzero.com/). Used in [system apps](https://catalog.fireflyzero.com/@sys).

To use the crate, `sudo = true` must be set in `firefly.toml`. Unofficial sudo apps can be build from source using `ff build` but cannot be installed using any other method or added into the catalog. This is to prevent seemingly regular apps accessing data that they are not supposed to access. We still allow building from source because then the users can inspect the app source code before installing it.

If you miss a feature in the core Firefly Zero ecosystem and want to add it or fix it, please, reach out to us and we'll help you.

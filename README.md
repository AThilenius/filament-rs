# filament-rs

> Not a usable crate yet.

Rust bindings for [Google Filament](https://github.com/google/filament).

## Building

_Only Windows and OSX are currently supported._

### Filament deps (Required)

The Filament deps are absurdly large (2.77GB extracted for Windows) so they are
not committed to git (or even git lfs). You need to download and extract them
yourself:

- Download and extract the tarball for Windows or OSX from:
  https://github.com/google/filament/releases
- Copy the `lib` and `include` folders to `cpp` (`./cpp/lib/` and
  `./cpp/include/` respectively).
- _Optional: If you wish to re-compile materials from source, copy the `bin`
  folder to the root (`./bin`)._

### Running Examples

```sh
cargo run --example triangle
```

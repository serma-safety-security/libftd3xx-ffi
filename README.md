# libftd3xx-ffi

_These crates (`libftd3xx-ffi` & `libftd3xx`) are widely inspired by the
[`FTD2XX` Rust bindings]._

Rust FFI bindings to the [FTDI D3XX drivers].

This crate is **just** the C bindings.
There is a separate crate, [libftd3xx], which provides safe wrappers around
the unsafe C bindings.

## Usage
Simply add this crate as a dependency in your `Cargo.toml`.
The static library is distributed in this crate with permission from FTDI.
The default feature set will use dynamic linking.

```toml
[dependencies]
libftd3xx-ffi = "0.1"
```

### Bindgen
The default feature set will use pre-generated bindings.

The bindings can also be generated during compilation using the [bindgen]
feature flag.
```toml
[dependencies]
libftd3xx-ffi = { version = "0.1", features = ["bindgen"] }
```

Bindgen has additional dependencies that must be installed in order to
compile successfully, see the [bindgen requirements] page for more details.

### Static Linking
Static linking the FTD3XX library into this crate can be done by using
the static feature flag.
```toml
[dependencies]
libftd3xx-ffi = { version = "0.1", features = ["static"] }
```
Static linking may be preferred, however there may be license
incompatibilities (static linking with GPL code).
If in doubt, check the FTDI [driver license terms].

### Emulation (no vendor library)
With the `emulation` feature the build script emits no link directive at all, not even with
`static`: another crate of the final binary must define every D3XX function this crate declares,
with the same names and C signatures (for example a shim that implements them over TCP for an
emulator). Cargo unifies features, so enabling it anywhere in a build removes the vendor library
from the whole binary.
```toml
[dependencies]
libftd3xx-ffi = { version = "0.1", features = ["static"] }

[features]
d3xx-emulation = ["libftd3xx-ffi/emulation"]
```
The package declares `links = "ftd3xx"`, so a build cannot contain two copies of these bindings.

### Dynamic Linking

⚠️  Under Windows, you must place the .dll file next to your executables !

## Supported Targets

This crate is in its early stages of developpement, the list will eventually grow
but is very limited for now.

### Tested Targets

* None yet

### Untested Targets

These targets are provided, but they are untested.
Use at your own risk.

* `x86_64-unknown-linux-gnu` (dynamic + static)

## References

* [FTDI Programmers Guide]
* [FTDI D3XX Drivers]

## License
FTDI provides the D3XX driver as a compiled library and a header file.
These files can be found within the `vendor` directory.

The code within the `vendor` directory is licensed by FTDI.
Please see the [driver license terms] page for their license.

**Note:** This crate is not affiliated with FTDI.
You will need to contact the vendor for any support requests with the
underlying library because it is closed source.

[`FTD2XX` Rust bindings]: https://github.com/ftdi-rs/libftd2xx
[bindgen requirements]: https://rust-lang.github.io/rust-bindgen/requirements.html
[bindgen]: https://github.com/rust-lang/rust-bindgen
[FTDI D3XX Drivers]: https://www.ftdichip.com/drivers/d3xx-drivers
[FTDI Programmers Guides]: https://ftdichip.com/document/programming-guides/
[driver license terms]: https://ftdichip.com/driver-licence-terms/

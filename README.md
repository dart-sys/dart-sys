<p align="center">
  <img
    src="https://raw.githubusercontent.com/dart-sys/dart-sys-branding-assets/main/dart-sys%20header.png"
    alt="Dart-sys brand header"
  >
  <br/>
  <br/>
</p>

# Dart-sys

[![Stars](https://img.shields.io/github/stars/dart-sys/dart-sys)](https://github.com/dart-sys/dart-sys/stargazers)
[![Forks](https://img.shields.io/github/forks/dart-sys/dart-sys)](https://github.com/dart-sys/dart-sys/network/members)
[![Crates.io](https://img.shields.io/crates/v/dart-sys.svg)](https://crates.io/crates/dart-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Docs.rs](https://docs.rs/dart-sys/badge.svg)](https://docs.rs/dart-sys)
[![CI](https://github.com/dart-sys/dart-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/dart-sys/dart-sys/actions/workflows/ci.yml)

> _Rust bindings to the [Dart ffi api](https://dart.dev/guides/libraries/c-interop)_

## Prerequisites 🔧

You will need the following tools available on your system:

- [Dart](https://dart.dev/get-dart) version 2.12 or higher
- [Rust](https://www.rust-lang.org/tools/install) version 1.51 or higher
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Git](https://git-scm.com/downloads)

### Unix/Linux 🐧

No additional requirements :)

### MacOS 🍎

No additional requirements :)

### Windows 🪟

On Windows platforms, dynamic libraries are linked _against_ the executable, not _into_ the executable as is the case o
Unix platforms.

⚠️ Important ⚠️

This means that (on Windows) you will **_Need_** to have the Dart SDK installed and available on your system path to
be able to compile Dart-sys.

## Installing 📦

Run the following Cargo command in your project directory:

```bash
cargo add dart-sys
```

Or add the following line to your Cargo.toml:

```toml
dart-sys = "4.1.0"
```

## Usage 💻

### Examples 📚

An extremely straightforward example of using `dart-sys` would be like such:

```rust
use dart_sys::{Dart_Handle, Dart_NewIntegerFromI64};

#[no_mangle]
/// Adds two integers together.
pub extern "C" fn dart_sys_example_extension_sum(
    a: Dart_Handle,
    b: Dart_Handle,
) -> Dart_Handle {
    let a = unsafe { Dart_NewIntegerFromI64(a) };
    let b = unsafe { Dart_NewIntegerFromI64(b) };
    a + b
}

#[no_mangle]
/// Multiplies two integers together.
pub extern "C" fn dart_sys_example_extension_product(
    a: Dart_Handle,
    b: Dart_Handle,
) -> Dart_Handle {
    let a = unsafe { Dart_NewIntegerFromI64(a) };
    let b = unsafe { Dart_NewIntegerFromI64(b) };
    a * b
}
```

```dart
import 'dart:ffi';

// open and link to the native library
final DynamicLibrary nativeLib = DynamicLibrary.open('libdart_sys_example_extension.so');

// lookup the sum function in the native library
final int Function(int, int) sum = nativeLib
    .lookup<NativeFunction<Int32 Function(Int32, Int32)>>('dart_sys_example_extension_sum')
    .asFunction();

// lookup the product function in the native library
final int Function(int, int) product = nativeLib
    .lookup<NativeFunction<Int32 Function(Int32, Int32)>>('dart_sys_example_extension_product')
    .asFunction();

void main() {
    print(sum(1, 2)); // 3
    print(product(1, 2)); // 2
}
```

While this example is certainly possible, you are not likely to ever use Dart-sys for this purpose.
See the [examples](examples/) directory for more in-depth
examples of how to use Dart-sys. All examples are tested using GitHub Actions and documented verbosely.

## Built With 🛠️

- [Rust](https://www.rust-lang.org/) - A systems programming language that runs
 blazingly fast, prevents segfaults, and guarantees thread safety.
- [Dart](https://dart.dev/) - A client-optimized language for fast apps on any platform.
- [Dart Native Extensions](https://dart.dev/server/c-interop-native-extensions) - A mechanism
 for writing native code in C/C++ and calling it from Dart.
- [bindgen](https://crates.io/crates/bindgen) - A Rust library for generating
 bindings to C and C++ APIs.

## Contributing ✏️

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct,
and the process for submitting pull requests. If you have any questions, please
open an issue, or contact admin <gutenfries@gmail.com> directly.

## Versioning 🪧

We use [SemVer](http://semver.org/) for versioning. For the versions available,
see the [tags on this repository](https://github.com/dart-sys/dart-sys/tags).

## License 📜

Dart-sys is open-sourced and released under the terms and conditions of one or both of the following licenses:

- [MIT License](LICENSE-MIT.md)
- [Apache License (Version 2.0)](LICENSE-APACHE-2.0.md)

## Acknowledgments 🙏

- [README starter](https://gist.github.com/PurpleBooth/109311bb0361f32d87a2) by [@PurpleBooth](https://gist.github.com/PurpleBooth)

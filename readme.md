# `dart-sys`
#### *Native bindings to the dart native extensions sdk using [`bindgen`](https://github.com/rust-lang/rust-bindgen).*

This crate exposes an api for [`dart_api.h`](https://github.com/dart-lang/sdk/blob/master/runtime/include/dart_api.h),
 which exposes the basic [dart](https://dart.dev/)
 native [extensions api](https://dart.dev/server/c-interop-native-extensions). 
 This crate uses bindgen to generate the bindings to the header.
 
##### Requirements
- Provide a path to the dart sdk using a `dart_sdk` environment variable.
  - If this variable is not available, will look for either a chocolatey install
  path, or an entry in the `PATH` variable which contains `dart-sdk` in it.
  This will fall back to the `flutter` sdk should it not find a dart sdk, but this
  is not recommended, as it is more difficult to compile using the flutter sdk
  and it appears it ships a non-standard dart sdk. 
- Have `clang` installed and on your path. 

##### Usage
Include the following in your `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]
[dependencies]
dart-sys = "0.1.0"
```
And follow the guide on the [native extensions api page](https://dart.dev/server/c-interop-native-extensions).

##### Examples
Please visit the examples directory for more information. If there should appear
more idiomatic bindings, I will try to keep this updated to link to it. 

### Note
A few things are not mentioned on the [native extensions api](https://dart.dev/server/c-interop-native-extensions)
page:

- You should compile using an x64 compiler
- You should place the compiled library in the same directory as the root of your dart
package (I.E. outside of your `lib` directory)

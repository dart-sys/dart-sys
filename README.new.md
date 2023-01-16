#

## Dart-sdk

Dart-sys will provide a dart-sdk for your project. First however, it attempts to use the same sdk you are using in your dev envoronment.

Dart-sys checks for an SDK in the following ways, in respective order:

1. Checks for installed dart-sdk

```rust
// build.rs
fn find_local_dart_sdk() {...}
```

2. Checks for installed flutter-sdk

```rust
// build.rs
fn find_local_flutter_sdk() {...}
```

4. Uses the packaged dart-sdk as a complete fallback

```rust
// build.rs
fn use_packaged_dart_sdk() {...}
```

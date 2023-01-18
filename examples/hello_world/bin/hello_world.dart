import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;

import 'package:path/path.dart' as path;

/// Return type of the _FFI_ function
typedef HelloWorldFunction = ffi.Void Function();

/// Return type of the _Dart_ function
typedef HelloWorld = void Function();

void main() {
  // Find the path to the dynamic library
  final cwd = Directory.current.path;

  // Check in cwd/../target/{debug,release}/ for the lib
  final targetDir = path.join(cwd, '..', 'target');
  final debugDir = path.join(targetDir, 'debug');
  final releaseDir = path.join(targetDir, 'release');

  /// Path to the library to link to
  late String libPath;

  // if a release build exists, prefer to use it
  switch (Platform.operatingSystem) {
    case 'macos':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'libhello_world.dylib');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'libhello_world.dylib');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'linux':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'libhello_world.so');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'libhello_world.so');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'windows':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'hello_world.dll');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'hello_world.dll');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }

    default:
      throw Exception('Unsupported platform');
  }

  // Load the library in memory
  final lib = ffi.DynamicLibrary.open(libPath);

  // look for the Rust function named `hello_world` in the library
  final HelloWorld helloWorld = lib
      .lookup<ffi.NativeFunction<HelloWorldFunction>>('hello_world')
      .asFunction();

  helloWorld();
}

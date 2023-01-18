import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;

import 'package:path/path.dart' as path;

/// Return type of the _FFI_ function
typedef RandomNumberFunction = ffi.Int64 Function();

/// Return type of the _Dart_ function
typedef RandomNumber = int Function();

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
        libPath = path.join(releaseDir, 'librandom_number.dylib');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'librandom_number.dylib');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'linux':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'librandom_number.so');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'librandom_number.so');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'windows':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'random_number.dll');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'random_number.dll');
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

  // look for the Rust function named `random_number` in the library
  final RandomNumber randomNumber = lib
      .lookup<ffi.NativeFunction<RandomNumberFunction>>('random_number')
      .asFunction();

  print(randomNumber());
}

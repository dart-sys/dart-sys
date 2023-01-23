import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;

import 'package:ffi/ffi.dart' as native;
import 'package:path/path.dart' as path;

import 'primitives.def.dart';

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
        libPath = path.join(releaseDir, 'libprimitives.lib');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'libprimitives.lib');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'linux':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'libprimitives.so');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'libprimitives.so');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'windows':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'primitives.dll');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'primitives.dll');
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

  // look for the Rust function named `random_number` in the primitives library
  final RandomNumber randomNumber = lib
      .lookup<ffi.NativeFunction<RandomNumberFunction>>('random_number')
      .asFunction();

  print('Random u8 number: ${randomNumber()}');

  // calls int add(int a, int b);
  final addPointer = lib.lookup<ffi.NativeFunction<AddFunc>>('add');
  final add = addPointer.asFunction<Add>();
  print('3 + 5 = ${add(3, 5)}');

  // calls int subtract(int *a, int b);
  // Create a pointer
  final p = native.calloc<ffi.Int32>();

  // Place a value into the address
  p.value = 3;

  final subtractPointer =
      lib.lookup<ffi.NativeFunction<SubtractFunc>>('subtract');
  final subtract = subtractPointer.asFunction<Subtract>();
  print('3 - 5 = ${subtract(p, 5)}');

  // Free up allocated memory.
  native.calloc.free(p);

  // calls int *multiply(int a, int b);
  final multiplyPointer =
      lib.lookup<ffi.NativeFunction<MultiplyFunc>>('multiply');
  final multiply = multiplyPointer.asFunction<Multiply>();
  final resultPointer = multiply(3, 5);
  // Fetch the result at the address pointed to
  final int result = resultPointer.value;
  print('3 * 5 = $result');

  // Free up allocated memory. This time in C, because it was allocated in C.
  final freePointerPointer =
      lib.lookup<ffi.NativeFunction<FreePointerFunc>>('free_pointer');
  final freePointer = freePointerPointer.asFunction<FreePointer>();
  freePointer(resultPointer);
}

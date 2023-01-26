import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;

import 'package:ffi/ffi.dart';
import 'package:path/path.dart' as path;

import 'structs.def.dart';

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
        libPath = path.join(releaseDir, 'libstructs.lib');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'libstructs.lib');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'linux':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'libstructs.so');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'libstructs.so');
        break;
      } else {
        throw Exception(
            'Could not find dynamic library. Ensure that the rust library is built.');
      }
    case 'windows':
      if (Directory(releaseDir).existsSync()) {
        libPath = path.join(releaseDir, 'structs.dll');
        break;
      } else if (Directory(debugDir).existsSync()) {
        libPath = path.join(debugDir, 'structs.dll');
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

  // create two coordinates
  final create_coordinate =
      lib.lookupFunction<CreateCoordinateNative, CreateCoordinate>(
          'create_coordinate');

  final coordinate_1 = create_coordinate(1.0, 2.0);
  final coordinate_2 = create_coordinate(3.0, 4.0);

  // create a place with the first coordinate called "Home"

  final create_place =
      lib.lookupFunction<CreatePlaceNative, CreatePlace>('create_place');

  final homeUtf8 = 'Home'.toNativeUtf8();

  final home =
      create_place(homeUtf8, coordinate_1.latitude, coordinate_1.longitude);

  // calculate the distance between home and the second coordinate

  final distance = lib.lookupFunction<DistanceNative, Distance>('distance');

  final distance_between = distance(coordinate_2, home.coordinate);

  print('Distance between place "${home.name.toDartString()}" and '
      '(${coordinate_2.latitude}, ${coordinate_2.longitude}) is '
      '${distance_between}');
}

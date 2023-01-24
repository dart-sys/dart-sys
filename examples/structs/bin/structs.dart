import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;

import 'package:ffi/ffi.dart' as native;
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

  final reverse_str = lib.lookupFunction<ReverseNative, Reverse>('reverse_str');
  final backwards = 'backwards';
  final backwardsUtf8 = backwards.toNativeUtf8();
  final reversedMessageUtf8 = reverse_str(backwardsUtf8, backwards.length);
  final reversedMessage = reversedMessageUtf8.toDartString();
  calloc.free(backwardsUtf8);
  print('$backwards reversed is $reversedMessage');

  final freeStr = lib.lookupFunction<FreeStrNative, FreeStr>('free_str');
  freeStr(reversedMessageUtf8);

  final createCoordinate =
      lib.lookupFunction<CreateCoordinateNative, CreateCoordinate>(
          'create_coordinate');
  final coordinate = createCoordinate(3.5, 4.6);
  print(
      'Coordinate is lat ${coordinate.latitude}, long ${coordinate.longitude}');

  final myHomeUtf8 = 'My Home'.toNativeUtf8();
  final createPlace =
      lib.lookupFunction<CreatePlaceNative, CreatePlace>('create_place');
  final place = createPlace(myHomeUtf8, 42.0, 24.0);
  final name = place.name.toDartString();
  calloc.free(myHomeUtf8);
  final coord = place.coordinate;
  print(
      'The name of my place is $name at ${coord.latitude}, ${coord.longitude}');
  final distance = lib.lookupFunction<DistanceNative, Distance>('distance');
  final dist = distance(createCoordinate(2.0, 2.0), createCoordinate(5.0, 6.0));
  print("distance between (2,2) and (5,6) = $dist");
}

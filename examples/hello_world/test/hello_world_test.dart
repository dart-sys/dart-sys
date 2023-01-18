@Timeout(Duration(minutes: 2))

import 'dart:io';

import 'package:test/test.dart';
import 'package:path/path.dart' as path;

void main() async {
  group('hello_world', () {
    test('Build rust library & test dart program', () async {
      // Build the library
      var buildProcess = await Process.run('cargo', ['build']);
      expect(buildProcess.exitCode, equals(0));

      final binDir = path.join(Directory.current.path, '..', 'target', 'debug');

      // Verify that the library exists
      switch (Platform.operatingSystem) {
        case 'macos':
          expect(
            File('$binDir/libhello_world.dylib').existsSync(),
            isTrue,
          );
          break;
        case 'linux':
          expect(
            File('$binDir/libhello_world.so').existsSync(),
            isTrue,
          );
          break;
        case 'windows':
          expect(
            File('$binDir/hello_world.dll').existsSync(),
            isTrue,
          );
          break;
        default:
          throw Exception('Unsupported platform');
      }

      // Run the dart program
      var dartProcess = await Process.run('dart', ['bin/hello_world.dart']);

      // Verify program did not throw error
      expect(dartProcess.stderr, isEmpty);
      // Verify that output is "Hello, World!"
      expect(dartProcess.stdout, equals('Hello, World!\n'));
      // Verify that the program exited successfully
      expect(dartProcess.exitCode, equals(0));
    });
  });
}

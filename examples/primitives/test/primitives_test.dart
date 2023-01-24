@Timeout(Duration(hours: 1))

import 'dart:io';

import 'package:test/test.dart';
import 'package:path/path.dart' as path;

void main() async {
  group('primitives', () {
    test('Build rust library & test dart program', () async {
      // Build the library
      var buildProcess = await Process.run('cargo', ['build']);
      expect(buildProcess.exitCode, equals(0));

      final binDir = path.join(Directory.current.path, '..', 'target', 'debug');

      // Verify that the library exists
      switch (Platform.operatingSystem) {
        case 'macos':
          expect(
            File('$binDir/libprimitives.dylib').existsSync(),
            isTrue,
          );
          break;
        case 'linux':
          expect(
            File('$binDir/libprimitives.so').existsSync(),
            isTrue,
          );
          break;
        case 'windows':
          expect(
            File('$binDir/primitives.dll').existsSync(),
            isTrue,
          );
          break;
        default:
          throw Exception('Unsupported platform');
      }

      // Run the dart program
      var dartProcess = await Process.run('dart', ['bin/primitives.dart']);

      // Verify program did not throw error
      expect(dartProcess.stderr, isEmpty);

      // expected output:
      //
      // {0-255}
      // 3 + 5 = 8
      // 3 - 5 = -2
      // 3 * 5 = 15
      expect(dartProcess.stdout, contains('3 + 5 = 8'));
      expect(dartProcess.stdout, contains('3 - 5 = -2'));
      expect(dartProcess.stdout, contains('3 * 5 = 15'));
      // Verify that the program exited successfully
      expect(dartProcess.exitCode, equals(0));
    });
  });
}

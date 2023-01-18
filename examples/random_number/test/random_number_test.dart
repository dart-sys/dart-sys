@Timeout(Duration(minutes: 2))

import 'dart:io';

import 'package:test/test.dart';
import 'package:path/path.dart' as path;

void main() async {
  group('random_number', () {
    test('Build rust library & test dart program', () async {
      // Build the library
      var buildProcess = await Process.run('cargo', ['build']);
      expect(buildProcess.exitCode, equals(0));

      final binDir = path.join(Directory.current.path, '..', 'target', 'debug');

      // Verify that the library exists
      switch (Platform.operatingSystem) {
        case 'macos':
          expect(
            File('$binDir/librandom_number.dylib').existsSync(),
            isTrue,
          );
          break;
        case 'linux':
          expect(
            File('$binDir/librandom_number.so').existsSync(),
            isTrue,
          );
          break;
        case 'windows':
          expect(
            File('$binDir/random_number.dll').existsSync(),
            isTrue,
          );
          break;
        default:
          throw Exception('Unsupported platform');
      }

      // Run the dart program
      var dartProcess = await Process.run('dart', ['bin/random_number.dart']);

      // Verify program did not throw error
      expect(dartProcess.stderr, isEmpty);
      // Verify that output is a positive integer between 0 and 255
      expect(int.parse(dartProcess.stdout), inInclusiveRange(0, 255));
      // Verify that the program exited successfully
      expect(dartProcess.exitCode, equals(0));
    });
  });
}

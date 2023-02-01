@Timeout(Duration(hours: 1))

import 'dart:io';

import 'package:test/test.dart';
import 'package:path/path.dart' as path;

void main() async {
  group('structs', () {
    test('Build rust library & test dart program', () async {
      // Build the library
      await Process.run('cargo', ['clean']);
      var buildProcess = await Process.run('cargo', ['build']);
      expect(buildProcess.exitCode, equals(0));

      final binDir = path.join(Directory.current.path, '..', 'target', 'debug');

      // Verify that the library exists
      switch (Platform.operatingSystem) {
        case 'macos':
          expect(
            File('$binDir/libstructs.dylib').existsSync(),
            isTrue,
          );
          break;
        case 'linux':
          expect(
            File('$binDir/libstructs.so').existsSync(),
            isTrue,
          );
          break;
        case 'windows':
          expect(
            File('$binDir/structs.dll').existsSync(),
            isTrue,
          );
          break;
        default:
          throw Exception('Unsupported platform');
      }

      // Run the dart program
      var dartProcess = await Process.run('dart', ['bin/structs.dart']);

      // Verify program did not throw error
      expect(dartProcess.stderr, isEmpty);

      // expected output:
      //
      // Distance between place "Home" and (3.0, 4.0) is 2.8284271247461903
      expect(dartProcess.stdout, contains('Home'));
      expect(dartProcess.stdout, contains('3.0'));
      expect(dartProcess.stdout, contains('4.0'));
      expect(dartProcess.stdout, contains('2.8284271247461903'));

      // Verify that the program exited successfully
      expect(dartProcess.exitCode, equals(0));
    });
  });
}

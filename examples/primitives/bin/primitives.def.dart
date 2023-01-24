import 'dart:ffi' as ffi;

typedef RandomNumberFunction = ffi.Int64 Function();
typedef RandomNumber = int Function();

typedef AddFunc = ffi.Int32 Function(ffi.Int32 a, ffi.Int32 b);
typedef Add = int Function(int a, int b);

typedef SubtractFunc = ffi.Int32 Function(
    ffi.Pointer<ffi.Int32> a, ffi.Int32 b);
typedef Subtract = int Function(ffi.Pointer<ffi.Int32> a, int b);

typedef MultiplyFunc = ffi.Pointer<ffi.Int32> Function(
    ffi.Int32 a, ffi.Int32 b);
typedef Multiply = ffi.Pointer<ffi.Int32> Function(int a, int b);

typedef FreePointerFunc = ffi.Void Function(ffi.Pointer<ffi.Int32> a);
typedef FreePointer = void Function(ffi.Pointer<ffi.Int32> a);

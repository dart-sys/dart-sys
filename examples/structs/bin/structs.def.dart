import 'dart:ffi';
import 'package:ffi/ffi.dart';

class Coordinate extends Struct {
  @Double()
  external double latitude;

  @Double()
  external double longitude;
}

class Place extends Struct {
  external Pointer<Utf8> name;

  external Coordinate coordinate;
}

typedef CreateCoordinateNative = Coordinate Function(
    Double latitude, Double longitude);
typedef CreateCoordinate = Coordinate Function(
    double latitude, double longitude);

typedef CreatePlaceNative = Place Function(
    Pointer<Utf8> name, Double latitude, Double longitude);
typedef CreatePlace = Place Function(
    Pointer<Utf8> name, double latitude, double longitude);

typedef DistanceNative = Double Function(Coordinate p1, Coordinate p2);
typedef Distance = double Function(Coordinate p1, Coordinate p2);

import 'dart:convert';
import 'dart:async';

import 'package:crypto/crypto.dart';
import 'package:camera/camera.dart';
import 'package:flutter/material.dart';

/// @brief This function hash a string using SHA256 and return the hashed value.
/// @param word -> The string that will be hashed.
/// @return The hashed value of the string.
String hash_string(String word) {
  return sha256.convert(utf8.encode(word)).toString();
}

/// @brief Given a json which is a list of users (sent in a response body by the server), this function retrive all pseudo in this list.
/// @param json -> The json string that is the list.
/// @return A list of pseudonyms.
List<String> listUserJsonRetrievePseudo(String json) {
  List<dynamic> decodedJson = jsonDecode(json);
  List<String> response = [];
  for (var item in decodedJson) {
    if (item is Map<String, dynamic> && item.containsKey('pseudo')) {
      response.add(item['pseudo'] as String);
    }
  }
  return response;
}

/// @brief This function retrieve the first or specified camera of the user device
/// @param [optional] camNumber -> The camera number in the list of camera available on the device.
/// @return The camera as a Future<CameraDescription>.
Future<CameraDescription?> findCamera({int camNumber = 0}) async {
  WidgetsFlutterBinding.ensureInitialized();
  try {
    final cameras = await availableCameras();
    try {
      return cameras[camNumber];
    } on Exception {
      return cameras.first;
    }
  } catch (e) {
    return null;
  }
}

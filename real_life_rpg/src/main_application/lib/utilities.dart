import 'dart:convert';

import 'package:crypto/crypto.dart';

/// @brief This function hash a string using SHA256 and return the hashed value.
/// @param word -> The string that will be hashed.
/// @return The hashed value of the string.
String hash_string(String word) {
  return sha256.convert(utf8.encode(word)).toString();
}

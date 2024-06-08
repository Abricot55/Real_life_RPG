import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'main.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ProfilePage extends StatefulWidget {
  @override
  _ProfilePageState createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage>{
final storage = const FlutterSecureStorage();
var savedUserID = "";
var labelUserIDController = Text("userID");
/**
 * @brief This function build all the widgets the user will see on the screen when the profile page is loaded. This function is automatically called.
 * @param context -> The context in which the home page is created.
 * @return The widget which is all the stuff on screen.
 */
@override
Widget build(BuildContext context) {
  readUserID();
  return Scaffold(
      body: Center(
        child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
          labelUserIDController,
          Text("COUCOU"),
          ElevatedButton(
              onPressed: () {
                navigateToNextScreen(context, 1);
              },
              child: Text("Disconnect"))
        ]),
      ));
}

Future<void> readUserID() async {
  savedUserID = (await storage.read(key: "_userID"))!;
  setState(() {
    labelUserIDController = Text(savedUserID);
  });
}
}

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'main.dart';
//import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class Settingspage extends StatefulWidget {
  @override
  _SettingspageState createState() => _SettingspageState();
}

class _SettingspageState extends State<Settingspage> {
  //final storage = const FlutterSecureStorage();
  var savedUserID = "";

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
          child: Column(children: [
            Text("SETTINGS"),
            ElevatedButton(
                onPressed: (){
                  navigateToNextScreen(context, 2);
                  },
                child: Text("Back")),
            ElevatedButton(
                onPressed: (){
                  navigateToNextScreen(context, 1);
                },
                child: Text("Disconnect"))
          ]),
        ));
  }

  Future<void> readUserID() async {
    //savedUserID = (await storage.read(key: "_userID"))!;
  }
}

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'main.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ProfilePage extends StatefulWidget {
  @override
  _ProfilePageState createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  final storage = const FlutterSecureStorage();
  var savedUserID = "";
  var labelUserIDController = Text("", style: TextStyle(fontSize: 20.0));

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
        Container(
          padding: EdgeInsets.only(left: 5.0, right: 5.0),
            color: Colors.grey,
            child: Column(
              children: [
                SizedBox(height: 30),
                Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      labelUserIDController,
                      ElevatedButton(
                          onPressed: () {
                            navigateToNextScreen(context, 1);
                          },
                          child: Text("Disconnect", style: TextStyle(fontSize: 15.0),)),
                    ])
              ],
            )),
        Text("COUCOU"),
      ]),
    ));
  }

  Future<void> readUserID() async {
    savedUserID = (await storage.read(key: "_userID"))!;
    setState(() {
      labelUserIDController =
          Text(savedUserID, style: TextStyle(fontSize: 20.0));
    });
  }
}

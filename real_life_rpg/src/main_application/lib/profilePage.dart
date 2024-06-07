import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'main.dart';

class ProfilePage extends StatelessWidget {
  /**
   * @brief This function build all the widgets the user will see on the screen when the profile page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Center(
      child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
        Text("COUCOU"),
        ElevatedButton(
            onPressed: () {
              navigateToNextScreen(context, 1);
            },
            child: Text("Disconnect"))
      ]),
    ));
  }
}

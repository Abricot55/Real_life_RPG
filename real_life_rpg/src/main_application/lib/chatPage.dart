import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'main.dart';
//import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ChatPage extends StatefulWidget {
  @override
  _ChatPageState createState() => _ChatPageState();
}

class _ChatPageState extends State<ChatPage> {
  //final storage = const FlutterSecureStorage();
  var savedUserID = "testUser";

  /**
   * @brief This function build all the widgets the user will see on the screen when the profile page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    readUserID();
    return Scaffold(
        body: Column(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
          Column(children: [
            Container(
                padding: EdgeInsets.only(left: 5.0, right: 5.0),
                //color: Theme.of(context).primaryColor,
                child: Column(children: [
                  SizedBox(height: 30),
                  Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        ElevatedButton(
                            onPressed: () {
                              navigateToNextScreen(context, 2);
                            },
                            child: Text("back")),
                        Text(savedUserID, style: TextStyle(fontSize: 25),),
                        SizedBox(width: 65,)
                      ]),
                  Row(
                    children: [
                      Container(
                        width: MediaQuery.of(context).size.width - 120,
                        child: TextField(
                          onChanged: (text) {},
                          decoration: new InputDecoration(
                              hintText: "Search a user",
                              contentPadding:
                                  EdgeInsets.only(left: 10, right: 10),
                              border: OutlineInputBorder(
                                  gapPadding: 0,
                                  borderRadius:
                                      BorderRadius.all(Radius.circular(25.0)))),
                        ),
                      ),
                      SizedBox(
                        width: 5,
                      ),
                      ElevatedButton(
                          onPressed: () {
                            //rien
                          },
                          child: Text("Search"))
                    ],
                  ),
                  Divider()
                ])),
            Container(
                padding: EdgeInsets.only(left: 10.0, right: 10.0),
                child: Column(
                    //children: _itemsRecherche,
                    ))
          ])
        ]));
  }

  Future<void> readUserID() async {
    //savedUserID = (await storage.read(key: "_userID"))!;
  }
}

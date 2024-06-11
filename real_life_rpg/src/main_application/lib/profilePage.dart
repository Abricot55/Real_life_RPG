import 'dart:convert';
import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:percent_indicator/circular_percent_indicator.dart';
import 'package:percent_indicator/linear_percent_indicator.dart';
import 'main.dart';
//import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ProfilePage extends StatefulWidget {
  @override
  _ProfilePageState createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  //final storage = const FlutterSecureStorage();

  //varaibles
  var savedUserID = "";
  var nbFriends = 0;
  var activeSkills = Map<String, double>();
  var profileDescription = "";

  //containers
  var labelUserIDController = Text("", style: TextStyle(fontSize: 25.0));
  var labelNbFriends = Text("Friends: ", style: TextStyle(fontSize: 20.0));
  var columnSkills = Column();
  var containerDescription = Container();

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
                color: Theme
                    .of(context)
                    .primaryColor,
                child: Column(
                  children: [
                    SizedBox(height: 30),
                    Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          labelUserIDController,
                          ElevatedButton(
                              onPressed: () {
                                navigateToNextScreen(context, 3);
                              },
                              child: Text(
                                "Settings",
                                style: TextStyle(fontSize: 15.0),
                              )),
                        ])
                  ],
                )),
            Container(
              padding: EdgeInsets.all(5.0),
              child: Column(
                children: [
                  containerDescription,
                  Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                      labelNbFriends,
                      GestureDetector(
                          onTap: () {},
                          child: Text("See all...",
                              style: TextStyle(color: Theme
                                  .of(context)
                                  .primaryColor)))]),
                  Divider(),
                  Row(children: [
                    Text(
                      "My skills",
                      style: TextStyle(fontSize: 20.0)
                    )
                  ]),
                  columnSkills
                ],
              ),
            ),
          ]),
        ));
  }

  Future<void> readUserID() async {
    //trouver le user id
    savedUserID = "testUser"; //(await storage.read(key: "_userID"))!;
    //faire les requêtes
    //userTest
    if (savedUserID == "testUser") {
      nbFriends = 999;
      activeSkills = {
        "Cooking": 34.3,
        "Skateboard": 12.1,
        "Chapeau melon": 99.90
      };
      profileDescription =
      "This is a test account made to preview what an actual account could display on a phone when the connection with the server is successful!";
    }
    setState(() {
      labelUserIDController = Text(
        savedUserID,
        style: TextStyle(fontSize: 25.0, color: Colors.white),
      );
      labelNbFriends =
          Text("Friends: ${nbFriends}", style: TextStyle(fontSize: 20.0));
      List<Widget> skills = [];
      for (MapEntry<String, double> item in activeSkills.entries) {
        skills.add(Row(
          children: [
            Text(
              "${item.key}: ${item.value.toInt()}",
              style: TextStyle(fontSize: 18.0),
            )
          ],
        ));
        skills.add(
            Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
              Text("${item.value.toInt()}"),
              LinearPercentIndicator(
                width: MediaQuery
                    .of(context)
                    .size
                    .width - 60,
                progressColor: Theme
                    .of(context)
                    .primaryColor,
                percent: item.value - item.value.toInt(),
                barRadius: Radius.circular(10),
                lineHeight: 18,
                center: Text(
                  "${((item.value - item.value.toInt()) * 100).toInt()}%",
                  style: TextStyle(color: Colors.white),
                ),
              ),
              Text("${item.value.toInt() + 1}")
            ]));
      }
      columnSkills = Column(children: skills);
      if (profileDescription != "") {
        containerDescription = Container(
          child: Column(
            children: [
              Text(
                profileDescription,
                style: TextStyle(fontSize: 18),
                softWrap: true,
              ),
              Divider()
            ],
          ),
        );
      }
    });
  }
}

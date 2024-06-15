import 'dart:convert';
import 'dart:ffi';
import 'dart:math';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:percent_indicator/linear_percent_indicator.dart';
import 'User.dart';
import 'main.dart';
import 'utilities.dart';
//import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ProfilePage extends StatefulWidget {
  @override
  _ProfilePageState createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  //final storage = const FlutterSecureStorage();

  //varaibles
  var savedUserID = "";
  late User me;
  var _selectedIndex = 0;

  //containers
  var containerGeneral = null;

  //containers homePage
  var containerHomePage = Column();
  var containerRecherche = Column();
  var _searchMode = false;
  var searchController;
  List<Widget> _itemsRecherche = [];

  //containers profilepage
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
    if (containerGeneral == null) {
      readUserID();
      containerGeneral = getMainScreen(_selectedIndex);
    }
    return Scaffold(
        bottomNavigationBar: BottomNavigationBar(
            onTap: (int index) {
              var _me = me;
              setState(() {
                me = _me;
                _searchMode = false;
                _selectedIndex = index;
                containerGeneral = getMainScreen(index);
              });
            },
            selectedFontSize: 20,
            currentIndex: _selectedIndex,
            selectedItemColor: Theme.of(context).primaryColor,
            items: [
              BottomNavigationBarItem(icon: Container(), label: "Home"),
              BottomNavigationBarItem(icon: Container(), label: "My profile"),
              BottomNavigationBarItem(icon: Container(), label: "Picture")
            ]),
        body: containerGeneral);
  }

  Center getMainScreen(int index) {
    if (index == 0) {
      return Center(child: getHomePage(_searchMode));
    } else if (index == 1) {
      return getUserPage(me, true);
    } else if (index == 2) {
      return Center(
          child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [Text("Tu veux ma photo banane?")],
      ));
    } else {
      return Center(
        child: Text("QU'EST-CE QUE TU FAIS LÀ MAN??"),
      );
    }
  }

  Column getHomePage(bool _sM) {
    if (_sM == false) {
      return Column(
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
                          Text(
                            "RealLifeRPG",
                            style: TextStyle(
                                fontWeight: FontWeight.bold,
                                fontSize: 30,
                                color: Theme.of(context).primaryColor),
                          ),
                          ElevatedButton(
                              onPressed: () {
                                var _me = me;
                                setState(() {
                                  me = _me;
                                  _searchMode = true;
                                  _itemsRecherche = getListeItems("");
                                  containerGeneral = getHomePage(_searchMode);
                                });
                              },
                              child: Text("Search"))
                        ]),
                    Divider()
                  ]))
            ])
          ]);
    } else {
      return Column(
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
                                var _me = me;
                                setState(() {
                                  me = _me;
                                  _searchMode = false;
                                  containerGeneral = getHomePage(_searchMode);
                                });
                              },
                              child: Text("back")),
                          Container(
                            width: MediaQuery.of(context).size.width - 200,
                            child: TextField(
                              onChanged: (text) {
                                var _me = me;
                                setState(() {
                                  me = _me;
                                  _itemsRecherche = getListeItems(text);
                                  containerGeneral = getHomePage(_searchMode);
                                });
                              },
                              decoration: new InputDecoration(
                                  hintText: "Search a user or a skill",
                                  contentPadding:
                                      EdgeInsets.only(left: 10, right: 10),
                                  border: OutlineInputBorder(
                                      gapPadding: 0,
                                      borderRadius: BorderRadius.all(
                                          Radius.circular(25.0)))),
                            ),
                          ),
                          ElevatedButton(
                              onPressed: () {
                                //rien
                              },
                              child: Text("Search"))
                        ]),
                    Divider()
                  ])),
              Container(
                  padding: EdgeInsets.only(left: 10.0, right: 10.0),
                  child: Column(
                    children: _itemsRecherche,
                  ))
            ])
          ]);
    }
  }

  Center getUserPage(User aUser, bool myProfile) {
    setUserContainer(aUser, myProfile);
    return Center(
      child: Column(children: [
        Container(
            padding: EdgeInsets.only(left: 5.0, right: 5.0),
            color: Theme.of(context).primaryColor,
            child: Column(
              children: [
                SizedBox(height: 30),
                getTopUserPageController(aUser, myProfile)
              ],
            )),
        Container(
          padding: EdgeInsets.all(5.0),
          child: Column(
            children: [
              containerDescription,
              Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
                labelNbFriends,
                GestureDetector(
                    onTap: () {
                      var _me = me;
                      setState(() {
                        me = _me;
                        containerGeneral =
                            getFriendsPage(aUser, aUser.getId() == me.getId());
                      });
                    },
                    child: Text("See all...",
                        style: TextStyle(
                            color: Theme.of(context).primaryColor,
                            decoration: TextDecoration.underline)))
              ]),
              Divider(),
              Row(children: [
                Text(getStringSkills(aUser, myProfile),
                    style: TextStyle(fontSize: 20.0))
              ]),
              columnSkills
            ],
          ),
        )
      ]),
    );
  }

  Row getTopUserPageController(User aUser, bool myProfile) {
    if (myProfile) {
      return Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
        labelUserIDController,
        ElevatedButton(
            onPressed: () {
              navigateToNextScreen(context, 3);
            },
            child: Text(
              "Settings",
              style: TextStyle(fontSize: 15.0),
            )),
      ]);
    } else {
      return Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
        ElevatedButton(
            onPressed: () {
              var _me = me;
              setState(() {
                me = _me;
                containerGeneral = getFriendsPage(me, true);
              });
            },
            child: Text(
              "Back",
              style: TextStyle(fontSize: 15.0),
            )),
        labelUserIDController,
        SizedBox(
          width: 70,
        )
      ]);
    }
  }

  String getStringSkills(User user, bool myProfile) {
    if (myProfile) {
      return "My skills";
    } else {
      return "${user.getId()}'s skills";
    }
  }

  List<Widget> getListeItems(String text) {
    sendRequest("get", path: "users/relevant", urlMap: {"pseudo": text})
        .then((liste) {
      List<Widget> users = [];
      List<String> search = listUserJsonRetrievePseudo(liste.body);
      print(search);
      for (var i in search) {
        users.add(Text(i));
      }
      return users;
    });
    return [];
  }

  Center getFriendsPage(User aUser, bool myProfile) {
    return Center(
        child: Column(children: [
      Container(
          padding: EdgeInsets.only(left: 5.0, right: 5.0),
          color: Theme.of(context).primaryColor,
          child: Column(
            children: [
              SizedBox(height: 30),
              Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
                ElevatedButton(
                    onPressed: () {
                      var _me = me;
                      setState(() {
                        me = _me;
                        containerGeneral = getMainScreen(1);
                      });
                    },
                    child: Text(
                      "Back",
                      style: TextStyle(fontSize: 15.0),
                    )),
                Text(
                  getFriendTitle(aUser, myProfile),
                  style: TextStyle(color: Colors.white, fontSize: 25),
                ),
                SizedBox(
                  width: 80,
                )
              ]),
            ],
          )),
      Container(
        padding: EdgeInsets.all(5.0),
        child: Column(
          children: [
            Row(
              children: [
                Text(
                  "All friends (${aUser.getNbFriends()})",
                  style: TextStyle(fontSize: 20),
                )
              ],
            ),
            Divider(),
            Column(children: getListAffichageFriends(aUser))
          ],
        ),
      )
    ]));
  }

  String getFriendTitle(User aUser, bool myProfile) {
    if (myProfile) {
      return "My friends";
    } else {
      return "${aUser.getId()}'s friends";
    }
  }

  void setUserContainer(User aUser, bool myProfile) {
    labelUserIDController = Text(
      aUser.getId(),
      style: TextStyle(fontSize: 25.0, color: Colors.white),
    );
    labelNbFriends = Text("Friends: ${aUser.getNbFriends()}",
        style: TextStyle(fontSize: 20.0));
    List<Widget> skills = [];
    for (MapEntry<String, double> item in aUser.getActiveSkills().entries) {
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
          width: MediaQuery.of(context).size.width - 60,
          progressColor: Theme.of(context).primaryColor,
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
    if (aUser.getProfileDescription() != "") {
      containerDescription = Container(
        child: Column(
          children: [
            Text(
              aUser.getProfileDescription(),
              style: TextStyle(fontSize: 18),
              softWrap: true,
            ),
            Divider()
          ],
        ),
      );
    } else
      containerDescription = Container();
  }

  List<Widget> getListAffichageFriends(User aUser) {
    List<Widget> liste = [];
    for (var i = 0; i < aUser.getNbFriends(); i++) {
      User aFriend = User(aUser.getMyFriends()[i]);
      liste.add(GestureDetector(
          onTap: () {
            var _me = me;
            setState(() {
              me = _me;
              containerGeneral = getUserPage(aFriend, false);
            });
          },
          child: Row(
            children: [
              CircleAvatar(
                backgroundColor: aFriend.getProfilePicture(),
              ),
              SizedBox(
                width: 10,
              ),
              Flexible(
                  child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                    Text(aFriend.getId()),
                    Text(
                      "${aFriend.getNbFriends()} friends",
                      style: TextStyle(
                        overflow: TextOverflow.ellipsis,
                      ),
                    )
                  ])),
            ],
          )));
      liste.add(SizedBox(
        height: 10,
      ));
    }
    return liste;
  }

  Future<void> readUserID() async {
    //trouver le user id
    savedUserID = "testUser"; //(await storage.read(key: "_userID"))!;
    me = User(savedUserID);

    //userTest
    if (savedUserID == "testUser") {
      me.setMyFriends(["Adamou", "Sbasien", "Jean-Jean", "Mike", "Marie-Ève"]);
      me.setNbFriends(me.getMyFriends().length);
      me.setActiveSkills(
          {"Cooking": 34.3, "Skateboard": 12.1, "Chapeau melon": 99.90});
      me.setProfileDescription(
          "This is a test account made to preview what an actual account could display on a phone when the connection with the server is successful!");
    }
  }
}

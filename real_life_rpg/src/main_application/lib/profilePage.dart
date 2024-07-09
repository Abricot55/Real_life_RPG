import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:main_application/activeMemory.dart';
import 'package:main_application/picturePage.dart';
import 'package:percent_indicator/linear_percent_indicator.dart';

import 'User.dart';
import 'main.dart';
import 'utilities.dart';

class ProfilePage extends StatefulWidget {
  @override
  _ProfilePageState createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  final storage = const FlutterSecureStorage();

  //varaibles
  var savedUserID = "";
  late User me;
  late Activememory memory;
  var _selectedIndex = 0;

  //containers
  var containerGeneral = null;
  Container containerAdd = Container();

  //containers homePage
  var containerHomePage = Column();
  var containerRecherche = Column();
  var _searchMode = false;
  TextEditingController searchController = TextEditingController();
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
    //Initialisation if the profile page gets loaded for the first time
    if (containerGeneral == null) {
      readUserID();
      containerGeneral = getMainScreen(_selectedIndex);
    } else {
      me = memory.getMainUser();
    }
    return Scaffold(
        bottomNavigationBar: BottomNavigationBar(
            onTap: (int index) {
              memory.clearStack();
              var _memory = memory;
              setState(() {
                memory = _memory;
                _searchMode = false;
                _selectedIndex = index;
                containerGeneral = getMainScreen(index);
              });
            },
            selectedFontSize: 20,
            currentIndex: _selectedIndex,
            selectedItemColor: Theme.of(context).primaryColor,
            items: [
              BottomNavigationBarItem(
                  icon: Icon(
                    Icons.home,
                    size: 35.0,
                  ),
                  label: "Home"),
              BottomNavigationBarItem(
                  icon: Icon(
                    Icons.account_circle_sharp,
                    size: 35.0,
                  ),
                  label: "My profile"),
              BottomNavigationBarItem(
                  icon: Icon(
                    Icons.camera_alt,
                    size: 35.0,
                  ),
                  label: "Picture")
            ]),
        body: containerGeneral);
  }

  /**
   * @brief this function returns the widget to display when the user interracts with the naviagtion bar
   * @param index - the index of the item clicked on the navigation bar
   * @return the widget to display over the navigation bar
   */
  Center getMainScreen(int index) {
    if (index == 0) {
      return Center(child: getHomePage(_searchMode));
    } else if (index == 1) {
      return getUserPage(me, true, memory.isStackEmpty());
    } else if (index == 2) {
      //navigateToNextScreen(context, 4, me : me);
      if (me != null) {
        var _this = this;
        findCamera().then((camera) {
          if (camera != null) {
            var _memory = memory;
            _this.setState(() {
              memory = _memory;
              containerGeneral = TakePictureScreen(camera: camera, me: me);
            });
          } else {
            var _memory = memory;
            _this.setState(() {
              memory = _memory;
              containerGeneral = Text("No camera found");
            });
          }
        });
        return Center(child: Text("Waiting for camera"));
      } else {
        return Center(child: Text("Utilisateur invalide"));
      }
    } else {
      return Center(
        child: Text("QU'EST-CE QUE TU FAIS LÀ MAN??"),
      );
    }
  }

  /**
   * @brief this function returns the widget to display on the home page
   * @param _sM - the state of the home page (searchMode false or true)
   * @return the widget of the home screen or search screen
   */
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
                          Row(children: [
                            GestureDetector(
                              onTap: () {
                                var _memory = memory;
                                setState(() {
                                  memory = _memory;
                                  searchController.text = "";
                                  _searchMode = true;
                                  _itemsRecherche = getListeItems("");
                                  containerGeneral = getHomePage(_searchMode);
                                });
                              },
                              child: Icon(
                                Icons.search_rounded,
                                color: Theme.of(context).primaryColor,
                                size: 35.0,
                              ),
                            ),
                            SizedBox(
                              width: 10,
                            ),
                            GestureDetector(
                                onTap: () {
                                  writeStorage("_userToTalk", "");
                                  navigateToNextScreen(context, 4);
                                },
                                child: Icon(
                                  Icons.chat,
                                  color: Theme.of(context).primaryColor,
                                  size: 35.0,
                                ))
                          ])
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
                          GestureDetector(
                              onTap: () {
                                var _memory = memory;
                                setState(() {
                                  memory = _memory;
                                  _searchMode = false;
                                  containerGeneral = getHomePage(_searchMode);
                                });
                              },
                              child: Icon(
                                Icons.arrow_back,
                                color: Theme.of(context).primaryColor,
                                size: 35.0,
                              )),
                          Container(
                            width: MediaQuery.of(context).size.width - 45,
                            child: TextField(
                              controller: searchController,
                              onChanged: (text) {
                                var _memory = memory;
                                setState(() {
                                  memory = _memory;
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
                          //ElevatedButton(
                          //onPressed: () {
                          //rien
                          //},
                          //child: Text("Search"))
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

  /**
   * @brief this function returns the widget that displays a user page
   * @param aUser - the user to display
   * @param myProfile - displaying the main user's profile or someone else's profuile
   * @return the widget that displays a user page
   */
  Center getUserPage(User aUser, bool myProfile, bool emptyStack) {
    setUserContainer(aUser, myProfile);
    Row userRow = Row();
    containerAdd = Container();
    if (!me.getMyFriends().contains(aUser)) {
      containerAdd = Container(
          child: ElevatedButton(
              onPressed: () {
                if (!me.getMyFriends().contains(aUser)) {
                  var relation = jsonEncode(<String, String>{
                    'from': me.getId(),
                    'to': aUser.getId(),
                  });
                  sendRequest("ADD", path: "friend", jsonBody: relation);
                  me.addFriend(aUser);
                  var _memory = memory;
                  setState(() {
                    memory = _memory;
                    containerGeneral =
                        getUserPage(aUser, myProfile, emptyStack);
                  });
                }
              },
              child: Text(
                "Add friend",
                style: TextStyle(fontSize: 15),
              )));
    } else {
      containerAdd = Container(
          child: ElevatedButton(
              onPressed: () {
                //TODO ADAM - Unfriend

                me.removeFriend(aUser);
                var _memory = memory;
                setState(() {
                  memory = _memory;
                  containerGeneral = getUserPage(aUser, myProfile, emptyStack);
                });
              },
              child: Text(
                "Remove friend",
                style: TextStyle(fontSize: 15),
              )));
    }
    if (myProfile) {
      userRow = Row(children: [
        CircleAvatar(radius: 50, backgroundImage: aUser.getProfilePicture()),
        SizedBox(
          width: 10,
        ),
        Flexible(
            child: Container(
          height: 90,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Row(children: [
                Text(
                  "${aUser.getFirstName()} ${aUser.getSurname()}",
                  style: labelUserIDController.style,
                )
              ]),
              Row(children: [
                ElevatedButton(
                    onPressed: () {},
                    child: Container(
                        width: 96,
                        child: Row(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              Text(
                                "Edit profile",
                                style: TextStyle(fontSize: 15),
                              ),
                              Icon(
                                Icons.edit,
                                size: 20,
                              ),
                            ])))
              ])
            ],
          ),
        ))
      ]);
    } else {
      userRow = Row(
        children: [
          CircleAvatar(radius: 50, backgroundImage: aUser.getProfilePicture()),
          SizedBox(
            width: 10,
          ),
          Flexible(
              child: Container(
                  height: 90,
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Row(children: [
                        Text(
                          "${aUser.getFirstName()} ${aUser.getSurname()}",
                          style: labelUserIDController.style,
                        )
                      ]),
                      Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            containerAdd,
                            ElevatedButton(
                                onPressed: () {
                                  writeStorage("_userToTalk", aUser.getId());
                                  navigateToNextScreen(context, 4);
                                },
                                child: Row(
                                    mainAxisAlignment: MainAxisAlignment.center,
                                    children: [
                                      Text(
                                        "Chat",
                                        style: TextStyle(fontSize: 15),
                                      ),
                                      SizedBox(
                                        width: 5,
                                      ),
                                      Icon(
                                        Icons.chat,
                                        size: 20.0,
                                      )
                                    ]))
                          ])
                    ],
                  )))
        ],
      );
    }
    return Center(
      child: Column(children: [
        Container(
            padding: EdgeInsets.only(left: 5.0, right: 5.0),
            color: Theme.of(context).primaryColor,
            child: Column(
              children: [
                SizedBox(height: 30),
                getTopUserPageController(aUser, myProfile, emptyStack),
                userRow,
                SizedBox(
                  height: 5,
                )
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
                      var _memory = memory;
                      setState(() {
                        memory = _memory;
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

  /**
   * @brief this function returns the widget that displays the top of a user page
   * @param aUser - the user to display
   * @param myProfile - displaying the main user's profile or someone else's profuile
   * @return the widget that displays the top of a user page
   */
  Row getTopUserPageController(User aUser, bool myProfile, bool emptyStack) {
    Container container = Container(
      child: SizedBox(
        width: 35,
      ),
    );
    if (!emptyStack) {
      //ajouter la flèche back
      container = Container(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            GestureDetector(
                onTap: () {
                  var _prevUser = memory.pop();
                  var _memory = memory;
                  setState(() {
                    memory = _memory;
                    containerGeneral =
                        getFriendsPage(_prevUser, _prevUser == me);
                  });
                },
                child: Icon(
                  Icons.arrow_back,
                  color: Colors.white,
                  size: 35.0,
                )),
          ],
        ),
      );
    }
    if (myProfile) {
      return Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
        container,
        labelUserIDController,
        GestureDetector(
            onTap: () {
              navigateToNextScreen(context, 3);
            },
            child: Icon(
              Icons.settings,
              color: Colors.white,
              size: 35.0,
            )),
      ]);
    } else {
      return Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
        GestureDetector(
            onTap: () {
              var _memory = memory;
              setState(() {
                memory = _memory;
                if (emptyStack) {
                  switch (memory.getStartStack()) {
                    case 0:
                      _searchMode = true;
                      containerGeneral = getHomePage(_searchMode);
                      break;
                  }
                } else {
                  var _prevUser = memory.pop();
                  containerGeneral = getFriendsPage(_prevUser, _prevUser == me);
                }
              });
            },
            child: Icon(
              Icons.arrow_back,
              color: Colors.white,
              size: 35.0,
            )),
        labelUserIDController,
        SizedBox(
          width: 35,
        )
      ]);
    }
  }

  /**
   * @brief this function returns a string to present a user's skills
   * @param aUser - the user to display
   * @param myProfile - displaying the main user's profile or someone else's profuile
   * @return the correct string
   */
  String getStringSkills(User user, bool myProfile) {
    if (myProfile) {
      return "My skills";
    } else {
      return "${user.getId()}'s skills";
    }
  }

  List<Widget> getListeItems(String text) {
    if (text != "") {
      sendRequest("get", path: "users/relevant", urlMap: {"pseudo": text})
          .then((liste) {
        List<Widget> users = [];
        List<User> search = loadUserMultiple(liste.body);
        print("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        print(search);
        for (var i in search) {
          User aFriend = i;
          users.add(GestureDetector(
            onTap: () {
              var _memory = memory;
              setState(() {
                memory = _memory;
                if (aFriend.getId() == me.getId()) {
                  _selectedIndex = 1;
                  containerGeneral = getMainScreen(_selectedIndex);
                } else {
                  containerGeneral = getUserPage(aFriend,
                      aFriend.getId() == me.getId(), memory.isStackEmpty());
                }
              });
            },
            child: Row(
              children: [
                CircleAvatar(
                  backgroundImage: aFriend.getProfilePicture(),
                ),
                SizedBox(
                  width: 10,
                ),
                Text(i.getId())
              ],
            ),
          ));
        }
        var _memory = memory;
        setState(() {
          memory = _memory;
          _itemsRecherche = users;
          containerGeneral = getHomePage(_searchMode);
        });
      });
    }
    return [];
  }

  /**
   * @brief this function returns the widget that displays the friends of a user
   * @param aUser - the user to display
   * @param myProfile - displaying the main user's profile or someone else's profuile
   * @return the widget that displays the friends of the user
   */
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
                GestureDetector(
                    onTap: () {
                      var _memory = memory;
                      setState(() {
                        memory = _memory;
                        containerGeneral = getUserPage(
                            aUser, myProfile, memory.isStackEmpty());
                      });
                    },
                    child: Icon(
                      Icons.arrow_back,
                      color: Colors.white,
                      size: 35.0,
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

  /**
   * @brief this function sets all of the containers with the user's informtations
   * @param aUser - the user to display
   * @param myProfile - displaying the main user's profile or someone else's profuile
   */
  void setUserContainer(User aUser, bool myProfile) {
    labelUserIDController = Text(
      aUser.getId(), //aUser.getNickname();
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

  /**
   * @brief this function returns the interactive widgets of the user's friends
   * @param aUser - the user to display
   * @return the interactive widgets of the friends
   */
  List<Widget> getListAffichageFriends(User aUser) {
    List<Widget> liste = [];
    for (var i = 0; i < aUser.getNbFriends(); i++) {
      User aFriend = aUser.getMyFriends()[i];
      liste.add(GestureDetector(
          onTap: () {
            memory.push(aUser);
            var _memory = memory;
            setState(() {
              memory = _memory;
              containerGeneral = getUserPage(aFriend,
                  aFriend.getId() == me.getId(), memory.isStackEmpty());
            });
          },
          child: Row(
            children: [
              CircleAvatar(
                backgroundImage: aFriend.getProfilePicture(),
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
    //TODO SEBASTIEN, A TESTER
    savedUserID = (await storage.read(key: "_userID"))!; //"testUser";
    me = User("", "", "", "");
    memory = Activememory(me);
    sendRequest("get", path: "users", urlMap: {"key": savedUserID})
        .then((value) {
      if (value.body != "[]") {
        me = loadUser(value.body)!;
        memory.setMainUser(me);
      }
    });

    //userTest
    if (savedUserID == "testUser") {
      setUserTest(me);
    }
  }

  Future<void> writeStorage(_key, _value) async {
    storage.write(key: _key, value: _value);
  }
}

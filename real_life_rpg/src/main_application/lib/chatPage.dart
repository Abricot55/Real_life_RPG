import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'Message.dart';
import 'User.dart';
import 'main.dart';
//import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ChatPage extends StatefulWidget {
  @override
  _ChatPageState createState() => _ChatPageState();
}

class _ChatPageState extends State<ChatPage> {
  //final storage = const FlutterSecureStorage();
  String savedUserID = "testUser";
  User me = User("testUser");

  //variables
  bool inConvo = false;
  int indexContactTalking = 0;

  //containers
  List<Widget> widgetContacts = [];

  /**
   * @brief This function build all the widgets the user will see on the screen when the profile page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    readUserID();
    setUserTest(me);
    setColumnContacts();
    if (inConvo == false) {
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
                          Text(
                            me.getId(),
                            style: TextStyle(fontSize: 25),
                          ),
                          SizedBox(
                            width: 65,
                          )
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
                                    borderRadius: BorderRadius.all(
                                        Radius.circular(25.0)))),
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
                    children: widgetContacts,
                  ))
            ])
          ]));
    } else
      return getConvoContact(indexContactTalking);
  }

  Scaffold getConvoContact(int index) {
    User aContact = me.getMyContacts()[index];
    return Scaffold(
        body: Column(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Column(children: [
          SizedBox(height: 30),
          Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
            ElevatedButton(
                onPressed: () {
                  setState(() {
                    inConvo = false;
                  });
                },
                child: Text("back")),
            Text(
              aContact.getId(),
              style: TextStyle(fontSize: 25),
            ),
            SizedBox(
              width: 65,
            )
          ]),
          Divider(),
          Column(children: getWidgetsMessages()),
        ]),
        Column(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
          Row(
            children: [
              Container(
                width: MediaQuery.of(context).size.width - 90,
                child: TextField(
                  onChanged: (text) {},
                  decoration: new InputDecoration(
                      hintText: "Your message",
                      contentPadding: EdgeInsets.only(left: 10, right: 10),
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
                  child: Text("Send"))
            ],
          ),
        ])
      ],
    ));
  }

  List<Widget> getWidgetsMessages() {
    List<Widget> widgetsMessages = [];
    List<Message>? messages =
        me.getMyMessages()[me.getMyContacts()[indexContactTalking].getId()];
    for (int i = (messages?.length)! - 1; i >= 0; i--) {
      widgetsMessages.add(Row(
        children: [Text(messages![i].text)],
      ));
    }
    return widgetsMessages;
  }

  void setColumnContacts() {
    widgetContacts.clear();
    var contacts = me.getMyContacts();
    for (int i = 0; i < contacts.length; i++) {
      User aContact = contacts[i];
      Message lastMessage = me.getMyMessages()[aContact.getId()]![0];
      String sentFrom = "";
      if (lastMessage.idSentFrom == me.getId()) {
        sentFrom = "You: ";
      }
      widgetContacts.add(GestureDetector(
          onTap: () {
            setState(() {
              inConvo = true;
              indexContactTalking = i;
            });
          },
          child: Row(
            children: [
              CircleAvatar(
                backgroundColor: aContact.getProfilePicture(),
              ),
              SizedBox(
                width: 10,
              ),
              Flexible(
                  child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                    Text(aContact.getId()),
                    Text(
                      "${sentFrom}${lastMessage.text}",
                      style: TextStyle(
                        overflow: TextOverflow.ellipsis,
                      ),
                    )
                  ])),
            ],
          )));
      widgetContacts.add(SizedBox(
        height: 10,
      ));
    }
  }

  Future<void> readUserID() async {
    //savedUserID = (await storage.read(key: "_userID"))!;
  }
}

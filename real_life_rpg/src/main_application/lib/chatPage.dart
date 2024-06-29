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
  var me = null;

  //variables
  bool inConvo = false;
  int indexContactTalking = 0;

  //containers
  List<Widget> widgetContacts = [];
  List<Widget> messagesController = [];

  //controllers
  TextEditingController chatTextFieldController =
      TextEditingController(text: "");
  ScrollController chatScrollController = ScrollController();

  /**
   * @brief This function build all the widgets the user will see on the screen when the profile page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      chatScrollController
          .animateTo(chatScrollController.position.maxScrollExtent, duration: Duration(milliseconds: 500), curve: Curves.easeOut);
    });

    if (me == null) {
      me = User("testUser");
      readUserID();
      setUserTest(me);
    }
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
    messagesController = getWidgetsMessages();
    Scaffold convoContact = Scaffold(
      body: Column(
          //mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Expanded(
                child: Column(children: [
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
              Expanded(
                  child: SingleChildScrollView(
                      controller: chatScrollController,
                      child: Column(children: messagesController))),
            ])),
            Row(
              children: [
                Container(
                  width: MediaQuery.of(context).size.width - 90,
                  child: TextField(
                    style: TextStyle(fontSize: 20.0),
                    controller: chatTextFieldController,
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
                      if (chatTextFieldController.text.trim() != "") {
                        sendMessage(Message(
                            DateTime.now(),
                            me.getId(),
                            me.getMyContacts()[indexContactTalking].getId(),
                            chatTextFieldController.text.trim()));
                      }
                    },
                    child: Text("Send"))
              ],
            ),
          ]),
    );
    //chatScrollController.jumpTo(chatScrollController.position.maxScrollExtent);
    return convoContact;
  }

  List<Widget> getWidgetsMessages() {
    List<Widget> widgetsMessages = [];
    List<Message>? messages =
        me.getMyMessages()[me.getMyContacts()[indexContactTalking].getId()];
    for (int i = 0; i < (messages?.length)!; i++) {
      var alignment = MainAxisAlignment.end;
      var _color = Theme.of(context).primaryColor;
      if (messages![i].idSentFrom != me.getId()) {
        alignment = MainAxisAlignment.start;
        _color = Colors.black54;
      }
      widgetsMessages.add(Container(
          padding: EdgeInsets.all(5.0),
          child: Row(
            mainAxisAlignment: alignment,
            children: [
              Flexible(
                  child: Container(
                      constraints: BoxConstraints(
                          maxWidth: MediaQuery.of(context).size.width * 0.8),
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.all(Radius.circular(10.0)),
                        color: _color,
                      ),
                      padding: EdgeInsets.all(5),
                      child: Text(messages![i].text,
                          style: TextStyle(color: Colors.white, fontSize: 20))))
            ],
          )));
    }
    return widgetsMessages;
  }

  void setColumnContacts() {
    widgetContacts.clear();
    var contacts = me.getMyContacts();
    for (int i = 0; i < contacts.length; i++) {
      User aContact = contacts[i];
      Message lastMessage = me.getMyMessages()[aContact.getId()]![
          me.getMyMessages()[aContact.getId()]!.length - 1];
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

  void sendMessage(Message message) {
    chatTextFieldController.text = "";
    me.addMessage(message);
    var _me = me;
    setState(() {
      me = _me;
    });
  }

  Future<void> readUserID() async {
    //savedUserID = (await storage.read(key: "_userID"))!;
  }
}

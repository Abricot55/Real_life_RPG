import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:intl/intl.dart';
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
  bool animate = false;
  bool scrollDown = true;
  bool adjustToKeyboardUP = false;
  bool adjustToKeyboardDOWN = false;
  double position = 0;
  var messageDateFocus = null;

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
      if (scrollDown) {
        if (adjustToKeyboardUP) {
          chatScrollController
              .jumpTo(position + MediaQuery.of(context).viewInsets.bottom);
        } else if (adjustToKeyboardDOWN) {
          chatScrollController.jumpTo(position);
        } else {
          if (animate) {
            chatScrollController.animateTo(
                chatScrollController.position.maxScrollExtent,
                duration: Duration(milliseconds: 500),
                curve: Curves.easeOut);
          } else {
            chatScrollController
                .jumpTo(chatScrollController.position.maxScrollExtent);
          }
        }
      }
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
                        animate = false;
                        messageDateFocus = null;
                        adjustToKeyboardDOWN = false;
                        adjustToKeyboardUP = false;
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
                    onSubmitted: (String blabla) {
                      if (adjustToKeyboardDOWN == false) {
                        adjustToKeyboardUP = false;
                        adjustToKeyboardDOWN = true;
                        position = chatScrollController.position.pixels -
                            MediaQuery.of(context).viewInsets.bottom;
                      }
                    },
                    onTap: () {
                      if (adjustToKeyboardUP == false) {
                        adjustToKeyboardUP = true;
                        adjustToKeyboardDOWN = false;
                        position = chatScrollController.position.pixels;
                      }
                      animate = true;
                      scrollDown = true;
                    },
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
                      scrollDown = true;
                      adjustToKeyboardUP = false;
                      adjustToKeyboardDOWN = false;
                      if (chatTextFieldController.text.trim() != "") {
                        sendMessage(Message(
                            DateTime.now().toUtc(),
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
    widgetsMessages.add(getWidgetDate(messages![0], false));
    DateTime prevDate = messages[0].date;
    for (int i = 0; i < (messages?.length)!; i++) {
      //add date
      if (i > 0) {
        Duration diff = messages[i].date.difference(prevDate);
        if (messageDateFocus == messages[i]) {
          widgetsMessages.add(getWidgetDate(messages[i], true));
          if (diff.inDays > 0) {
            prevDate = messages[i].date;
          }
        } else if (diff.inDays > 0) {
          widgetsMessages.add(getWidgetDate(messages[i], false));
          prevDate = messages[i].date;
        }
      }
      //add message
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
                  child: GestureDetector(
                      onTap: () {
                        scrollDown = false;
                        messageDateFocus = messages[i];
                        var _me = me;
                        setState(() {
                          me = _me;
                        });
                      },
                      child: Container(
                          constraints: BoxConstraints(
                              maxWidth:
                                  MediaQuery.of(context).size.width * 0.8),
                          decoration: BoxDecoration(
                            borderRadius:
                                BorderRadius.all(Radius.circular(10.0)),
                            color: _color,
                          ),
                          padding: EdgeInsets.all(5),
                          child: Text(messages![i].text,
                              style: TextStyle(
                                  color: Colors.white, fontSize: 20)))))
            ],
          )));
    }
    return widgetsMessages;
  }

  Row getWidgetDate(Message message, bool heure) {
    DateTime _date = message.date.toLocal();
    String text =
        "${DateFormat('MMMM').format(DateTime(0, _date.month))} ${_date.day}";
    if (_date.year != DateTime.now().year) {
      text += " ${_date.year}";
    }
    if (heure) {
      text += " ${_date.hour}h${_date.minute}";
    }
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [Text(text)],
    );
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

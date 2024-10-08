import 'dart:convert';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
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
  final storage = const FlutterSecureStorage();
  String savedUserID = "";
  var me = null;

  //variables
  bool inConvo = false;
  User userTalking = User("", "", "", "");
  bool animate = false;
  bool scrollDown = true;
  bool adjustToKeyboardUP = false;
  bool adjustToKeyboardDOWN = false;
  double position = 0;
  var messageDateFocus = null;
  bool firstBuild = true;
  bool _searchMode = false;

  //containers
  List<Widget> widgetContacts = [];
  List<Widget> messagesController = [];

  //controllers
  TextEditingController chatTextFieldController =
      TextEditingController(text: "");
  ScrollController chatScrollController = ScrollController();
  TextEditingController searchController = TextEditingController();

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
          chatScrollController.animateTo(
              position + MediaQuery.of(context).viewInsets.bottom,
              duration: Duration(milliseconds: 200),
              curve: Curves.easeOut);
        } else if (adjustToKeyboardDOWN) {
          chatScrollController.animateTo(position,
              duration: Duration(milliseconds: 200), curve: Curves.easeOut);
        } else {
          if (animate) {
            chatScrollController.animateTo(
                chatScrollController.position.maxScrollExtent,
                duration: Duration(milliseconds: 500),
                curve: Curves.easeOut);
          } else {
            if (chatScrollController.hasClients) {
              chatScrollController
                  .jumpTo(chatScrollController.position.maxScrollExtent);
            }
          }
        }
      }
    });
    if (firstBuild == true) {
      firstBuild = false;
      readUserID().then((var value) {
        if (savedUserID == "testUser") {
          setUserTest(me);
        }
        setColumnContacts();
        getContextOpen();
        var _me = me;
        setState(() {
          me = _me;
        });
      });
    } else {
      if (_searchMode == false) {
        setColumnContacts();
      }
    }
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
                          GestureDetector(
                              onTap: () {
                                navigateToNextScreen(context, 2);
                              },
                              child: Icon(
                                Icons.home,
                                color: Theme.of(context).primaryColor,
                                size: 35.0,
                              )),
                          Text(
                            savedUserID,
                            style: TextStyle(fontSize: 25),
                          ),
                          SizedBox(
                            width: 65,
                          )
                        ]),
                    Row(
                      children: [
                        Container(
                          width: MediaQuery.of(context).size.width - 15,
                          child: TextField(
                            controller: searchController,
                            onChanged: (text) {
                              getListeItems(text);
                            },
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
                        //ElevatedButton(
                        //onPressed: () {
                        //rien
                        //},
                        //child: Text("Search"))
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
      return getConvoContact(userTalking);
  }

  /**
   * @brief This function build the conversation with a contact.
   * @param index -> The index of the contact to display the conversation in me._myContacts.
   * @return The widget which is all the stuff on screen.
   */
  Scaffold getConvoContact(User aContact) {
    //TODO ADAM - méthode qui update les messages à "seen"... je trouve un moyen que l'autre recoive l'info

    messagesController.clear();
    var messages = me.getMyMessages()[aContact.getId()];
    if (messages != null && messages.length > 0) {
      messagesController = getWidgetsMessages();
    }
    Scaffold convoContact = Scaffold(
      body: Column(
          //mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Expanded(
                child: Column(children: [
              SizedBox(height: 30),
              Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
                GestureDetector(
                    onTap: () {
                      setState(() {
                        inConvo = false;
                        animate = false;
                        messageDateFocus = null;
                        adjustToKeyboardDOWN = false;
                        adjustToKeyboardUP = false;
                        _searchMode = false;
                        searchController.text = "";
                        var messages = me.getMyMessages()[userTalking.getId()];
                        if (messages == null || messages.length == 0) {
                          me.removeGhostContact(userTalking);
                        }
                      });
                    },
                    child: Icon(
                      Icons.arrow_back,
                      color: Theme.of(context).primaryColor,
                      size: 35.0,
                    )),
                Text(
                  aContact.getNickame(),
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
                  width: MediaQuery.of(context).size.width - 40,
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
                GestureDetector(
                    onTap: () {
                      scrollDown = true;
                      adjustToKeyboardUP = false;
                      adjustToKeyboardDOWN = false;
                      if (chatTextFieldController.text.trim() != "") {
                        sendMessage(Message(DateTime.now().toUtc(), me,
                            userTalking, chatTextFieldController.text.trim()));
                      }
                    },
                    child: Icon(
                      Icons.send,
                      color: Theme.of(context).primaryColor,
                      size: 35.0,
                    ))
              ],
            ),
          ]),
    );
    //chatScrollController.jumpTo(chatScrollController.position.maxScrollExtent);
    return convoContact;
  }

  /**
   * @brief This function returns the lift of all the elements ton display on the conversation
   * @return The widgets of the conversation.
   */
  List<Widget> getWidgetsMessages() {
    List<Widget> widgetsMessages = [];
    List<Message>? messages = me.getMyMessages()[userTalking.getId()];
    widgetsMessages
        .add(getWidgetDate(messages![0], messageDateFocus == messages[0]));
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
      Container pdp = Container();
      double _maxWidth = MediaQuery.of(context).size.width * 0.8;
      if (messages![i].isSentFrom != me) {
        _maxWidth = (MediaQuery.of(context).size.width * 0.8) - 30;
        alignment = MainAxisAlignment.start;
        _color = Colors.black54;
        pdp = Container(
            child: Row(children: [
          CircleAvatar(
              radius: 20, backgroundImage: userTalking.getProfilePicture()),
          SizedBox(
            width: 10,
          )
        ]));
      }
      widgetsMessages.add(Container(
          padding: EdgeInsets.only(top: 10, left: 5, right: 5),
          child: Row(
            mainAxisAlignment: alignment,
            children: [
              pdp,
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
                          constraints: BoxConstraints(maxWidth: _maxWidth),
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
      //state
      if (i == messages.length - 1 && messages[i].isSentFrom == me) {
        widgetsMessages.add(getWidgetState(messages[messages.length - 1]));
      } else if (messages[i] == messageDateFocus &&
          messages[i].isSentFrom == me) {
        widgetsMessages.add(getWidgetState(messages[i]));
      }
    }
    return widgetsMessages;
  }

  /**
   * @brief This function build the widget that displays the state of a message
   * * @param message -> The message to display the state
   * @return The widget which displays the state
   */
  Row getWidgetState(Message message) {
    var _text = "Sending...";
    if (message.state == MessageState.sent) {
      _text = "Sent";
    }
    if (message.state == MessageState.seen) {
      _text = "Seen";
    }
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        Text(_text),
        SizedBox(
          width: 5,
        )
      ],
    );
  }

  /**
   * @brief This function build the widget that displays the hour of a message
   * * @param message -> The message to display the hour
   * @param heure -> If you display the hour
   * @return The widget which displays the hour
   */
  Container getWidgetDate(Message message, bool heure) {
    DateTime _date = message.date.toLocal();
    String text =
        "${DateFormat('MMMM').format(DateTime(0, _date.month))} ${_date.day}";
    if (_date.year != DateTime.now().year) {
      text += " ${_date.year}";
    }
    if (heure) {
      text += " ${_date.hour}h${_date.minute}";
    }
    return Container(
        padding: EdgeInsets.only(top: 5),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [Text(text)],
        ));
  }

  /**
   * @brief This function build all the widgets that lets the user select the conversation they want to open
   */
  void setColumnContacts() {
    widgetContacts.clear();
    var contacts = me.getMyContacts();
    for (int i = 0; i < contacts.length; i++) {
      User aContact = contacts[i];
      Message lastMessage = Message(
          DateTime(0),
          User("test", "test", "test", "test"),
          User("test", "test", "test", "test"),
          '');
      if (me.getMyMessages()[aContact.getId()]!.length > 0) {
        lastMessage = me.getMyMessages()[aContact.getId()]![
            me.getMyMessages()[aContact.getId()]!.length - 1];
      }
      String sentFrom = "";
      String _text = "";
      if (lastMessage.isSentFrom == me) {
        sentFrom = "You: ";
        if (lastMessage.state == MessageState.sending) {
          _text = "Sending...";
        }
      }
      String _date = "";
      DateTime now = DateTime.now();
      if (_text != "Sending...") {
        _text = sentFrom + lastMessage.text;
      }
      _date =
          "${DateFormat('MMMM').format(DateTime(0, lastMessage.date.month))} ${lastMessage.date.day}";
      if (lastMessage.date.year != now.year) {
        _date += " ${lastMessage.date.year}";
      } else if (now.day == lastMessage.date.day &&
          now.month == lastMessage.date.month) {
        _date = "${lastMessage.date.hour}:${lastMessage.date.minute}";
      }
      Text textDate = Text(_text,
          style: TextStyle(
            overflow: TextOverflow.ellipsis,
          ));
      TextPainter textPainter = TextPainter(
          text: TextSpan(text: _date, style: textDate.style),
          maxLines: 1,
          textDirection: ui.TextDirection.rtl);
      textPainter.layout(minWidth: 0.0, maxWidth: double.infinity);
      widgetContacts.add(GestureDetector(
          onTap: () {
            setState(() {
              inConvo = true;
              userTalking = me.getMyContacts()[i];
            });
          },
          child: Row(
            children: [
              CircleAvatar(
                radius: 20,
                backgroundImage: userTalking.getProfilePicture(),
              ),
              SizedBox(
                width: 10,
              ),
              Flexible(
                  child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                    Text(aContact.getNickame()),
                    Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Container(
                              constraints: BoxConstraints(
                                  maxWidth: MediaQuery.of(context).size.width -
                                      10 -
                                      40 -
                                      10 -
                                      textPainter.size.width -
                                      15),
                              child: textDate),
                          Text(_date)
                        ]),
                  ])),
            ],
          )));
      widgetContacts.add(SizedBox(
        height: 10,
      ));
    }
  }

  /**
   * @brief This function send the message to the servers and update the conversation on screen
   * * @param message -> The message to send
   */
  void sendMessage(Message message) {
    chatTextFieldController.text = "";
    me.addMessage(message);
    var _me = me;
    setState(() {
      me = _me;
    });
    
    var jsonThing = jsonEncode(<String, String>{"from": message.isSentFrom.getId(), "to": message.isSentTo.getId(), "message": message.text});
    sendRequest("ADD", path: "message", jsonBody: jsonThing).then((value)  {
      message.state = MessageState.sent;
      var _me = me;
      setState(() {
        me = _me;
      });
    });
  }

  /**
   * @brief This function reads the stoarage to know what discussion to display if the context asks it
   */
  Future<void> getContextOpen() async {
    final userToTalk = await storage.read(key: "_userToTalk");
    if (userToTalk != null) {
      if (userToTalk != "") {
        inConvo = true;
        bool found = false;
        for (var i = 0; i < me.getMyContacts().length; i++) {
          if (userToTalk == me.getMyContacts()[i].getId()) {
            userTalking = me.getMyContacts()[i];
            found = true;
          }
        }
        if (!found) {
          //amis
          for (var i = 0; i < me.getMyFriends().length; i++) {
            if (userToTalk == me.getMyFriends()[i].getId()) {
              userTalking = me.getMyFriends()[i];
              found = true;
              //addContact
              me.addContact(userTalking);
            }
          }
        }
        if (!found) {
          print(userToTalk);
          sendRequest("get",
              path: "/users/search",
              urlMap: {"_id": userToTalk}).then((value) {
            if (value.body != "[]") {
              userTalking = loadUser(value.body)!;
              me.addContact(userTalking);
              var _me = me;
              setState(() {
                me = _me;
              });
            }
          });
        }
      }
    }
  }

  List<Widget> getListeItems(String text) {
    if (text != "") {
      sendRequest("get", path: "users/relevant", urlMap: {"pseudo": text})
          .then((liste) {
        List<Widget> users = [];
        List<User> search = loadUserMultiple(liste.body);
        print(search);
        for (var i in search) {
          if (i.getId() != me.getId()) {
            User aFriend = i;
            users.add(GestureDetector(
              onTap: () {
                setState(() {
                  inConvo = true;
                  userTalking = aFriend;
                });
              },
              child: Row(
                children: [
                  CircleAvatar(
                    backgroundImage: userTalking.getProfilePicture(),
                  ),
                  SizedBox(
                    width: 10,
                  ),
                  Text(i.getNickame())
                ],
              ),
            ));
          }
        }
        setState(() {
          _searchMode = true;
          widgetContacts = users;
        });
      });
    } else {
      setState(() {
        _searchMode = false;
      });
    }
    return [];
  }

  Future<void> readUserID() async {
    savedUserID = (await storage.read(key: "_userID"))!;
    me = User("", "", "", "");
    sendRequest("get", path: "/users/search", urlMap: {"pseudo": savedUserID})
        .then((value) {
      if (value.body != "[]") {
        me = loadUser(value.body, messages: true)!;
      }
    });
  }
}

import 'dart:convert';

import 'package:camera/camera.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

import 'Message.dart';
import 'main.dart';

class User {
  //TODO ADAM -  tout les load et upload
  final String _key;
  final String _id;
  String _nickname = "";
  String _firstName = "";
  String _surname = "";

  List<User> _myFriends = [];
  int _nbFriends = 0;

  String _profileDescription = "";
  Map<String, double> _activeSkills = {};

  NetworkImage basicPdp = NetworkImage(
      "https://www.voici.fr/imgre/fit/~1~voi~2023~01~11~419636a9-5bf9-46de-bc19-e9184b465242.jpeg/1200x675/quality/80/focus-point/2050%2C1352/pitbull-que-devient-l-interprete-du-titre-i-know-you-want-me.jpg");
  late NetworkImage _profilePicture;
  List<XFile> _photos = [];

  Map<String, List<Message>> _myMessages = {};
  List<User> _myContacts = [];

  User(this._id, this._key, this._firstName, this._nickname) {
    //assert(_id != "");
    _profilePicture = basicPdp;
  }

  String getId() {
    return _id;
  }

  String getNickame() {
    if (_nickname == "") {
      //load the name
    }
    return _nickname;
  }

  void setNickame(String nickname) {
    this._nickname = nickname;
  }

  String getFirstName() {
    if (_firstName == "") {
      //load the name
    }
    return _firstName;
  }

  void setFirstName(String firstName) {
    this._firstName = firstName;
  }

  String getSurname() {
    if (_surname == "") {
      //load the name
    }
    return _surname;
  }

  void setSurname(String surname) {
    this._surname = surname;
  }

  List<User> getMyFriends() {
    if (_myFriends == []) {
      //load the list of the user's friend's id
    }
    return _myFriends;
  }

  void setMyFriends(List<User> myFriends) {
    this._myFriends = myFriends;
    this._nbFriends = myFriends.length;
  }

  int getNbFriends() {
    if (_nbFriends == 0) {
      //load the number of freinds
    }
    return _nbFriends;
  }

  void addFriend(User aFriend) {
    _nbFriends += 1;
    _myFriends.add(aFriend);

    //upload the new friend!!
  }

  void removeFriend(User aFriend) {
    _nbFriends -= 1;
    _myFriends.remove(aFriend);

    //upload the new friend list!!
  }

  void setNbFriends(int nbFriends) {
    this._nbFriends = nbFriends;
  }

  String getProfileDescription() {
    if (_profileDescription == "") {
      //load the description
    }
    return _profileDescription;
  }

  void setProfileDescription(String profileDescription) {
    this._profileDescription = profileDescription;
  }

  Map<String, double> getActiveSkills() {
    if (_activeSkills == {}) {
      //load active skills
    }
    return _activeSkills;
  }

  void setActiveSkills(Map<String, double> activeSkills) {
    this._activeSkills = activeSkills;
  }

  NetworkImage getProfilePicture() {
    if (_profilePicture == basicPdp) {
      //load profile picture
    }
    return _profilePicture;
  }

  void setProfilePicture(NetworkImage profilePicture) {
    this._profilePicture = profilePicture;
  }

  Map<String, List<Message>> getMyMessages() {
    if (_myMessages == {}) {
      //load messages
    }
    return _myMessages;
  }

  void addMessage(Message message) {
    String idOtherUser = message.isSentFrom.getId();
    User _otherUser = message.isSentFrom;
    if (idOtherUser == _id) {
      idOtherUser = message.isSentTo.getId();
      _otherUser = message.isSentTo;
    }
    if (_myMessages.keys.contains(idOtherUser)) {
      _myMessages[idOtherUser] = (_myMessages[idOtherUser]! + [message]);
    } else {
      /*
      bool trouve = false;
      for (var i = 0; i < _nbFriends && !trouve; i++) {
        if (idOtherUser == _myFriends[i].getId()) {
          //if a friend - already created
          addContact(_myFriends[i]);
          trouve = true;
        }
      }
       */
      //if (!trouve) {
      //if not friend
      addContact(_otherUser);
      //}
      _myMessages[idOtherUser] = [message];
    }
    //update contact order
    int indexUser = -1;
    late User otherUser;
    for (var i = 0; i < _myContacts.length && indexUser == -1; i++) {
      if (_myContacts[i].getId() == idOtherUser) {
        indexUser = i;
        otherUser = _myContacts[i];
      }
    }
    for (var i = indexUser; i > 0; i--) {
      _myContacts[i] = _myContacts[i - 1];
    }
    _myContacts[0] = otherUser;

    //updload myContacts and myMessages
  }

  void setMyMessages(Map<String, List<Message>> myMessages) {
    this._myMessages = myMessages;

    _myContacts.clear();
    //update contacts UNOPTIMIZED
    for (MapEntry<String, List<Message>> item in _myMessages.entries) {
      bool isFriend = false;
      for (int i = 0; i < _myFriends.length && !isFriend; i++) {
        if (_myFriends[i].getId() == item.key) {
          isFriend = true;
          _myContacts.add(_myFriends[i]);
        }
      }
      //new user
      // TODO SEBASTIEN, À TESTER
      if (!isFriend) {
        sendRequest("get", path: "users", urlMap: {"pseudo": item.key})
            .then((value) {
          if (value.body != "[]") {
            User newUser = loadUser(value.body)!;
            _myContacts.add(newUser);
          }
        });
      }
    }
  }

  List<User> getMyContacts() {
    if (_myContacts == []) {
      //load my contacts
    }
    return this._myContacts;
  }

  void addContact(User aUser) {
    _myContacts.add(aUser);
    _myMessages[aUser.getId()] = [];
  }

  void loadPhotos() {
    sendRequest("get", path: "photo", urlMap: {"key": this._key})
        .then((value) {

          //TODO les photos c'est pas encore finalisé, la sérialisation et désérialisation, so c'est sur que le code en dessous load pas vraiment les photos mais la requete c'est la bonne.
          _photos = value;
    });
  }
  
  void loadFriend(){
    sendRequest()
  }

  /**
   * @brief Remove the contact if no messages has been sent after opening a new conversation with a non-contact
   * @param aUser - The ghost contact
   */
  void removeGhostContact(User aUser) {
    _myContacts.remove(aUser);
  }
}

User? loadUser(String json,
    {photos = false, messages = false, friends = false}) {
  dynamic decodedJson = jsonDecode(json)[0];
  User? user;
  if (decodedJson is Map<String, dynamic>) {
    try {
      var _id = decodedJson['_id'] as String;
      var _key = decodedJson['_key'] as String;
      var pseudo = decodedJson['pseudo'] as String;
      var name = decodedJson['name'] as String;
      user = User(_id, _key, name, pseudo);
    } catch (Exception) {
      print("OH NO");
    }
  }
  if (user != null && photos){
    user.loadPhotos();
  }
  return user;
}

List<User> loadUserMultiple(String json) {
  List<dynamic> decodedJson = jsonDecode(json);
  List<User> liste = [];
  for (dynamic i in decodedJson) {
    if (i is Map<String, dynamic>) {
      try {
        var _id = i['_id'] as String;
        var _key = i['_key'] as String;
        var pseudo = i['pseudo'] as String;
        var name = i['name'] as String;
        liste.add(User(_id, _key, name, pseudo));
      } catch (Exception) {
        print("OH NO");
      }
    }
  }
  return liste;
}

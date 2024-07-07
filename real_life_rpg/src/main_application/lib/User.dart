import 'package:camera/camera.dart';
import 'package:flutter/material.dart';

import 'Message.dart';
import 'main.dart';

class User {
  //TODO ADAM -  tout les load et upload

  final String _id;
  String _name = "";
  List<User> _myFriends = [];
  int _nbFriends = 0;
  String _profileDescription = "";
  Map<String, double> _activeSkills = {};
  Color _profilePicture = Colors.grey;
  List<XFile> _photos = [];
  Map<String, List<Message>> _myMessages = {};
  List<User> _myContacts = [];

  User(this._id) {
    assert(_id != "");
  }

  String getId() {
    return _id;
  }

  String getName() {
    if (_name == "") {
      //load the name
    }
    return _name;
  }

  void setName(String name) {
    this._name = name;
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

  void addFriend(User aFriend){
    _nbFriends += 1;
    _myFriends.add(aFriend);

    //upload the new friend!!
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

  Color getProfilePicture() {
    if (_profilePicture == Colors.grey) {
      //load profile picture
    }
    return _profilePicture;
  }

  void setProfilePicture(Color profilePicture) {
    this._profilePicture = profilePicture;
  }

  Map<String, List<Message>> getMyMessages() {
    if (_myMessages == {}) {
      //load messages
    }
    return _myMessages;
  }

  void addMessage(Message message) {
    String idOtherUser = message.idSentFrom;
    if (idOtherUser == _id) {
      idOtherUser = message.idSentTo;
    }
    if (_myMessages.keys.contains(idOtherUser)) {
      _myMessages[idOtherUser] = (_myMessages[idOtherUser]! + [message]);
    } else {
      bool trouve = false;
      for(var i = 0; i < _nbFriends && !trouve; i++){
        if(idOtherUser == _myFriends[i].getId()){
          addContact(_myFriends[i]);
          trouve = true;
        }
      }
      if (!trouve) {
        addContact(User(idOtherUser));
      }
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
    for(var i = indexUser; i > 0; i--){
      _myContacts[i] = _myContacts[i-1];
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
      if (!isFriend) {
        _myContacts.add(User(item.key));
      }
    }
  }

  List<User> getMyContacts() {
    if (_myContacts == []){
      //load my contacts
    }
    return this._myContacts;
  }

  void addContact(User aUser) {
    _myContacts.add(aUser);
    _myMessages[aUser.getId()] = [];
  }

  /**
   * @brief Remove the contact if no messages has been sent after opening a new conversation with a non-contact
   * @param aUser - The ghost contact
   */
  void removeGhostContact(User aUser) {
    _myContacts.remove(aUser);
  }
}

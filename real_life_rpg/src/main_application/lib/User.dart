import 'package:camera/camera.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class User {
  final String _id;
  String _name = "";
  List<User> _myFriends = [];
  int _nbFriends = 0;
  String _profileDescription = "";
  Map<String, double> _activeSkills = {};
  Color _profilePicture = Colors.grey;
  List<XFile> _photos = [];

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
}

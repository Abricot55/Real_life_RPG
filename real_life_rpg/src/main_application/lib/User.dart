


import 'package:flutter/cupertino.dart';

class User {
  final String id;
  String name = "";
  List<String> myFriends = [];
  int nbFriends = 0;
  String profileDescription = "";
  Map<String, double> activeSkills = {};

  User(this.id){
    assert(id != "");
  }

  void loadUserData(List<String> dataLoad){
    for(int i = 0; i < dataLoad.length; i++){
      switch(dataLoad[i]){
        case("name"):
          //load the name
          break;
        case("myFriends"):
        //load the list of the user's friend's id
          break;
        case("nbFriends"):
        //load the number of friends
          break;
        case("description"):
        //load the description
          break;
      }
    }
  }

}
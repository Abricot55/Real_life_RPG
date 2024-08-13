import 'package:flutter/material.dart';

import 'User.dart';

enum PublicationType { text, image, levelUp }

class Publication {
  final PublicationType type;
  User user;
  List<String> likes = [];
  DateTime date;
  //List<Comments> comments = []; faire un objet commentaire qui peut se faire afficher et interragir avec, avoir des likes et reply


  //message
  String message = "";

  //image
  NetworkImage image = NetworkImage(
      "https://www.voici.fr/imgre/fit/~1~voi~2023~01~11~419636a9-5bf9-46de-bc19-e9184b465242.jpeg/1200x675/quality/80/focus-point/2050%2C1352/pitbull-que-devient-l-interprete-du-titre-i-know-you-want-me.jpg");

  //levelUp
  String skill = "";
  int level = 0;

  Publication(this.type, this.user, this.date) {}

  void setMessage(String message) {
    this.message = message;
  }

  void setImage(NetworkImage image) {
    this.image = image;
  }

  void setSkill(String skill, int level) {
    this.skill = skill;
    this.level = level;
  }

  Column getPubWidget() {
    Column content;
    switch (type) {
      case PublicationType.text:
        content = Column(
          children: [Text(message)],
        );
        break;
      case PublicationType.image:
        content = Column(
          children: [Image(image: image)],
        );
        break;
      case PublicationType.levelUp:
        content = Column(
          children: [
            Text(
                "Congratulations, ${user.getFirstName()} reached level ${level} in ${skill}")
          ],
        );
        break;
    }
    return Column(
      children: [
        Row(children: [
          CircleAvatar(radius: 20, backgroundImage: user.getProfilePicture()),
          SizedBox(width: 10,),
          Text(user.getNickame(), style: TextStyle(fontSize: 20),)
        ]),
        content,
        Row(children: [
          ElevatedButton(onPressed: () {}, child: Text("Like")),
          ElevatedButton(onPressed: () {}, child: Text("Comment"))
        ]),
        Divider()
      ],
    );
  }
}

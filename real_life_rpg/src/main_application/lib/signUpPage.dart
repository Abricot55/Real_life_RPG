import 'package:flutter/material.dart';
import 'main.dart';

class SignUpPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            createRow(context, "Name"),
            createRow(context, "Pseudo"),
            createRow(context, "Email adress"),
            createRow(context, "Birth Date"),
            ElevatedButton(onPressed: () {navigateToNextScreen(context, 1);}, child: Text("Sign Up"))
          ],
        ),
      ),
    );
  }

  Row createRow(context, String text) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(text + " :   "),
        Container(
          width: MediaQuery.of(context).size.width * 0.05,
          height: MediaQuery.of(context).size.height * 0.03,
          decoration: BoxDecoration(
              borderRadius: BorderRadius.all(Radius.elliptical(1, 2)),
              border: Border.all(color: Colors.black)),
          child: TextField(
            decoration: InputDecoration(border: InputBorder.none),
          ),
        )
      ],
    );
  }
}

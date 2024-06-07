import 'dart:convert';

import 'package:flutter/material.dart';
import 'main.dart';

/**
 * @brief This class create represent the object which is the widget on screen when on signup page.
 */
class SignUpPage extends StatelessWidget {
  /**
   * @brief This function build all the widgets the user will see on the screen when the home page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    var nameController = TextEditingController();
    var pseudoController = TextEditingController();
    var emailController = TextEditingController();
    var birthController = TextEditingController();
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            createRow(context, "Name", nameController),
            createRow(context, "Pseudo", pseudoController),
            createRow(context, "Email adress", emailController),
            createRow(context, "Birth Date", birthController),
            ElevatedButton(
                onPressed: () {
                  if (nameController.text.isNotEmpty &&
                      pseudoController.text.isNotEmpty &&
                      emailController.text.isNotEmpty &&
                      birthController.text.isNotEmpty) {
                    var user = jsonEncode(<String, String>{
                      'name': nameController.text,
                      'pseudo': pseudoController.text,
                      'email': emailController.text,
                      'birthday': birthController.text,
                      'level': "0"
                    });
                    sendRequest("ADD", path: "users", jsonBody: user);
                    navigateToNextScreen(context, 1);
                  }
                },
                child: Text("Sign Up"))
          ],
        ),
      ),
    );
  }

  /**
   * @brief This function create and return a Row widget. This type of widget will be used to create the different entries fields on the sign up page.
   * @param context -> The context in which the home page is created.
   * @param text -> The text that need to be displayed on the side of the textfield.
   * @param controller -> A controller so the textfield can be accessed later.
   * @return the resulting row widget.
   */
  Row createRow(context, String text, TextEditingController controller) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(text + " :   "),
        Container(
          width: MediaQuery.of(context).size.width * 0.1,
          height: MediaQuery.of(context).size.height * 0.03,
          decoration: BoxDecoration(
              borderRadius: BorderRadius.all(Radius.elliptical(1, 2)),
              border: Border.all(color: Colors.black)),
          child: TextField(
            controller: controller,
            decoration: InputDecoration(border: InputBorder.none),
          ),
        )
      ],
    );
  }
}

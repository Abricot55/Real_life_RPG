import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
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
        Row(mainAxisAlignment: MainAxisAlignment.center, children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text("Name: "),
              SizedBox(height: 35),
              Text("Pseudo: "),
              SizedBox(height: 35),
              Text("Email adress: "),
              SizedBox(height: 35),
              Text("Birth date: ")
            ],
          ),
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              createTextField("", nameController, context, TextInputType.text),
              SizedBox(height: 5),
              createTextField("", pseudoController, context, TextInputType.text),
              SizedBox(height: 5),
              createTextField("ex : username@gmail.com", emailController, context, TextInputType.emailAddress),
              SizedBox(height: 5),
              createTextField("DD/MM/YYYY", birthController, context, TextInputType.datetime)
            ],
          )
        ]),
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
            child: Text("Sign Up")),
        ElevatedButton(
            onPressed: () {
              navigateToNextScreen(context, 1);
            },
            child: Text("Cancel"))
      ])),
    );
  }

  /**
   * @brief This function create and return a Row widget. This type of widget will be used to create the different entries fields on the sign up page.
   * @param context -> The context in which the home page is created.
   * @param text -> The text that need to be displayed on the side of the textfield.
   * @param controller -> A controller so the textfield can be accessed later.
   * @return the resulting row widget.
   */
  Container createTextField(String text, TextEditingController controller, BuildContext context, TextInputType _keyboardType) {
    return Container(
      width: MediaQuery.of(context).size.width * 0.65,
      height: 50.0,
      decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(10.0),
          border: Border.all(color: Colors.black)),
      child: TextField(
        keyboardType: _keyboardType,
        autocorrect: false,
        controller: controller,
        decoration: new InputDecoration(
          border: InputBorder.none,
          contentPadding: EdgeInsets.all(10.0),
          hintText: text,
        ),
      ),
    );
  }
}

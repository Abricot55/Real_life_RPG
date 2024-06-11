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
    var password1Controller = TextEditingController();
    var password2Controller = TextEditingController();

    var columnKey = GlobalKey();

    return Scaffold(
      body: Center(
          child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
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
              Text("Birth date: "),
              SizedBox(height: 35),
              Text("Password: "),
              SizedBox(height: 35),
              Text("Confirm password: ")
            ],
          ),
          Column(
            key: columnKey,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              createTextField("", nameController, context, TextInputType.text),
              SizedBox(height: 5),
              createTextField(
                  "", pseudoController, context, TextInputType.text),
              SizedBox(height: 5),
              createTextField("ex : username@gmail.com", emailController,
                  context, TextInputType.emailAddress),
              SizedBox(height: 5),
              createTextField("DD/MM/YYYY", birthController, context,
                  TextInputType.datetime),
              SizedBox(height: 5),
              createTextField(
                  "", password1Controller, context, TextInputType.text,
                  invisible: true),
              SizedBox(height: 5),
              createTextField(
                  "", password2Controller, context, TextInputType.text,
                  invisible: true),
              SizedBox(height: 5),
            ],
          )
        ]),
        ElevatedButton(
            onPressed: () {
              if (nameController.text.trim().isNotEmpty &&
                  pseudoController.text.trim().isNotEmpty &&
                  emailController.text.trim().isNotEmpty &&
                  birthController.text.trim().isNotEmpty) {
                var user = jsonEncode(<String, String>{
                  'name': nameController.text,
                  'pseudo': pseudoController.text,
                  'email': emailController.text,
                  'birthday': birthController.text,
                  'level': "0",
                  'password': password1Controller.text
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
  Container createTextField(String text, TextEditingController controller,
      BuildContext context, TextInputType _keyboardType,
      {invisible = false}) {
    return Container(
      key: key,
      width: MediaQuery.of(context).size.width * 0.65,
      height: 50.0,
      decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(10.0),
          border: Border.all(color: Colors.black)),
      child: TextField(
        obscureText: invisible,
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

  void createLabel(String text, GlobalKey key, int spot,
      {Color color = Colors.red, double fontSize = 16.0}) {
    final RenderObject renderObject = key.currentContext!.findRenderObject()!;

    /*if (renderObject is RenderBox) {
      Container(
          child: Text(
        text,
        style: TextStyle(
          color: color,
          fontSize: fontSize,
        ),
      ));
      RenderBox containerRenderBox = renderObject;
      RenderObject? childRenderObject = containerRenderBox.visitChildren(visitor);
      for (int i = 0; i < spot; i++) {
        if (childRenderObject == null) {
          break;
        }
        childRenderObject = childRenderObject!.parentData?.nextSibling;
      }
      if (childRenderObject != null) {
        containerRenderBox.insert(childRenderObject,
            after: label.renderObject!);
      } else {
        containerRenderBox.add(label.renderObject!);
      }

      // Mark the Column as needing layout
      renderObject.markNeedsLayout();
    }*/
  }

  /**
   * @brief Function used to validate the creation of a new password for an account.
   * @param password1 -> The first password written by the user.
   * @param password2 -> The "Confirm password" field.
   * @param kay_pass1 -> The Global key of the first password container
   * @return true if password are valid.
   */
  bool okayPasswordCreation(
      int spot, GlobalKey keyPass1, String password1, String password2) {
    bool isOkay = true;
    if (password1 != password2) {
      createLabel("The two passwords are not the same", keyPass1, spot);
      isOkay = false;
    }
    if (password1.trim() != password2) {
      return isOkay;
    }
    return isOkay;
  }
}

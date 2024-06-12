import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'main.dart';

/**
 * @brief This class create represent the object which is the widget on screen when on signup page.
 */
class SignUpPage extends StatefulWidget {
  @override
  SignUpPageState createState() => SignUpPageState();
}

class SignUpPageState extends State<SignUpPage> {
  var nameController = TextEditingController();
  var pseudoController = TextEditingController();
  var emailController = TextEditingController();
  var birthController = TextEditingController();
  var password1Controller = TextEditingController();
  var password2Controller = TextEditingController();
  List<Widget> rowList = [];

  @override
  void initState() {
    super.initState();
    rowList = [
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        SizedBox(height: 5),
      ]),
    ];
  }

  /**
   * @brief This function build all the widgets the user will see on the screen when the home page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    insertRows();
    return Scaffold(
      body: Center(
          child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
        ...rowList,
        ElevatedButton(
            onPressed: () {
              if (nameController.text.trim().isNotEmpty &&
                  pseudoController.text.trim().isNotEmpty &&
                  emailController.text.trim().isNotEmpty &&
                  birthController.text.trim().isNotEmpty &&
                  okayPasswordCreation(
                      password1Controller.text, password2Controller.text)) {
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
        Row(mainAxisAlignment: MainAxisAlignment.center, children: [
          SizedBox(height: 5),
        ]),
        ElevatedButton(
            onPressed: () {
              navigateToNextScreen(context, 1);
            },
            child: Text("Cancel"))
      ])),
    );
  }

  void insertRows() {
    rowList.removeAt(0);
    rowList.insert(
        0,
        Row(mainAxisAlignment: MainAxisAlignment.center, children: [
          Container(
              width: 150,
              alignment: Alignment.centerRight,
              child: Text("Name : ")),
          createTextField("", nameController, context, TextInputType.text),
        ]));
    rowList.removeAt(2);
    rowList.insert(
      2,
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        Container(
            width: 150,
            alignment: Alignment.centerRight,
            child: Text("Pseudo : ")),
        createTextField("", pseudoController, context, TextInputType.text),
      ]),
    );
    rowList.removeAt(4);
    rowList.insert(
      4,
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        Container(
            width: 150,
            alignment: Alignment.centerRight,
            child: Text("Email : ")),
        createTextField("ex : username@gmail.com", emailController, context,
            TextInputType.emailAddress),
      ]),
    );
    rowList.removeAt(6);
    rowList.insert(
      6,
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        Container(
            width: 150,
            alignment: Alignment.centerRight,
            child: Text("Birth date : ")),
        createTextField(
            "DD/MM/YYYY", birthController, context, TextInputType.datetime),
      ]),
    );
    rowList.removeAt(8);
    rowList.insert(
      8,
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        Container(
            width: 150,
            alignment: Alignment.centerRight,
            child: Text("Password : ")),
        createTextField("", password1Controller, context, TextInputType.text,
            invisible: true),
      ]),
    );
    rowList.removeAt(10);
    rowList.insert(
      10,
      Row(mainAxisAlignment: MainAxisAlignment.center, children: [
        Container(
            width: 150,
            alignment: Alignment.centerRight,
            child: Text("Confirm Password : ")),
        createTextField("", password2Controller, context, TextInputType.text,
            invisible: true),
      ]),
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

  void createLabel(String text, int spot,
      {Color color = Colors.red, double fontSize = 16.0}) {
    rowList.removeAt(spot);
    setState(() {
      rowList.insert(
          spot,
          Row(mainAxisAlignment: MainAxisAlignment.center, children: [
            Container(
                alignment: Alignment.center,
                child: Text(
                  text,
                  style: TextStyle(color: color),
                ))
          ]));
    });
  }

  void removeLabel(String text, int spot) {
    rowList.removeAt(spot);
    setState(() {
      rowList.insert(
          spot,
          Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [SizedBox(height: 5)]));
    });
  }

  /**
   * @brief Function used to validate the creation of a new password for an account.
   * @param password1 -> The first password written by the user.
   * @param password2 -> The "Confirm password" field.
   * @param kay_pass1 -> The Global key of the first password container
   * @return true if password are valid.
   */
  bool okayPasswordCreation(String password1, String password2) {
    int spot = 9;
    bool isOkay = true;
    if (password1.length < 8) {
      createLabel("The must be at least 8 characters long!", spot);
      isOkay = false;
    } else if (password1.replaceAll(RegExp(r'\s'), '*') != password2) {
      createLabel("The password cannot contain any white characters!", spot);
      isOkay = false;
    } else if (password1 != password2) {
      createLabel("The two passwords must be the same!", spot);
      isOkay = false;
    } else {}
    return isOkay;
  }

  void delayAndExecute(String first, String function,
      {String second = ""}) async {
    await Future.delayed(Duration(seconds: 1));
    print('This message is printed after a 1-second delay.');
  }
}

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:main_application/utilities.dart';
import 'main.dart';
import 'package:intl/intl.dart';
import 'package:intl/date_symbol_data_local.dart';

/// @brief This class create represent the object which is the widget on screen when on signup page.
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

  /// @brief The initStateFunction is called only one time and is used to setup the initial state of the class.
  @override
  void initState() {
    super.initState();
    initializeDateFormatting();
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

  /// @brief This function build all the widgets the user will see on the screen when the home page is loaded. This function is automatically called.
  /// @param context -> The context in which the home page is created.
  /// @return The widget which is all the stuff on screen.
  @override
  Widget build(BuildContext context) {
    insertRows();
    return Scaffold(
      body: Center(
          child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
        ...rowList,
        ElevatedButton(
            onPressed: () {
              pseudoController.text = pseudoController.text.trim();
              emailController.text = emailController.text.trim();
              nameController.text = nameController.text.trim();
              password1Controller.text = password1Controller.text.trim();
              var b = okayNameCreation(nameController.text);
              var b2 = okayPseudoCreation(pseudoController.text);
              var b3 = okayEmailCreation(emailController.text);
              var b4 = okayBirthCreation(birthController.text);
              var b5 = okayPasswordCreation(
                  password1Controller.text, password2Controller.text);
              if (b && b2 && b3 && b4 && b5) {
                var user = jsonEncode(<String, String>{
                  'name': nameController.text,
                  'pseudo': pseudoController.text,
                  'email': emailController.text,
                  'birthday': birthController.text,
                  'level': "0",
                  'password': hash_string(password1Controller.text),
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

  /// @brief This function insert all rows that are important to the sign up process.
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
            "DD/MM/YYYY", birthController, context, TextInputType.datetime,
            date: true),
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

  /// @brief This function create and return a Container widget. This type of widget will be used to create the different entries fields on the sign up page.
  /// @param context -> The context in which the home page is created.
  /// @param text -> The text that need to be displayed on the side of the textfield.
  /// @param controller -> A controller so the textfield can be accessed later.
  /// @return the resulting Container widget.
  Container createTextField(String text, TextEditingController controller,
      BuildContext context, TextInputType _keyboardType,
      {invisible = false, date = false}) {
    TextField field = TextField(
      obscureText: invisible,
      keyboardType: _keyboardType,
      autocorrect: false,
      controller: controller,
      decoration: new InputDecoration(
        border: InputBorder.none,
        contentPadding: EdgeInsets.all(10.0),
        hintText: text,
      ),
    );
    if (date) {
      field = TextField(
          obscureText: invisible,
          keyboardType: _keyboardType,
          autocorrect: false,
          controller: controller,
          decoration: new InputDecoration(
            icon: Icon(Icons.calendar_today),
            border: InputBorder.none,
            contentPadding: EdgeInsets.all(10.0),
            hintText: text,
          ),
          readOnly: true,
          onTap: () async {
            DateTime? pickedDate = await showDatePicker(
              context: context,
              initialDate: DateTime.now(),
              firstDate: DateTime(1900),
              lastDate: DateTime(2040),
            );
            if (pickedDate != null) {
              String formattedDate =
                  DateFormat('dd-MM-yyyy').format(pickedDate);
              setState(() {
                controller.text = formattedDate;
              });
            }
          });
    }
    return Container(
        width: MediaQuery.of(context).size.width * 0.58,
        height: 50.0,
        decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(10.0),
            border: Border.all(color: Colors.black)),
        child: field);
  }

  /// @brief This function create a label at a specific spot in the row lits that compose the signup page.
  /// @param text -> The text that the label will display.
  /// @param spot -> The spot in the list which will get replace by the label.
  /// @param Color -> The color of the text, red by default.
  /// @param fontSize -> The size of the font of the label, 16 by default.
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

  /// @brief This function create a empty sizedBox at a specific spot in the row list that compose the signup page.
  /// @param spot -> The spot in the list which will get replace by the SizedBox.
  void removeLabel(int spot) {
    rowList.removeAt(spot);
    setState(() {
      rowList.insert(
          spot,
          Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [SizedBox(height: 5)]));
    });
  }

  /// @brief This function verifies if an user with this pseudonym can sign up.
  /// @param pseudo -> The pseudonym.
  /// @return A boolean which indicate if the pseudonym is valid.
  bool okayPseudoCreation(String pseudo) {
    bool isOkay = true;
    int spot = 3;
    sendRequest("get", path: "users/search/", urlMap: {"pseudo": pseudo})
        .then((response) {
      if (response.body != "[]") {
        createLabel(
            "This pseudo is already taken, please choose another one", spot);
        isOkay = false;
      } else if (pseudo.length > 20) {
        createLabel("The pseudo must be less than 20 characters", spot);
        isOkay = false;
      } else if (pseudo.isEmpty) {
        createLabel("The pseudo must not be empty", spot);
        isOkay = false;
      } else {
        removeLabel(spot);
      }
    });
    return isOkay;
  }

  /// @brief This function verifies if an user with this name can sign up.
  /// @param name -> The name.
  /// @return A boolean which indicate if the name is valid.
  bool okayNameCreation(String name) {
    bool isOkay = false;
    int spot = 1;
    if (name.contains(RegExp(r'[^a-zA-Z\\s]'))) {
      createLabel("The name must contains only letters", spot);
    } else if (name.length > 20) {
      createLabel("The name must be less than 20 characters", spot);
    } else if (name.isEmpty) {
      createLabel("The name must not be empty", spot);
    } else {
      removeLabel(spot);
      isOkay = true;
    }
    return isOkay;
  }

  /// @brief Function used to validate the creation of a new password for an account.
  /// @param password1 -> The first password written by the user.
  /// @param password2 -> The "Confirm password" field.
  /// @return true if password are valid.
  bool okayPasswordCreation(String password1, String password2) {
    int spot = 9;
    bool isOkay = true;
    if (password1.length < 8) {
      createLabel("The password must be at least 8 characters long!", spot);
      isOkay = false;
    } else if (password1 != password2) {
      createLabel("The two passwords must be the same!", spot);
      isOkay = false;
    } else if (password1.isEmpty) {
      createLabel("The password cannot be empty!", spot);
      isOkay = false;
    } else if (password1.contains(RegExp(r'\s'))) {
      createLabel("The password cannot contain any white characters!", spot);
      isOkay = false;
    } else {
      removeLabel(spot);
    }
    return isOkay;
  }

  /// @brief This function verifies if an user with this email can sign up.
  /// @param email -> The email.
  /// @return A boolean which indicate if the email is valid.
  bool okayEmailCreation(String email) {
    int spot = 5;
    bool isOkay = true;
    RegExp exp = RegExp(r"^[\w.+-]+@\w+\.\w{2,}$", caseSensitive: false);
    sendRequest("get", path: "users/search/", urlMap: {"email": email})
        .then((response) {
      if (response.body != "[]") {
        createLabel("An account is already linked to this email", spot);
        isOkay = false;
      } else if (exp.hasMatch(email)) {
        removeLabel(spot);
      } else {
        createLabel("The email is invalid", spot);
        isOkay = false;
      }
    });
    return isOkay;
  }

  /// @brief This function verifies if an user with this birth date can sign up (13 years old at least).
  /// @param birth -> The birth date in a dd-MM-YYYY format.
  /// @return A boolean which indicate if the birth date is valid.
  bool okayBirthCreation(String birth) {
    int spot = 7;
    try {
      DateTime birthdate = DateFormat('dd-MM-yyyy').parse(birth);
      DateTime now = DateTime.now();
      var boundaryDate = DateTime(now.year - 13, now.month, now.day);
      if (birthdate.isAfter(boundaryDate)) {
        createLabel("You must be at least 13 years old to sign up!", spot);
        return false;
      } else {
        removeLabel(spot);
        return true;
      }
    } catch (exception) {
      createLabel("Please choose a date", spot);
      return false;
    }
  }
}

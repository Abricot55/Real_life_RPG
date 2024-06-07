import 'dart:collection';
import 'dart:convert';
import 'dart:ui';

import 'package:english_words/english_words.dart';
import 'package:flutter/material.dart';
import 'package:main_application/profilePage.dart';
import 'package:provider/provider.dart';
import 'package:http/http.dart' as http;
import 'signUpPage.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => MyAppState(),
      child: MaterialApp(
        title: 'Namer App',
        theme: ThemeData(
          useMaterial3: true,
          colorScheme: ColorScheme.fromSeed(
              seedColor: Color.fromARGB(255, 92, 252, 255)),
        ),
        home: MyHomePage(),
      ),
    );
  }
}

class MyAppState extends ChangeNotifier {
  var current = WordPair.random();

  void getNext() {
    current = WordPair.random();
    notifyListeners();
  }
}

/**
 * @brief This class represent the home page of the application. It have a sign up button.
 */
class MyHomePage extends StatelessWidget {
  /**
   * @brief This function build all the widgets the user will see on the screen when the home page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    var pseudoController = TextEditingController();
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
              width: MediaQuery.of(context).size.width * 0.8,
              height: 50.0,
              decoration:
                  BoxDecoration(border: Border.all(color: Colors.black),
                  borderRadius: BorderRadius.circular(10.0)),
              child: TextField(
                //textAlignVertical: TextAlignVertical.center,
                autocorrect: false,
                controller: pseudoController,
                decoration: new InputDecoration(
                  border: InputBorder.none,
                  contentPadding: EdgeInsets.all(10.0),
                  hintText: 'ex : username@gmail.com',
                ),
                onSubmitted: (value) {
                  connexionTest(context, value);
                },
              ),
            ),
            ElevatedButton(
                onPressed: () {
                  connexionTest(context, pseudoController.text);
                },
                child: Text("Connexion")),
            ElevatedButton(
                onPressed: () {
                  navigateToNextScreen(context, 0);
                },
                child: Text("Sign Up")),

            /*This container is the TexField for the server request ***(FOR TEST PURPOSE)****/
            Container(
              width: MediaQuery.of(context).size.width * 0.3,
              height: MediaQuery.of(context).size.height * 0.1,
              decoration:
                  BoxDecoration(border: Border.all(color: Colors.black)),
              child: TextField(
                decoration: InputDecoration(border: InputBorder.none),
                onSubmitted: (value) {
                  newRequest(value);
                },
              ),
            )
          ],
        ),
      ),
    );
  }
}

/**
 * @brief This function check if a user exist in the database by using it pseudo. If it finds the users, it goes to its profile page.
 * @param context -> The context in which this function is called.
 * @param pseudo -> The pseudo of the user.
 * @return A boolean which represent the existence of the user.
 */
bool connexionTest(BuildContext context, String pseudo) {
  if (pseudo.isNotEmpty) {
    if (pseudo == "test") {
      navigateToNextScreen(context, 2, data: null);
      return true;
    } else {
      newRequest("get/document/${pseudo}/Users/MainDB/").then((value) {
        if (value.statusCode == 200) {
          navigateToNextScreen(context, 2, data: value);
          return true;
        }
        return false;
      });
    }
  }
  return false;
}

/**
 * @brief Cette fonction envoie une requête http au serveur
 * @param path -> le chemin envoyé dans la fonction, représente quelle requête est pa?ssée au serveur
 * @return La réponse du serveur.
 */
Future<dynamic> sendRequest(String function,
    {String path = "",
    HashMap<String, String>? urlMap,
    String jsonBody = ""}) async {
  var url = Uri.http('127.0.0.1:3000', path, {"name" : "a"});
  print(url);
  var response;
  switch (function.toUpperCase()) {
    case "GET":
      response = await http.get(url);
      break;
    case "ADD":
      response = await http.post(url,
          headers: {'Content-Type': 'application/json'}, body: jsonBody);
      break;
    case "POST":
      response = await http.post(url);
      break;
    case "UPDATE":
      response = await http.put(url);
      break;
    case "PUT":
      response = await http.put(url);
      break;
    case "DELETE":
      response = await http.delete(url);
      break;
  }
  if (response.statusCode == 200) {
    // Si le serveur retourne une réponse OK, alors parsez le JSON.
    var body = response.body;
    print(body);
  } else {
    print(response.statusCode);
  }
  return response;
}

/**
 * @brief This function take a basic String and transform it into a correct request for the server. Then it send the request to the sendRequest function.
 * @param path -> The basic String.
 */
Future<dynamic> newRequest(String path) {
  var word = "";
  for (int i = 0; i < path.length; i++) {
    if (path[i] != "/") {
      word += path[i];
    } else {
      path = path.substring(i + 1);
      break;
    }
  }
  return sendRequest(word, path: path);
}

/**
 * @brief This function change the dispalyed screen according to the number passed as an argument.
 * @param context -> The context in which this function is called.
 * @param screenNumber -> The number representing the scene you want to display.
 */
void navigateToNextScreen(BuildContext context, int screenNumber,
    {dynamic data}) {
  switch (screenNumber) {
    case 0:
      Navigator.of(context)
          .push(MaterialPageRoute(builder: (context) => SignUpPage()));
      break;
    case 1:
      Navigator.of(context)
          .push(MaterialPageRoute(builder: (context) => MyHomePage()));
      break;
    case 2:
      print("hourra");
      Navigator.of(context)
          .push(MaterialPageRoute(builder: (context) => ProfilePage()));
      break;
  }
}

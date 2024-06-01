import 'dart:ui';

import 'package:english_words/english_words.dart';
import 'package:flutter/material.dart';
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
          colorScheme: ColorScheme.fromSeed(seedColor: Color.fromARGB(255, 92, 252, 255)),
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
   * @brief This function build all the user will see on the screen when the home page is loaded. This function is automatically called.
   * @param context -> The context in which the home page is created.
   * @return The widget which is all the stuff on screen.
   */
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            ElevatedButton(onPressed: (){navigateToNextScreen(context, 0);}, child: Text("Sign Up")),

            /*This container is the TexField for the server request ***(FOR TEST PURPOSE)****/
            Container(
              width : MediaQuery.of(context).size.width * 0.3,
              height: MediaQuery.of(context).size.height * 0.1,
              decoration: BoxDecoration(border: Border.all(color : Colors.black)),
              child:
                TextField(
                decoration: InputDecoration(border: InputBorder.none),
                onSubmitted: (value){
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
 * @brief Cette fonction envoie une requête http au serveur
 * @param path -> le chemin envoyé dans la fonction, représente quelle requête est passée au serveur
 * @return La réponse du serveur.
 */
Future<void> sendRequest(String function ,String path) async {
  print(path);
  var url = Uri.http('127.0.0.1:3000', path);
  var response;
  switch (function.toUpperCase()) {
    case "GET" : response = await http.get(url); break;
    case "ADD" : response = await http.post(url); break;
    case "POST" : response = await http.post(url); break;
    case "UPDATE" : response = await http.put(url); break;
    case "PUT" : response = await http.put(url); break;
    case "DELETE" : response = await http.delete(url); break;
  }
  if (response.statusCode == 200) {
    print('Response body: ${response.body}');
  } else {
    print('Request failed with status: ${response.statusCode}.');
  }
}

/**
 * @brief This function take a basic String and transform it into a correct request for the server. Then it send the request to the sendRequest function.
 * @param path -> The basic String.
 */
void newRequest(String path){
  var word = "";
  for( int i = 0; i < path.length; i++){
    if (path[i] != "/"){
      word += path[i];
    }
    else{
      path = path.substring(i+1);
      break;}
  }
  sendRequest(word, path);
}

/**
 * @brief This function change the dispalyed screen according to the number passed as an argument.
 * @param context -> The context in which this function is called.
 * @param screenNumber -> The number representing the scene you want to display.
 */
void navigateToNextScreen(BuildContext context, int screenNumber) {
  switch (screenNumber){
    case 0 : Navigator.of(context).push(MaterialPageRoute(builder: (context) => SignUpPage()));break;
    case 1 : Navigator.of(context).push(MaterialPageRoute(builder: (context) => MyHomePage()));break;
  }
}
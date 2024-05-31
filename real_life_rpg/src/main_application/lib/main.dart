import 'dart:ui';

import 'package:english_words/english_words.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:http/http.dart' as http;

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

class MyHomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();
    var pair = appState.current;

    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            BigCard(pair: pair),
            ElevatedButton(
              onPressed: () {
                appState.getNext();
              },
              child: Text('Next'),
            ),
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

class BigCard extends StatelessWidget {
  const BigCard({
    super.key,
    required this.pair,
  });

  final WordPair pair;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.displayMedium!.copyWith(
      color: theme.colorScheme.onPrimary,
      //letterSpacing: 10.0,
    );

    return Card(
      color: theme.colorScheme.primary,
      //elevation: 10.0,
      child: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Text(
          pair.asLowerCase,
          style: style,
          semanticsLabel: "${pair.first} ${pair.second}",
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

void newRequest(String path){
  var word = "";
  for( int i = 0; i < path.length; i++){
    if (path[i] != "/"){
      word += path[i];
    }
    else{
      path = path.substring(i-1);
      break;}
  }
  sendRequest(word, path);
}
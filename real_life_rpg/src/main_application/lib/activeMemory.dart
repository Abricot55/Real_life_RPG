import 'User.dart';

class Activememory {
  User _memoryUser;
  List<User> _stack = [];
  int _stackLength = 0;

  Activememory(this._memoryUser){}

  User getMainUser(){
    return _memoryUser;
  }

  void push(User aUser){
    if (_stackLength < _stack.length) {
      _stack[_stackLength] = aUser;
    } else {
      _stack.add(aUser);
    }
    _stackLength += 1;
  }

  User pop(){
    _stackLength -= 1;
    return _stack[_stackLength];
  }

  void clearStack(){
    _stackLength = 0;
    _stack.clear();
  }

  bool isStackEmpty(){
    return _stackLength == 0;
  }
}
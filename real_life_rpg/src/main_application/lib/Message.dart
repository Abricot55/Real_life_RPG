import 'User.dart';

enum MessageState {sending, sent, seen}

class Message {
  DateTime date;
  String text;
  User isSentFrom;
  User isSentTo;
  MessageState state = MessageState.sending;

  Message(this.date, this.isSentFrom, this.isSentTo, this.text) {
    //this.date = date.toUtc();
  }

  bool operator <(Message other) {
    return this.date.isBefore(other.date);
  }

  void updateState(MessageState val){
    this.state = val;
  }

}

enum MessageState {sending, sent, seen}

class Message {
  DateTime date;
  String text;
  String idSentFrom;
  String idSentTo;
  MessageState state = MessageState.sending;

  Message(this.date, this.idSentFrom, this.idSentTo, this.text) {
    //this.date = date.toUtc();
  }

  bool operator <(Message other) {
    return this.date.isBefore(other.date);
  }

  void updateState(MessageState val){
    this.state = val;
  }

}

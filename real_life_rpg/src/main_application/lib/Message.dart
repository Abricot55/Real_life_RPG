class Message {
  DateTime date;
  String text;
  String idSentFrom;
  String idSentTo;
  bool isSent = false;
  bool isSeen = false;

  Message(this.date, this.idSentFrom, this.idSentTo, this.text) {
    //this.date = date.toUtc();
  }

  bool operator <(Message other) {
    return this.date.isBefore(other.date);
  }

  void setSentMessage(bool val){
    this.isSent = val;
  }

  void setSeeMessage(bool val){
    this.isSeen = val;
  }

}

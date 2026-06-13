import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';

class DocumentEvents 
{
  final _events = StreamController<DocEvent>.broadcast();
  Stream<DocEvent> get events => _events.stream;
  
  void documentSaved(Document doc)
  {
    _events.add(DocSavedEvent(doc));
  }
  void documentSelected(Document doc)
  {
    _events.add(DocSelectedEvent(doc));
  }
  Stream<DocSavedEvent> get docSavedEvents => _events.stream.where((event) => event is DocSavedEvent).cast<DocSavedEvent>();
  Stream<DocSelectedEvent> get docSelectedEvents => _events.stream.where((event) => event is DocSelectedEvent).cast<DocSelectedEvent>();
}


sealed class DocEvent 
{
  const DocEvent();
}
class DocUpdatedEvent extends DocEvent 
{
  final Document doc;
  const DocUpdatedEvent(this.doc);
}
class DocSavedEvent extends DocEvent 
{
  final Document doc;
  const DocSavedEvent(this.doc);
}
class DocSelectedEvent extends DocEvent 
{
  final Document doc;
  const DocSelectedEvent(this.doc);
}
import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/events/event.dart';

class DocumentEvents extends Event<DocEvent>
{
  late final Stream<RequestDocumentsForDateEvent> requestDocumentsForDateEvent = getStream();

  late final Stream<DocSavedEvent> docSavedEvent = getStream();

  late final Stream<DocSelectedEvent> docSelectedEvent = getStream();
  
  void documentSaved(Document doc)
  {
    push(DocSavedEvent(doc));
  }
  void documentSelected(Document doc)
  {
    push(DocSelectedEvent(doc));
  }
  void requestDocumentsForDate(DateTime date)
  {
    push(RequestDocumentsForDateEvent(date));
  }
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
class RequestDocumentsForDateEvent extends DocEvent 
{
  final DateTime date;
  const RequestDocumentsForDateEvent(this.date);
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
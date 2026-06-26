import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/documents_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:api_client/src/services/error_service.dart';
import 'package:intl/intl.dart';
import 'package:rinf/rinf.dart';

class DocumentsService 
{
  final DocumentsListProvider provider = DocumentsListProvider();
  final ErrorService _errorService;
  final EventBus _eventBus;
  late final StreamSubscription _sub;
  late final StreamSubscription _onDocumentsRequestEventSubscription;
  DocumentsService({required this._eventBus, required this._errorService}) 
  {
    _sub = DocumentPublicationDateResponse.rustSignalStream.listen((pack) => _onResponse(pack), onError: (error) 
    {
      provider.setLoading(false);
    });
    _onDocumentsRequestEventSubscription = _eventBus.documentEvents.requestDocumentsForDateEvent.listen(_onDocumentsRequestEvent);
  }

  void _onDocumentsRequestEvent(RequestDocumentsForDateEvent event)
  {
    getDocumentsForDate(event.date);
  }
  
  void getDocumentsForDate(DateTime date) 
  {
    provider.setLoading(true);
    final formatter = DateFormat("yyyy-MM-dd");
    DocumentPublicationDateRequest(
      publicationDate: formatter.format(date),
    ).sendSignalToRust();
  }
  
  void saveDocument(Document doc) 
  {
    provider.upsert(doc);
    UpdateDocumentRequest(document: doc).sendSignalToRust();
    _eventBus.documentEvents.documentSaved(doc);   // эмитим после успешной отправки
  }
  
  void selectDocument(Document doc) 
  {
    provider.select(doc.docId);
    _eventBus.documentEvents.documentSelected(doc);
  }
  //Stream<DocSavedEvent> get docSavedEvents => _events.documentEvents.docSavedEvents;
  //Stream<DocSelectedEvent> get docSelectedEvents => _events.documentEvents.docSelectedEvents;
  
  void _onResponse(RustSignalPack<DocumentPublicationDateResponse> pack) 
  {
    var date = DateTime.tryParse(pack.message.selectedDate);
    if (date == null)
    {
      _errorService.spawnError('Ошибка формата даты: ${pack.message.selectedDate}');
    }
    else
    {
      provider.setData(pack.message.documents, date);
    }
  }
  
  Future<void> dispose() async 
  {
    await _sub.cancel();
    provider.dispose();
    _onDocumentsRequestEventSubscription.cancel();
  }
}
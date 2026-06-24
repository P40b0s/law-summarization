import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/documents_provider.dart';
import 'package:api_client/src/services/error_service.dart';
import 'package:intl/intl.dart';
import 'package:rinf/rinf.dart';

class DocumentsService 
{
  final DocumentsListProvider provider = DocumentsListProvider();
  final ErrorService errorService;
  final DocumentEvents _events = DocumentEvents();
  late final StreamSubscription _sub;
  
  DocumentsService({required this.errorService}) 
  {
    _sub = DocumentPublicationDateResponse.rustSignalStream.listen((pack) => _onResponse(pack), onError: (error) 
    {
      //errorService.spawnError('Ошибка ответа');
      provider.setLoading(false);
    });
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
    //DocumentSaveRequest(document: doc).sendSignalToRust();
    _events.documentSaved(doc);   // эмитим после успешной отправки
  }
  
  void selectDocument(Document doc) 
  {
    provider.select(doc.docId);
    _events.documentSelected(doc);
  }
  Stream<DocSavedEvent> get docSavedEvents => _events.docSavedEvents;
  Stream<DocSelectedEvent> get docSelectedEvents => _events.docSelectedEvents;
  
  void _onResponse(RustSignalPack<DocumentPublicationDateResponse> pack) 
  {
    var date = DateTime.tryParse(pack.message.selectedDate);
    if (date == null)
    {
      errorService.spawnError('Ошибка формата даты: ${pack.message.selectedDate}');
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
  }
}
import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/events/calendar_events.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/image_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:api_client/src/services/error_service.dart';
import 'package:rinf/rinf.dart';

class ImageViewerService 
{
  final ErrorService _errorService;
  final EventBus _eventBus;
  final ImageProvider provider = ImageProvider();
  late final StreamSubscription _sub;
  late final StreamSubscription _documentSelectedEventSub;
  late final StreamSubscription _dateSelectedEventSub;
  ImageViewerService({required this._eventBus, required this._errorService}) 
  {
    _sub = PageResponse.rustSignalStream.listen((pack) => _onResponse(pack));
    _documentSelectedEventSub = _eventBus.documentEvents.docSelectedEvent.listen(_onDocumentSelectedEvent);
    _dateSelectedEventSub = _eventBus.calendarEvents.selectDateEvent.listen(_onDateSelectedEvent);
  }
  
  void _onResponse(RustSignalPack<PageResponse> signal) 
  {
    int pageNum = signal.message.pageNumber;
    provider.changePage(pageNum, signal.binary);
  }
  void _onDocumentSelectedEvent(DocSelectedEvent event)
  {
    provider.setDocument(event.doc.docId, 1, event.doc.pagesCount);
    requestPage(1);
  }
  void _onDateSelectedEvent(_)
  {
    provider.noDocument();
  }

  void requestPage(int pageNum)
  {
    provider.requestPageState(pageNum);
    PageRequest(id: provider.docId, pageNumber: pageNum).sendSignalToRust();
  }

  void nextPage() 
  {
    if (provider.currentPage < provider.maxPage && !provider.isPageLoading) 
    {
      requestPage(provider.currentPage + 1);
    } 
    else 
    {
      _errorService.spawnError('Это последняя страница', severity: ErrorSeverity.info);
    }
  }

  void previousPage() 
  {
    if (provider.currentPage > 1 && !provider.isPageLoading) 
    {
      requestPage(provider.currentPage - 1);
    } 
    else 
    {
      _errorService.spawnError('Это последняя страница', severity: ErrorSeverity.info);
    }
  }
  
  Future<void> dispose() async 
  {
    await _sub.cancel();
    _documentSelectedEventSub.cancel();
    provider.dispose();
  }
}
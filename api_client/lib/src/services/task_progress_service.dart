import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/providers/task_progress_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:rinf/rinf.dart';

class TaskProgressService 
{
  final DocumentsProgressProvider doc_provider = DocumentsProgressProvider();
  final PagesProgressProvider page_provider = PagesProgressProvider();
  late final StreamSubscription _pages_sub;
  late final StreamSubscription _docs_sub;
  final EventBus _eventBus;
  
  TaskProgressService({required this._eventBus}) 
  {
    _docs_sub = ServiceDocumentsProgress.rustSignalStream.listen((pack) => _onDocumentProgressResponse(pack));
    _pages_sub = ServicePagesProgress.rustSignalStream.listen((pack) => _onPageProgressResponse(pack));
  }

  void _onPageProgressResponse(RustSignalPack<ServicePagesProgress> pack) 
  {
    page_provider.changeState(pack.message.count, pack.message.progress);
  }
  void _onDocumentProgressResponse(RustSignalPack<ServiceDocumentsProgress> pack) 
  {
    doc_provider.changeState(pack.message.count, pack.message.progress);
  }
  
  Future<void> dispose() async 
  {
    await _docs_sub.cancel();
    await _pages_sub.cancel();
    doc_provider.dispose();
    page_provider.dispose();
  }
}
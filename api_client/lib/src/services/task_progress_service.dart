import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/providers/task_progress_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:rinf/rinf.dart';

class TaskProgressService 
{
  final DocumentsProgressProvider docsProvider = DocumentsProgressProvider();
  final PagesProgressProvider pagesProvider = PagesProgressProvider();
  late final StreamSubscription _pagesSub;
  late final StreamSubscription _docsSub;
  final EventBus _eventBus;
  
  TaskProgressService({required this._eventBus}) 
  {
    _docsSub = ServiceDocumentsProgress.rustSignalStream.listen((pack) => _onDocumentProgressResponse(pack));
    _pagesSub = ServicePagesProgress.rustSignalStream.listen((pack) => _onPageProgressResponse(pack));
   Future.delayed(Duration(seconds: 5), () =>
   {
      pagesProvider.changeState(73, 12),
      docsProvider.changeState(12, 4)
   });
   Future.delayed(Duration(seconds: 15), () =>
   {
      pagesProvider.changeState(73, 59),
      docsProvider.changeState(12, 8)
   });
   Future.delayed(Duration(seconds: 25), () =>
   {
      pagesProvider.changeState(73, 73),
      docsProvider.changeState(12, 12)
   });
   Future.delayed(Duration(seconds: 30), () =>
   {
      checkClean()
   });
  }

  void _onPageProgressResponse(RustSignalPack<ServicePagesProgress> pack) 
  {
    pagesProvider.changeState(pack.message.count, pack.message.progress);
    checkClean();
  }
  void _onDocumentProgressResponse(RustSignalPack<ServiceDocumentsProgress> pack) 
  {
    docsProvider.changeState(pack.message.count, pack.message.progress);
    checkClean();
  }

  void checkClean()
  {
    if((pagesProvider.count == pagesProvider.progress && pagesProvider.count != 0)
    && (docsProvider.count == docsProvider.progress && docsProvider.count != 0)
    )
    {
      Future.delayed(Duration(seconds: 5), () =>
      {
        pagesProvider.resetProgress(),
        docsProvider.resetProgress()
      });
    }
  }
  
  Future<void> dispose() async 
  {
    await _docsSub.cancel();
    await _pagesSub.cancel();
    docsProvider.dispose();
    pagesProvider.dispose();
  }
}
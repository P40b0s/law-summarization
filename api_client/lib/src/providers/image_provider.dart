import 'dart:collection';
import 'dart:typed_data';
import 'package:flutter/material.dart';

class ImageProvider extends ChangeNotifier
{
  late int _currentPage;
  late int _maxPage;
  late String _docId;
  bool _isPageLoading = true;
  Uint8List? _currentImage;
  Key _imageKey = const ValueKey<int>(1);
  String get docId => _docId;
  int get maxPage => _maxPage;
  int get currentPage => _currentPage;
  bool get isPageLoading => _isPageLoading;
  Uint8List? get currentImage => _currentImage;
  Key get imageKey => _imageKey;
  HashMap<int, Uint8List> _pages = HashMap();
  ImageProvider();
  void _genKey(int pageNumber)
  {
     _imageKey = ValueKey<String>(_docId + pageNumber.toString());
  }
  void setDocument(String docId, int initialPage, int maxPage)
  {
    _docId = docId;
    _currentPage = initialPage;
    _maxPage = maxPage;
    _pages = HashMap();
    requestPageState(initialPage);
  }
  void noDocument()
  {
    _docId = "";
    _currentPage = 0;
    _maxPage = 0;
    _currentImage = null;
    _pages = HashMap();
    notifyListeners();
  }
  bool pageExistsInCache(int page)
  {
    return _pages.containsKey(page);
  }
  void changePageFromCache(int page)
  {
    _currentImage = _pages[page];
    _currentPage = page;
    _genKey(page);
    notifyListeners();
  }
  ///оповещение о изменении страницы
  void changePage(int page, Uint8List data)
  {
    _currentImage = data;
    _isPageLoading = false;
    _currentPage = page;
    _pages[page] = data;
    _genKey(page);
    notifyListeners();
  }
  ///Запрос изображения страницы
  void requestPageState(int pageNumber)
  {
    _isPageLoading = true;
    _genKey(pageNumber);
    notifyListeners();
  }
}
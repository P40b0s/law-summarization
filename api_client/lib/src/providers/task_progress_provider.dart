import 'package:flutter/material.dart';

class DocumentsProgressProvider extends ChangeNotifier
{
  int _count = 0;
  int _progress = 0;
  int get count => _count;
  int get progress => _progress;
  DocumentsProgressProvider();

  void changeState(int count, int progress)
  {
    _count = count;
    _progress = progress;
    notifyListeners();
  }
}

class PagesProgressProvider extends ChangeNotifier
{
  int _count = 0;
  int _progress = 0;
  int get count => _count;
  int get progress => _progress;
  PagesProgressProvider();

  void changeState(int count, int progress)
  {
    _count = count;
    _progress = progress;
    notifyListeners();
  }
}
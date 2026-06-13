import 'package:flutter/material.dart';

class ErrorProvider extends ChangeNotifier
{
  String? _error;
  String? get error => _error;
  bool _isShow = false;
  bool get isShow => _isShow;
  ErrorProvider();

  void spawnError(String error)
  {
    _error = error;
    _isShow = true;
    notifyListeners();
  }
  void close()
  {
    _error = null;
    _isShow = false;
    notifyListeners();
  }
}
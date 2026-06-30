import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class HealthProvider extends ChangeNotifier
{
  bool _alive = false;
  bool get alive => _alive;
  bool _isBusy = false;
  bool get isBusy => _isBusy;
  DateTime? _date;
  String? _formattedDate;
  String? get formattedDate => _formattedDate;
  DateTime? get date => _date;
  HealthProvider();

  void changeState(bool state, bool busy)
  {
    _alive = state;
    _isBusy = busy;
    _date = DateTime.now();
    _formattedDate = DateFormat("HH:mm:ss").format(_date!);
    notifyListeners();
  }
}
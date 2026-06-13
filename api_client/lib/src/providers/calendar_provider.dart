import 'dart:collection';

import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/providers/error_provider.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class CalendarProvider extends ChangeNotifier
{

  final HashMap<String, int> _checkedDates = HashMap();
  final HashMap<String, int> _unloadedDates = HashMap();
  final HashMap<String, int> _countDates = HashMap();
  final DateTime _minDate = DateTime.now().subtract(const Duration(days: 35));
  final int _requestDurationMin = 5;
  final formatter = DateFormat("yyyy-MM-dd");
  DateTime get minDate => _minDate;
  int? checked(DateTime date) =>  _checkedDates[formatter.format(date)];
  int? unloaded(DateTime date) =>  _unloadedDates[formatter.format(date)];
  int? count(DateTime date) =>  _countDates[formatter.format(date)];
  String keyString(DateTime date) => '${formatter.format(date)}_${checked(date)}_${unloaded(date)}_${count(date)}';
  int get requestDurationMin => _requestDurationMin;
  CalendarProvider();

  void updateDates(Iterable<MapEntry<String, DateState>> dates)
  {
    for (var date in dates)
    {
      var checked = date.value.checked;
      var unloaded = date.value.unloaded;
      var count = date.value.count;
      _checkedDates[date.key] = checked;
      _unloadedDates[date.key] = unloaded;
      _countDates[date.key] = count;
    }
    notifyListeners();
  }

}
import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/providers/calendar_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:rinf/rinf.dart';

class CalendarService 
{
  final CalendarProvider provider = CalendarProvider();
  late final StreamSubscription _sub;
  final EventBus _eventBus;
  late final StreamSubscription _nextDateSelectEventSub;
  late final StreamSubscription _prevDateSelectEventSub;
  bool _periodicTaskIsRunning = true;
  
  CalendarService({required this._eventBus}) 
  {
    _sub = CalendarResponse.rustSignalStream.listen((pack) => _onResponse(pack));
    _nextDateSelectEventSub = _eventBus.calendarEvents.nextDateEvent.listen(_onNextDateEvent);
    _prevDateSelectEventSub = _eventBus.calendarEvents.previousDateEvent.listen(_onPrevDateEvent);
    _startPeriodicTask();
  }

  Future<void> _startPeriodicTask() async 
  {
    while (_periodicTaskIsRunning) 
    {
      CalendarRequest(from: provider.formatter.format(provider.minDate)).sendSignalToRust();
      await Future.delayed(Duration(minutes: provider.requestDurationMin));
    }
  }

  void _onNextDateEvent(_)
  {
    var curDate = provider.selectedDate ?? DateTime.now();
    var nextDate = curDate.add(Duration(days: 1));
    selectDate(nextDate);
  }
  void _onPrevDateEvent(_)
  {
    var curDate = provider.selectedDate ?? DateTime.now();
    var prevDate = curDate.subtract(Duration(days: 1));
    selectDate(prevDate);
  }
  void nexDate()
  {

  }
  void previousDate()
  {

  }

  void _stopPeriodicTask() 
  {
    _periodicTaskIsRunning = false; // Safely breaks the loop on the next cycle
  }
  
  
  void _onResponse(RustSignalPack<CalendarResponse> pack) 
  {
    provider.updateDates(pack.message.dates.entries);
    if (!provider.anyIsSelected)
    {
      var date = DateTime.now();
      selectDate(date);
    }
  }
  ///Выбирает указанную дату в календаре и получает за нее список документов
  void selectDate(DateTime date)
  {
    provider.selectDate(date);
    _eventBus.documentEvents.requestDocumentsForDate(date);
  }
  
  Future<void> dispose() async 
  {
    await _sub.cancel();
    _nextDateSelectEventSub.cancel();
    _prevDateSelectEventSub.cancel();
    provider.dispose();
  }
}
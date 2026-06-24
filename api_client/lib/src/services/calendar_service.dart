import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/calendar_provider.dart';
import 'package:api_client/src/providers/error_provider.dart';
import 'package:api_client/src/services/error_service.dart';
import 'package:rinf/rinf.dart';

class CalendarService 
{
  final ErrorService errorService;
  final CalendarProvider provider = CalendarProvider();
  late final StreamSubscription _sub;
  bool _periodicTaskIsRunning = true;
  
  CalendarService({required this.errorService}) 
  {
    _sub = CalendarResponse.rustSignalStream.listen((pack) => _onResponse(pack));
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

  void _stopPeriodicTask() 
  {
    _periodicTaskIsRunning = false; // Safely breaks the loop on the next cycle
  }
  
  
  void _onResponse(RustSignalPack<CalendarResponse> pack) 
  {
    provider.updateDates(pack.message.dates.entries);
  }
  
  Future<void> dispose() async 
  {
    await _sub.cancel();
    provider.dispose();
    errorService.dispose();
  }
}
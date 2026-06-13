import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/calendar_provider.dart';
import 'package:api_client/src/providers/error_provider.dart';
import 'package:rinf/rinf.dart';

class CalendarService 
{
  final ErrorProvider errorProvider = ErrorProvider();
  final CalendarProvider provider = CalendarProvider();
  late final StreamSubscription _sub;
  bool _periodicTaskIsRunning = true;
  
  CalendarService() 
  {
    _sub = CalendarResponse.rustSignalStream.listen((pack) => _onResponse(pack), onError: (error) 
    {
      errorProvider.spawnError('Ошибка ответа');
    });
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
    try 
    {
      provider.updateDates(pack.message.dates.entries);
      
    } 
    catch (_) 
    {
      errorProvider.spawnError('Ошибка ответа');
    } 
    finally 
    {
    }
  }
  
  Future<void> dispose() async 
  {
    await _sub.cancel();
    provider.dispose();
    errorProvider.dispose();
  }
}
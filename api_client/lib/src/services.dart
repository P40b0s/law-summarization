import 'package:api_client/src/providers/error_provider.dart';
import 'package:api_client/src/services/calendar_service.dart';
import 'package:api_client/src/services/documents_service.dart';
import 'package:api_client/src/services/error_service.dart';
import 'package:flutter/material.dart';

class RustSignals
{
  //Todo необходимо сделать вызовы rust сигналов 

}

class AppServices
{
  static late final AppServices I;
  late DocumentsService documentsService;
  late CalendarService calendarService;
  late ErrorService errorService;
  AppServices._()
  {
    errorService = ErrorService();
    documentsService = DocumentsService(errorService: errorService);
    calendarService = CalendarService(errorService: errorService);
  }
  static void init()
  {
    I = AppServices._();
  }

  Future<void> dispose() async 
  {
    await documentsService.dispose();
    await calendarService.dispose();
    await errorService.dispose();
  }
}

extension DocumentEventsX on BuildContext 
{
  AppServices get appServices => AppServices.I;
}

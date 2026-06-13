import 'package:api_client/src/providers/error_provider.dart';
import 'package:api_client/src/services/calendar_service.dart';
import 'package:api_client/src/services/documents_service.dart';
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
  late ErrorProvider errorProvider;
  AppServices._()
  {
    documentsService = DocumentsService();
    calendarService = CalendarService();
    errorProvider = ErrorProvider();
  }
  static void init()
  {
    I = AppServices._();
  }
}

extension DocumentEventsX on BuildContext 
{
  AppServices get appServices => AppServices.I;
}

import 'package:api_client/src/events/calendar_events.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/error_provider.dart';
import 'package:api_client/src/services/calendar_service.dart';
import 'package:api_client/src/services/documents_service.dart';
import 'package:api_client/src/services/error_service.dart';
import 'package:api_client/src/services/image_viewer_service.dart';
import 'package:flutter/material.dart';

class EventBus
{
  final DocumentEvents documentEvents = DocumentEvents();
  final CalendarEvents calendarEvents = CalendarEvents();
  //Todo необходимо сделать вызовы rust сигналов 
  EventBus();
}

class AppServices
{
  static late final AppServices I;
  final EventBus eventBus = EventBus();
  late DocumentsService documentsService;
  late CalendarService calendarService;
  late ImageViewerService imageViewerService;
  late ErrorService errorService;
  AppServices._()
  {
    errorService = ErrorService();
    documentsService = DocumentsService(eventBus: eventBus, errorService: errorService);
    calendarService = CalendarService(eventBus: eventBus);
    imageViewerService = ImageViewerService(eventBus: eventBus, errorService: errorService);
  }
  static void init()
  {
    I = AppServices._();
  }

  Future<void> dispose() async 
  {
    await documentsService.dispose();
    await calendarService.dispose();
    await imageViewerService.dispose();
    await errorService.dispose();
    
  }
}

extension DocumentEventsX on BuildContext 
{
  AppServices get appServices => AppServices.I;
}

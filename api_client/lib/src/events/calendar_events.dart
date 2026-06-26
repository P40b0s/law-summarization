import 'dart:async';
import 'package:api_client/src/events/event.dart';

class CalendarEvents extends Event<CalendarEvent>
{
  late final Stream<SelectDateEvent> selectDateEvent = getStream();
  late final Stream<NextDateEvent> nextDateEvent = getStream();
  late final Stream<PreviousDateEvent> previousDateEvent = getStream();
  void dateSelected(DateTime date)
  {
    push(SelectDateEvent(date));
  }
  void nextDate()
  {
    push(NextDateEvent());
  }
  void previousDate()
  {
    push(PreviousDateEvent());
  }
}


sealed class CalendarEvent 
{
  const CalendarEvent();
}
class SelectDateEvent extends CalendarEvent 
{
  final DateTime date;
  const SelectDateEvent(this.date);
}

class NextDateEvent extends CalendarEvent 
{
  const NextDateEvent();
}
class PreviousDateEvent extends CalendarEvent 
{
  const PreviousDateEvent();
}
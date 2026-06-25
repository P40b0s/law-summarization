import 'dart:async';
import 'dart:collection';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/services.dart';
import 'package:flutter/material.dart';
import 'package:intl/date_symbol_data_local.dart';
import 'package:intl/intl.dart';
import 'package:paged_vertical_calendar/paged_vertical_calendar.dart';

class Calendar extends StatefulWidget 
{
  const Calendar({super.key, });
  
  @override
  State<Calendar> createState() => _CalendarState();
}

class _CalendarState extends State<Calendar> 
{
  final d = initializeDateFormatting('ru_RU', null);

  @override
  void initState() 
  {
    super.initState();
  }

  @override
  Widget build(BuildContext context) 
  {
    return  Card(
      margin: const EdgeInsets.all(8.0),
      elevation: 2,
        child: PagedVerticalCalendar(
        minDate: context.appServices.calendarService.provider.minDate,
        maxDate: DateTime.now().add(Duration(days: 1)),
        invisibleMonthsThreshold: 1,
        startWeekWithSunday: false,
        onMonthLoaded: (year, month) {
          // on month widget load 
        },
        onDayPressed: (value) {
          // on day widget pressed   
        },
        onPaginationCompleted: (direction) {
          // on pagination completion
        },
        monthBuilder: (context, month, year) 
        {
          final monthName = DateFormat.MMMM('ru_RU').format(DateTime(year, month));
          return Container(
            color: Colors.grey[300],
            padding: const EdgeInsets.symmetric(vertical: 8.0),
            child: Center(
              child: Text(
                '$monthName $year',
                style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
              ),
            ),
          );
        },
        dayBuilder: (context, date) 
        {
          return RepaintBoundary(
            child:ListenableBuilder(
              //listenable: context.appServices.documentsService.provider,
              listenable: context.appServices.calendarService.provider,
              builder: (_, _)
              {
                return CalendarDayWidget(
                      date: date,
                      count: context.appServices.calendarService.provider.count(date),
                      checked: context.appServices.calendarService.provider.checked(date),
                      unloaded: context.appServices.calendarService.provider.unloaded(date),
                      selected: context.appServices.calendarService.provider.selected(date) ?? false,
                      onDateSelected: context.appServices.calendarService.provider.selectDate);
              })
          );
        },
      )
    );
  }
}


class CalendarDayWidget extends StatelessWidget 
{
  final DateTime date;
  final int? checked;
  final int? unloaded;
  final int? count;
  final bool selected;
  final Function(DateTime) onDateSelected;

  const CalendarDayWidget({
    super.key, 
    required this.date,
    required this.count,
    required this.checked, 
    required this.unloaded,
    required this.selected,
    required this.onDateSelected
  });


  @override
  Widget build(BuildContext context) 
  {
    final keyString = context.appServices.calendarService.provider.keyString(date);
    return Container(
      color: getToday(),
      child: InkWell(
        onTap: () => 
        {
          context.appServices.documentsService.getDocumentsForDate(date),
          onDateSelected(date)
        },
        child: Column(
          key: ValueKey(keyString),
          children: [
            Expanded(
              child: Center(
                child: Text(
                  date.day.toString(), 
                  style: const TextStyle(color: Colors.black, fontWeight: FontWeight.w500, fontSize: 16),
                ),
              ),
            ),
            if (checked != null || unloaded != null || count != null)
              Expanded(
                child: Tooltip(
                    message: "Всего/проверено/выгружено",
                    child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text((count ?? 0).toString(), style: const TextStyle(color: Color.fromARGB(255, 2, 2, 2), fontSize: 12)),
                      const Text("/", style: TextStyle(fontSize: 8)),
                      Text((checked ?? 0).toString(), style: const TextStyle(color: Colors.green, fontSize: 12)),
                      const Text("/", style: TextStyle(fontSize: 8)),
                      Text((unloaded ?? 0).toString(), style: const TextStyle(color: Colors.red, fontSize: 12)),
                    ],
                  )
                )
              )
          ],
        )
      )
    );
  }

  Color getToday()
  {
    final dateNow = DateTime.now();
    final monthNow = dateNow.month;
    final dayNow = dateNow.day;
    if (selected)
    {
      return const Color.fromARGB(92, 45, 198, 236);
    }
    else if (date.day == dayNow && monthNow == date.month)
    {
      return Colors.lightGreen;
    }
    else
    {
      return Colors.transparent;
    }
  }
}
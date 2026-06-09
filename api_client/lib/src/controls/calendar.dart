import 'dart:async';
import 'dart:collection';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:flutter/material.dart';
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
  bool _periodicTaskIsRunning = true;
  final HashMap<String, int> _readyDates = HashMap();
  final HashMap<String, int> _unreadyDates = HashMap();
  final DateTime _minDate = DateTime.now().subtract(const Duration(days: 35));
  final int _requestDurationMin = 5;
  late StreamSubscription _rustSubscription;
  final formatter = DateFormat("yyyy-MM-dd");
  @override
  void initState() 
  {
    super.initState();
    _rustSubscription = _listenToStream();
    _startPeriodicTask();
  }

 
  Future<void> _startPeriodicTask() async 
  {
    while (_periodicTaskIsRunning) 
    {
      CalendarRequest(from: formatter.format(_minDate)).sendSignalToRust();
      await Future.delayed(Duration(minutes: _requestDurationMin));
    }
  }

  void _stopPeriodicTask() 
  {
    _periodicTaskIsRunning = false; // Safely breaks the loop on the next cycle
  }

  StreamSubscription _listenToStream() 
  {
    return CalendarResponse.rustSignalStream.listen((signalPack) 
    {
      if (!mounted) return;
      var dates = signalPack.message.dates;
      updateAll(dates.entries);
    });
  }
  @override
  Future<void> dispose() async
  {
    await _rustSubscription.cancel();
    super.dispose();
  }

  void updateAll(Iterable<MapEntry<String, DateState>> dates)
  {
    setState(() 
      {
        for (var date in dates)
        {
          var ready = date.value.ready;
          var unready = date.value.unready;
          _unreadyDates[date.key] = unready;
          _readyDates[date.key] = ready;
        }
      });
  }

  @override
  Widget build(BuildContext context) 
  {

    return  Card(
      margin: const EdgeInsets.all(8.0),
      elevation: 2,
        child: PagedVerticalCalendar(
        minDate: _minDate,
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
        dayBuilder: (context, date) 
        {
          final formattedDate = formatter.format(date);
          final unprocessed = _unreadyDates[formattedDate];
          final processed = _readyDates[formattedDate];
          return RepaintBoundary(
            child: CalendarDayWidget(
              date: date,
              unprocessed: unprocessed,
              processed: processed,
              formatter: formatter,
              ),
          );
        },
      )
    );
  }
}


class CalendarDayWidget extends StatelessWidget 
{
  final DateTime date;
  final int? processed;
  final int? unprocessed;
  final DateFormat? formatter;

  const CalendarDayWidget({
    super.key, 
    required this.date,
    required this.formatter,
    required this.processed, 
    required this.unprocessed,
  });

  @override
  Widget build(BuildContext context) 
  {
    final keyString = '${formatter!.format(date)}_${processed ?? 0}_${unprocessed ?? 0}';
    return Container(
      color: getToday(),
      child: InkWell(
        onTap: () => DocumentPublicationDateRequest(publicationDate: formatter!.format(date)).sendSignalToRust(),
      
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
            if (processed != null || unprocessed != null)
              Expanded(
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text((processed ?? 0).toString(), style: const TextStyle(color: Colors.green, fontSize: 12)),
                    const Text("/", style: TextStyle(fontSize: 8)),
                    Text((unprocessed ?? 0).toString(), style: const TextStyle(color: Colors.red, fontSize: 12)),
                  ],
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
    if (date.day == dayNow && monthNow == date.month)
    {
      return Colors.lightGreen;
    }
    else
    {
      return Colors.transparent;
    }
  }
}
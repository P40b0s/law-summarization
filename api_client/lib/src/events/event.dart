import 'dart:async';

abstract class Event<E> 
{
  final events = StreamController<E>.broadcast();
  Stream<T> getStream<T>()
  {
    return events.stream.where((event) => event is T)
        .cast<T>()
        .asBroadcastStream();
  }
  void push(E event)
  {
    events.add(event);
  }

}
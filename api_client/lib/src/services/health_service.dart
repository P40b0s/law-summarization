import 'dart:async';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/providers/health_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:rinf/rinf.dart';

class HealthService 
{
  final HealthProvider provider = HealthProvider();
  late final StreamSubscription _sub;
  final EventBus _eventBus;
  
  HealthService({required this._eventBus}) 
  {
    _sub = ServiceHealth.rustSignalStream.listen((pack) => _onResponse(pack));
  }

  void _onResponse(RustSignalPack<ServiceHealth> pack) 
  {
    provider.changeState(pack.message.alive, pack.message.busy);
  }
  
  Future<void> dispose() async 
  {
    await _sub.cancel();
    provider.dispose();
  }
}
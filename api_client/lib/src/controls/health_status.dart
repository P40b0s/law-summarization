

import 'package:api_client/src/services.dart';
import 'package:flutter/material.dart';

class HealthStatus extends StatelessWidget 
{
  const HealthStatus({super.key});
  @override
  Widget build(BuildContext context) 
  {
     return ListenableBuilder(
      listenable: context.appServices.healthService.provider,
      builder: (_, _)
      {
        if(context.appServices.healthService.provider.date == null)
        {
          return Tooltip(
            message: "Проверка сервера в процессе...",
            child: Icon(Icons.signal_wifi_4_bar ,color: Colors.white60,)
            );
        }
        else if(context.appServices.healthService.provider.alive)
        {
          return Tooltip(
            message: "Сервер доступен на ${context.appServices.healthService.provider.formattedDate}",
            child: Icon(Icons.signal_wifi_4_bar ,color: Colors.lightGreen,)
            );
        }
        else
        {
          return Tooltip(
            message: "Сервер не доступен с ${context.appServices.healthService.provider.formattedDate}",
            child: Icon(Icons.signal_wifi_off ,color: Colors.redAccent,)
            );
        }
      }
     );       
  }
}
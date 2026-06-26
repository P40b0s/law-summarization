import 'package:api_client/src/controls/document_widget.dart';
import 'package:api_client/src/controls/image_viewer.dart';
import 'package:api_client/src/controls/left_panel.dart';
import 'package:api_client/src/controls/toast.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/error_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:api_client/src/themes.dart';
import 'package:provider/provider.dart';
import 'package:rinf/rinf.dart';
import 'src/bindings/bindings.dart';
import 'package:flutter/material.dart';

Future<void> main() async 
{
  AppServices.init();
  await initializeRust(assignRustSignal);
  runApp(const MyApp());
}

class MyApp extends StatelessWidget 
{
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) 
  {
    return MultiProvider(providers: [
      ChangeNotifierProvider.value(
            value: context.appServices.errorService.provider)
    ],
    child:  ValueListenableBuilder<ThemeMode>(
      valueListenable: themeNotifier,
      builder: (_, ThemeMode currentMode, _) {
        return MaterialApp(
      theme: AppTheme.light,
      darkTheme: AppTheme.dark,
      themeMode: currentMode,
      builder: (context, child) => ToastOverlay(child: child!),
      home: Scaffold(
        appBar: AppBar(title: const Text('Проверка документов сервиса суммаризации'), shape: Border.all( color: Colors.black, width: 2.0),
        actions: [
                    IconButton(
                      icon: Icon(currentMode.isDark ? Icons.wb_sunny : Icons.nightlight_round),
                      onPressed: () 
                      {
                        themeNotifier.value = currentMode.isDark ? ThemeMode.light : ThemeMode.dark;
                      },
                    ),
                    //Это второй способ использовать провайдера из error_service
                    //точно так же можно добавить других провайдеров и использовать их в виджетах
                    //а не делать привязку к провадеру внутри
                        Consumer<ErrorProvider>(
                builder: (context, errorProvider, _) => IconButton(
                  icon: Badge(
                    isLabelVisible: errorProvider.count > 0,
                    label: Text('${errorProvider.count}'),
                    child: const Icon(Icons.history),
                  ),
                  tooltip: 'Recent errors',
                  onPressed: () => RecentErrorsPanel.show(context),
                ),
              ),
          
        ],),
        
        body: Padding(padding: EdgeInsets.all(30),
          child: Row(
            textDirection: TextDirection.ltr,
            children: [
              SizedBox(width: 480,  child: Leftpanel()),
              SizedBox(width: 600, child: ImageViewer()),
              Expanded(child: StreamBuilder<DocSelectedEvent>(
                stream: context.appServices.eventBus.documentEvents.docSelectedEvent,
                builder: (_, snapshot)
                {
                  if(snapshot.hasData)
                  {
                    return DocumentWidget(key: Key(snapshot.data!.doc.docId), document:  snapshot.data!.doc,);
                  }
                  else
                  {
                    return SizedBox.shrink();
                  }
                  
                },))
            
            ]
          ),
        ) 
        )
      );
      }
    )
    );
    
  }
}
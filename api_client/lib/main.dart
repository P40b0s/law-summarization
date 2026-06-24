import 'package:api_client/src/controls/document_widget.dart';
import 'package:api_client/src/controls/error_snack.dart';
import 'package:api_client/src/controls/image_viewer.dart';
import 'package:api_client/src/controls/left_panel.dart';
import 'package:api_client/src/controls/toast.dart';
import 'package:api_client/src/events/documents_events.dart';
import 'package:api_client/src/providers/error_provider.dart';
import 'package:api_client/src/services.dart';
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
    child:  MaterialApp(
      builder: (context, child) => ToastOverlay(child: child!),
      home: Scaffold(
        appBar: AppBar(title: const Text('Проверка документов сервиса суммаризации'), shape: Border.all( color: Colors.black, width: 2.0),
        actions: [
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
              // SizedBox(width: 0, height: 0, child: 
              //  ChangeNotifierProvider(
              //     create: (_) => context.appServices.errorProvider,
              //     child: const SnackBarExample(),
              //  )),
              SizedBox(width: 480,  child: Leftpanel()),
              SizedBox(width: 600, 
                child: StreamBuilder<DocSelectedEvent>(
                  stream: context.appServices.documentsService.docSelectedEvents,
                  builder: (_, snapshot)
                  {
                    if(snapshot.hasData)
                    {
                      return ImageViewer(key: ValueKey(snapshot.data!.doc.docId), docId: snapshot.data!.doc.docId, initialPage: 1, maxPage: snapshot.data!.doc.pagesCount,);
                    }
                    else
                    {
                      return SizedBox.shrink();
                    }
                  }
                )
              ),
              //SizedBox(width: 600, child: ImageViewer(docId: "5133ba0c-1d95-42e5-822f-c10c691b467d", initialPage: 1, maxPage: 2,),),
              Expanded(child: StreamBuilder<DocSelectedEvent>(
                stream: context.appServices.documentsService.docSelectedEvents,
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
      )
    );
    
  }
}
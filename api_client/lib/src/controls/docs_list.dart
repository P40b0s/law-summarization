import 'package:api_client/src/bindings/bindings.dart';
import 'package:api_client/src/services.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class DocumentsList extends StatefulWidget 
{
  const DocumentsList({super.key, });
  
  @override
  State<DocumentsList> createState() => _DocumentsListState();
}

class _DocumentsListState extends State<DocumentsList> 
{
  final formatter = DateFormat("dd.MM.yyyy");
  final dateTimeFormatter = DateFormat("dd.MM.yyyy HH:mm");
  //late ValueNotifier<TitleNotifier> _title;
  @override
  void initState() 
  {
    super.initState();
    //_title = ValueNotifier(TitleNotifier(overall: 0, ready: 0, unloaded: 0));
  }

  @override
  void dispose()
  {
    //_title.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) 
  {
    return ListenableBuilder(
      listenable: context.appServices.documentsService.provider,
      builder: (_, _)
      {
        final documents = context.appServices.documentsService.provider.documents;
        if(context.appServices.documentsService.provider.selectedDate == null)
        {
          return SizedBox.shrink();
        }
        else
        {
          return Card(
            margin: const EdgeInsets.all(8.0),
            elevation: 2,
            child: Column(
            children: [
              _titleWidget(context),
              Divider(
            color: Theme.of(context).colorScheme.primary,     
            thickness: 2,          
            indent: 20,              
            endIndent: 20,          
            height: 4,
          ),
              Expanded(child: ListView.builder(
            itemCount: documents.length,
            padding: const EdgeInsets.only(left: 3, right: 3, bottom: 4),
            
            itemBuilder: (context, index) 
            {
              var document = documents[index];
                return Material( 
                  elevation: 2.0,
                  color: context.appServices.documentsService.provider.getIsSelected(document) ? 
                    Theme.of(context).colorScheme.secondaryContainer : 
                    Theme.of(context).colorScheme.surfaceContainerLow,
                  shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(4.0),
                      side: BorderSide(color:  Theme.of(context).colorScheme.tertiaryFixed, width: 1.0),
                    ),
                  child:  InkWell(
                    onTap: () => context.appServices.documentsService.selectDocument(document),
                    child:  ListTile(
                      key: ValueKey(document.docId),
                      title: Row(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                             Text(document.eoNumber),
                             Row(
                              children: [
                                _checkedIcon(document),
                                //SizedBox(width: 8,),
                                //_unloadedIcon(document),
                              ],
                             )
                        ],
                      ),
                   
                      subtitle: Column(
                      crossAxisAlignment: CrossAxisAlignment.start, // Выравнивание текста по левому краю
                      mainAxisSize: MainAxisSize.min,   
                      children: 
                      [

                          Text(document.complexName,
                            softWrap: true, // Включает перенос слов (true по умолчанию)
                            maxLines: 5,    // Максимальное количество строк (опционально)
                            overflow: TextOverflow.ellipsis),
                      ]),
                    
                    )
                  )
                );
              //});
            },
          )
          ),
            ],
          ),
          );
        }
      }
      );
  }

  Widget _checkedIcon(Document doc)
  {
    if(doc.checkedTime != null)
    {
      return Tooltip(
        message: 'Проверено ${dateTimeFormatter.format(DateTime.parse(doc.checkedTime!))}',
        child: Icon(Icons.check_circle, color: Colors.green,),
      );
    }
    else
    {
      return Tooltip(
        message: 'Не проверено',
        child: Icon(Icons.check_circle_outline, color: Colors.grey,),
      );
    }
  }
  Widget _unloadedIcon(Document doc)
  {
    if(doc.unloaded)
    {
      return Tooltip(
        message: 'Выгружено',
        child: Icon(Icons.upload, color: Colors.green,),
      );
    }
    else
    {
      return Tooltip(
        message: 'Не выгружено',
        child: Icon(Icons.upload_outlined, color: Colors.grey,),
      );
    }
  }

  Widget _titleWidget(BuildContext context) 
  {
    final provider = context.appServices.documentsService.provider;
    final selectedDate = provider.selectedDate;
    final unloaded = provider.title.unloaded;
    final overall = provider.title.count;
    final checked = provider.title.ready;
    if (selectedDate != null)
    {
      var date = formatter.format(selectedDate);
      return Row(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        mainAxisSize: MainAxisSize.min,
        children: [
          IconButton(onPressed: () => context.appServices.eventBus.calendarEvents.previousDate(), icon: const Icon(Icons.arrow_back_ios)),
          Expanded( 
            child: ListTile(
              leading: Icon(Icons.document_scanner_rounded),
              title: Text('Список документов за $date'),
              subtitle: Text('Процесс: $checked/$overall')
              )
          ),
          IconButton(onPressed: () => context.appServices.eventBus.calendarEvents.nextDate(), icon: const Icon(Icons.arrow_forward_ios)),
        ],
      );
    }
    else
    {
      return SizedBox.shrink();
    }
    
  }
}
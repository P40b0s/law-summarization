import 'dart:async';

import 'package:api_client/src/bindings/bindings.dart';
import 'package:api_client/src/providers/select_document_provider.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:provider/provider.dart';

class DocumentsList extends StatefulWidget 
{
  const DocumentsList({super.key, });
  
  @override
  State<DocumentsList> createState() => _DocumentsListState();
}

class _DocumentsListState extends State<DocumentsList> 
{
  String? _selectedDate;
  List<Document> _documents = [];
  late StreamSubscription _rustSubscription;
  final formatter = DateFormat("dd.MM.yyyy");
  late ValueNotifier<TitleNotifier> _title;
  @override
  void initState() 
  {
    super.initState();
    _rustSubscription = _listenToStream();
    _title = ValueNotifier(TitleNotifier(overall: 0, ready: 0, unloaded: 0));
  }


  StreamSubscription _listenToStream() 
  {
    return DocumentPublicationDateResponse.rustSignalStream.listen((signalPack) 
    {
      if (!mounted) return;
      var documents = signalPack.message.documents;
      var selectedDate = signalPack.message.selectedDate;
      var parsed = DateTime.parse(selectedDate);
      updateAll(documents, formatter.format(parsed));
    });
  }
  @override
  Future<void> dispose() async
  {
    await _rustSubscription.cancel();
    super.dispose();
  }

  void updateAll(Iterable<Document> documents, String? selectedDate)
  {
    setState(() 
      {
        _documents = documents.toList();
        _selectedDate = selectedDate;
      });
  }

  @override
  Widget build(BuildContext context) 
  {
    return Card(
      margin: const EdgeInsets.all(8.0),
      elevation: 2,
      child: Column(
      children: [
        //FIXME не обновляется знаечние после апдейта списка
        _getSelectedDate(),
        Divider(
      color: Colors.blueAccent,     // Color of the line
      thickness: 2,           // Thickness of the line
      indent: 20,              // Empty space to the leading edge
      endIndent: 20,           // Empty space to the trailing edge
      height: 4,             // The divider's total height (spacing)
    ),
        Expanded(child: ListView.builder(
      itemCount: _documents.length,
      padding: const EdgeInsets.only(left: 3, right: 3, bottom: 4),
      
      itemBuilder: (context, index) 
      {
        var document = _documents[index];
        var unloadedLocal = document.unloaded;
        return Consumer<SaveDocumentProvider>(builder: (context, value, child)
        {
          if (document.docId == value.data?.docId)
          {
            _documents[index] = value.data!;
            document = _documents[index];
            unloadedLocal = document.unloaded;
          }
          return Material( 
            //borderRadius: BorderRadius.all(Radius.circular(3)),
            elevation: 1.0,
            shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(4.0),
                side: BorderSide(color: _getColor(), width: 1.0),
              ),
            child:  InkWell(
              onTap: () => Provider.of<SelectDocumentProvider>(context, listen: false).updateData(document),
              child:  ListTile(
                key: ValueKey(document.docId),
                title: Text(document.eoNumber),
                subtitle: Row(children: 
                [
                    Text(document.complexName),
                    Text(" Состояние unloaded: $unloadedLocal")
                ]),
              
              )
            )
          );
        });
       
      },
    )
    ),
      ],
    ),
    );
  }

  Color _getColor()
  {
    var overall = _documents.length;
    var unloaded = _documents.where((w) => w.unloaded).length;
    var checked = _documents.where((w) => w.checkedTime != null).length;
    if (overall > checked)
    {
      return const Color.fromARGB(166, 224, 131, 131);
    }
    else if (overall > unloaded)
    {
      return Colors.amberAccent;
    }
    else {return const Color.fromARGB(125, 166, 248, 133);}
  }

  void updateTitleNotifier(int overall, int ready, int unloaded)
  {
     _title.value = TitleNotifier(overall: overall, ready: ready, unloaded: unloaded);
  }

//FIXME так не обновить, потому что обновление не связано с обновлением итема в списке!
  Widget _getSelectedDate() 
  {
    if (_selectedDate != null)
    {
       return Consumer<SaveDocumentProvider>(builder: (context, value, child)
        {
          var overall = _documents.length;
          var unloaded = _documents.where((w) => w.unloaded).length;
          var checked = _documents.where((w) => w.checkedTime != null).length;
          return  ListTile(
          leading: Icon(Icons.document_scanner_rounded),
          title: Text('Список документов за ${_selectedDate!}'),
          subtitle: Text('Выгружено: $unloaded/$overall Проверено: $checked/$overall')
          );
        }
       );
    }
    else
    {
      return SizedBox.shrink();
    }
    
  }
}

class TitleNotifier
{
  final int ready;
  final int unloaded;
  final int overall;
  const TitleNotifier({required this.overall, required this.ready, required this.unloaded});
}
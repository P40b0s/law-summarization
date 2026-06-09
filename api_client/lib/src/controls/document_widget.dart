import 'package:api_client/src/bindings/bindings.dart';
import 'package:api_client/src/providers/select_document_provider.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:provider/provider.dart';
// class Editor extends StatelessWidget 
// {
//   final Document? _document;
//   const Editor({super.key, this._document});
  

//   @override
//   Widget build(BuildContext context) 
//   {
//     if (_document != null)
//     {
//       return Column(children: [
//         Expanded(child: Text(_document.eoNumber)),
//         const SizedBox(
//           width: 250,
//           child: TextField(
            
//             obscureText: false,
//             decoration: InputDecoration(
//               border: OutlineInputBorder(),
//               labelText: 'Суммаризация',
//             ),
//           ),
//         ),
//         Divider(
//         color: Colors.grey,     // Color of the line
//         thickness: 2,           // Thickness of the line
//         indent: 20,              // Empty space to the leading edge
//         endIndent: 20,           // Empty space to the trailing edge
//         height: 40,             // The divider's total height (spacing)
//       ),
//         Expanded(child: Text(_document.publicationDate)),
//       ],);
//     }
//     else
//     {
//       return const SizedBox.shrink();
//     }
    
              
//   }
// }



// Виджет для редактирования Document
class DocumentWidget extends StatefulWidget 
{
  final Document document;
  
  const DocumentWidget({
    super.key,
    required this.document,
  });

  @override
  State<DocumentWidget> createState() => _DocumentWidgetState();
}

class _DocumentWidgetState extends State<DocumentWidget> 
{
  late TextEditingController _summaryController;
  late ValueNotifier<String?> _summarizationText;
  late ValueNotifier<String?> _checkedTime;
  late ValueNotifier<bool> _unloaded;
  final WidgetStateProperty<Color?> trackColor =  WidgetStateProperty<Color?>.fromMap(<WidgetStatesConstraint, Color>
  {
    WidgetState.selected: const Color.fromARGB(255, 34, 124, 226),
  });
    // This object sets the track color based on two WidgetState attributes.
    // If neither state applies, it resolves to null.
  final WidgetStateProperty<Color?> overlayColor = WidgetStateProperty<Color?>.fromMap(<WidgetState, Color>
  {
    WidgetState.selected: const Color.fromARGB(255, 11, 95, 4).withValues(alpha: 0.54),
    WidgetState.disabled: Colors.grey.shade400,
  });

  @override
  void initState() 
  {
    super.initState();
    _summarizationText = ValueNotifier<String?>(widget.document.summarizationText);
    _checkedTime = ValueNotifier<String?>(widget.document.checkedTime);
    _unloaded = ValueNotifier<bool>(widget.document.unloaded);
    _summaryController = TextEditingController(
      text: _summarizationText.value ?? '',
    );
    
    // Отслеживаем изменения для уведомления родителя
    // _summarizationText.addListener(_notifyDocumentChanged);
    // _checkedTime.addListener(_notifyDocumentChanged);
    // _unloaded.addListener(_notifyDocumentChanged);
    _summarizationText.addListener(_updateControllerFromNotifier);
  }

  @override
  void dispose() {
    _summaryController.dispose();
    // _summarizationText.removeListener(_notifyDocumentChanged);
    // _checkedTime.removeListener(_notifyDocumentChanged);
    // _unloaded.removeListener(_notifyDocumentChanged);
    _summarizationText.removeListener(_updateControllerFromNotifier);
    _summarizationText.dispose();
    _checkedTime.dispose();
    _unloaded.dispose();
    super.dispose();
  }

  void _notifyDocumentChanged() 
  {
      final updatedDocument = widget.document.copyWith(
      summarizationText: () => _summarizationText.value,
      checkedTime: () =>  _checkedTime.value,
      unloaded: _unloaded.value,
    );
    //FIXME как то это запутано немного...
    Provider.of<SaveDocumentProvider>(context, listen: false).updateData(updatedDocument);
    // if (widget.onDocumentChanged != null) {
    //   // Создаем копию документа с новыми значениями
   
    //   widget.onDocumentChanged!(updatedDocument);
    // }
  }

  String _publicationDate() 
  {
    try {
      DateTime pd = DateTime.parse(widget.document.publicationDate);
      return DateFormat('dd.MM.yyyy').format(pd);
    } catch (e) {
      return 'Неверная дата';
    }
  }
  
  String _checkedDateFormat() {
    if (_checkedTime.value != null) {
      try {
        DateTime cd = DateTime.parse(_checkedTime.value!);
        return DateFormat('HH:mm dd.MM.yyyy').format(cd);
      } catch (e) {
        return "Ошибка формата";
      }
    } else {
      return "Не проверено";
    }
  }
    void _updateControllerFromNotifier() 
    {
      final newText = _summarizationText.value ?? '';
      if (_summaryController.text != newText) {
        _summaryController.text = newText;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.all(8.0),
      elevation: 2,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Заголовок с eoNumber
            Row(
              children: [
                Expanded(
                  child: Text(
                    widget.document.complexName,
                    style: const TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                _buildCheckedTimeWidget(),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              widget.document.eoNumber,
              style: const TextStyle(fontSize: 14, color: Colors.grey),
            ),
            const SizedBox(height: 4),
            Text(
              'Дата публикации: ${_publicationDate()}',
              style: const TextStyle(fontSize: 12, color: Colors.grey),
            ),
            Text(
              'ID документа: ${widget.document.docId}',
              style: const TextStyle(fontSize: 12, color: Colors.grey),
            ),
            const SizedBox(height: 12),
            
            // Поле краткого содержания
            TextFormField(
              controller: _summaryController,
              decoration: const InputDecoration(
                labelText: 'Краткое содержание',
                hintText: 'Введите краткое содержание...',
                border: OutlineInputBorder(),
              ),
              maxLines: 5,
              onChanged: (c) 
              {
                //Future.delayed(Duration(milliseconds: 500), () =>  _summarizationText.value = c);
              }
            ),
            const SizedBox(height: 12),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                // Чекбокс unloaded
                _buildUnloadedCheckbox(),
                 TextButton(
                  style: TextButton.styleFrom(
                    backgroundColor: Colors.lightGreen,
                    foregroundColor: Colors.black
                    ),
                  onPressed: _notifyDocumentChanged,
                  child: Text('Сохранить'),
                ),
              ],
            )
           
          ],
        ),
      ),
    );
  }

  Widget _buildCheckedTimeWidget() 
  {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        ValueListenableBuilder<String?>(
          valueListenable: _checkedTime,
          builder: (context, value, child) {
            return Switch(
              value: value != null,
              trackColor: trackColor,
              overlayColor: overlayColor,
              onChanged: (_) {
                if (_checkedTime.value == null) {
                  _checkedTime.value = DateFormat("yyyy-MM-ddTHH:mm:ss").format(DateTime.now());
                } else {
                  _checkedTime.value = null;
                }
              },
            );
          },
        ),
        const SizedBox(width: 8),
        ValueListenableBuilder<String?>(
          valueListenable: _checkedTime,
          builder: (context, value, child) {
            return SizedBox(
              width: 150,
              child: Text(
              _checkedDateFormat(),
              style: TextStyle(
                fontSize: 16,
                color: value != null ? Colors.green : Colors.grey,
              ),
            ),
          );
          },
        ),
      ],
    );
  }

  Widget _buildUnloadedCheckbox() 
  {
    return ValueListenableBuilder<bool>(
      valueListenable: _unloaded,
      builder: (context, value, child) 
      {
        final text =  value ? 'Выгружен' : "Не выгружен";
        return Row(
          children: [
            Switch(
              trackColor: trackColor,
              overlayColor: overlayColor,
              value: value,
              onChanged: (c) => _unloaded.value = c,
            ),
            const SizedBox(width: 8),
            Text(
               text,
              style: TextStyle(fontSize: 16),
            ),
          ],
        );
      },
    );
  }
}
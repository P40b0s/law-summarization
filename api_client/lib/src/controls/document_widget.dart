import 'package:api_client/src/bindings/bindings.dart';
import 'package:api_client/src/services.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

//FIXME переделать для провайдера!
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
///Виджет обновляется из родительского виджета в main при изменении выбранного документа.
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
  void dispose() 
  {
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

  void _notifyDocumentChanged(BuildContext context) 
  {
      final updatedDocument = widget.document.copyWith(
      summarizationText: () => _summarizationText.value,
      checkedTime: () =>  _checkedTime.value,
      unloaded: _unloaded.value,
    );
    //Сообщаем провайдеру о том, что документ изменился, чтобы он мог обновить свое состояние и при необходимости уведомить других слушателей
    context.appServices.documentsService.saveDocument(updatedDocument);
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
        return "Проверено ${DateFormat('HH:mm dd.MM.yyyy').format(cd)}";
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
              // Заголовок
            Text(
              widget.document.complexName,
              style: const TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
              ),
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
              maxLines: 15,
            ),
            const SizedBox(height: 12),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                // Чекбокс
                _buildCheckedTimeWidget(),
                ElevatedButton.icon(
                    icon: const Icon(Icons.save),
                    onPressed: () =>  _notifyDocumentChanged(context),
                    label: const Text('Сохранить'),
                    style: ElevatedButton.styleFrom(
                      textStyle: TextStyle(fontFamily: 'Roboto'),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.all(Radius.circular(8)),
                      ),
                    ),
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
              width: 250,
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

  // Widget _buildUnloadedCheckbox() 
  // {
  //   return ValueListenableBuilder<bool>(
  //     valueListenable: _unloaded,
  //     builder: (context, value, child) 
  //     {
  //       final text =  value ? 'Выгружен' : "Не выгружен";
  //       return Row(
  //         children: [
  //           Switch(
  //             trackColor: trackColor,
  //             overlayColor: overlayColor,
  //             value: value,
  //             onChanged: (c) => _unloaded.value = c,
  //           ),
  //           const SizedBox(width: 8),
  //           Text(
  //              text,
  //             style: TextStyle(fontSize: 16),
  //           ),
  //         ],
  //       );
  //     },
  //   );
  // }
}
import 'dart:collection';
import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:flutter/material.dart';

class DocumentsListProvider extends ChangeNotifier
{
  List<Document> _documents = [];
  List<Document> get documents => _documents;
  DateTime? _selectedDate;
  DateTime? get selectedDate => _selectedDate;
  bool _isLoading = false;
  String? _error;
  bool get isLoading => _isLoading;
  String? get error => _error;
  TitleNotifier _title = TitleNotifier(count: 0, ready: 0, unloaded: 0);
  TitleNotifier get title => _title;
  //HashMap<String, Color> _colorMap = HashMap();
  String? _isSelected;
  String? get isSelected => _isSelected;
  bool getIsSelected(Document doc) => _isSelected == doc.docId;
  //Color getColor(Document doc) => _colorMap[doc.docId] ?? Colors.transparent;
  //Color getSelectedColor(Document doc) => _isSelected == doc.docId ? const Color.fromARGB(255, 211, 248, 236) : const Color.fromARGB(255, 255, 255, 255);
  DocumentsListProvider();

  void setLoading(bool v) { _isLoading = v; notifyListeners(); }
  //TODO если добавиться еще какая то логика сделать отдельный метод _mutate и добавить все туда
  void setData(List<Document> documents, DateTime selectedDate) 
  {  
    //_colorMap.clear();
    _documents = documents;
    _isSelected = null;
    // for (var doc in documents) 
    // {
    //   _setColor(doc);
    // }
    _selectedDate = selectedDate;
    _updateCounts();
    notifyListeners();

  }

  void upsert(Document doc) 
  {
    final i = _documents.indexWhere((d) => d.docId == doc.docId);
    if (i == -1) 
    {
      _documents = [..._documents, doc];
      //_colorMap.remove(doc.docId); // сбрасываем цвет, чтобы пересчитать
      //_setColor(doc);
    } 
    else 
    {
      _documents = [..._documents]..[i] = doc;
      //_colorMap.remove(doc.docId); // сбрасываем цвет, чтобы пересчитать
      //_setColor(doc);
    }
    _updateCounts();
    notifyListeners();
  }
  void select(String? docId) 
  {
    _isSelected = docId;
    notifyListeners();
  }
  void _updateCounts() 
  {
    var count = documents.length;
    var unloaded = documents.where((w) => w.unloaded).length;
    var checked = documents.where((w) => w.checkedTime != null).length;
    if (count != _title.count || unloaded != _title.unloaded || checked != _title.ready)
    {
      _title = TitleNotifier(count: count, ready: checked, unloaded: unloaded);
    }
  }
}

class TitleNotifier
{
  final int ready;
  final int unloaded;
  final int count;
  const TitleNotifier({required this.count, required this.ready, required this.unloaded});
   @override
  bool operator ==(Object other) =>
    other is TitleNotifier &&
    other.count == count &&
    other.ready == ready &&
    other.unloaded == unloaded;
  
  @override
  int get hashCode => Object.hash(count, ready, unloaded);
}

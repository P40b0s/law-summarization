import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:flutter/material.dart';

class SelectDocumentProvider extends ChangeNotifier
{
  Document? _data;
  Document? get data => _data;
  void updateData(Document selectedDocument)
  {
    if(_data == null || _data!.docId != selectedDocument.docId)
    {
      _data = selectedDocument;
      notifyListeners();
    }
  }
}


class SaveDocumentProvider extends ChangeNotifier
{
  Document? _data;
  Document? get data => _data;
  void updateData(Document doc)
  {
    
      _data = doc;
      notifyListeners();
    
  }
}
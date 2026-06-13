import 'dart:typed_data';

import 'package:flutter/material.dart';

class ImageProvider extends ChangeNotifier
{
  late int _currentPage;
  late int _maxPage;
  late String _docId;
  late bool _isPageLoading = true;
  late Uint8List? _currentImage;
  Key _imageKey = const ValueKey<int>(1);
  ImageProvider();

  void goToPage(int page)
  {
    
  }
}
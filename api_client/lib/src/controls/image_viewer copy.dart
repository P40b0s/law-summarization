import 'dart:async';
import 'dart:typed_data';

import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

class ImageViewer extends StatefulWidget 
{
  const ImageViewer({required this.docId, required this.initialPage, required this.maxPage, super.key, });
  final int initialPage;
  final int maxPage;
  final String docId;
  @override
  State<ImageViewer> createState() => _ImageViewerState();
}


class _ImageViewerState extends State<ImageViewer> 
{
  late int _currentPage;
  late int _maxPage;
  late String _docId;
  late bool _isPageLoading = true;
  late Uint8List? _currentImage;
  Key _imageKey = const ValueKey<String>("1");
  late StreamSubscription _rustSubscription;
  @override
  void initState() 
  {
    super.initState();
    _currentPage = widget.initialPage;
    _maxPage = widget.maxPage;
    _docId = widget.docId;
    _rustSubscription = _listenToStream();
    _requestPage(1);
   
  }

  StreamSubscription _listenToStream() 
  {
    return PageResponse.rustSignalStream.listen((signalPack) 
    {
      if (!mounted) return;
      int pageNum = signalPack.message.pageNumber;
      setState(() 
      {
        _currentImage = signalPack.binary;
        _isPageLoading = false;
        _imageKey = ValueKey<String>(_docId + pageNum.toString());
      });
    });
  }

  @override
  Future<void> dispose() async
  {
    await _rustSubscription.cancel();
    super.dispose();
  }

  void _requestPage(int pageNumber)
  {
    setState(() 
    {
      _isPageLoading = true;
      _imageKey = ValueKey<String>(_docId + pageNumber.toString());
    });
    // Future.delayed(const Duration(milliseconds: 1500), () {
    //   PageRequest(id: _docId, pageNumber: pageNumber).sendSignalToRust();
    // });
    PageRequest(id: _docId, pageNumber: pageNumber).sendSignalToRust();
  }
  //анимация и замена изображения нормально не работатет со стримбилдером, поменял на листенер
  StreamBuilder _pageStream()
  {
    return StreamBuilder(
        stream: PageResponse.rustSignalStream,
        builder: (context, snapshot) 
        {
          //обновление состояния после получения данных
          WidgetsBinding.instance.addPostFrameCallback((_) 
          {
              if (mounted) 
              {
                setState(() 
                {
                  _isPageLoading = false;
                });
              }
          });
          final signalPack = snapshot.data;
          if (signalPack == null || _isPageLoading) 
          {
            return const Center(
              child: CircularProgressIndicator(),
            );
          }
          final imageData = signalPack.binary;
          
                    // Показываем изображение с анимацией
          return AnimatedSwitcher(
  duration: const Duration(milliseconds: 800), // Увеличили длительность
  transitionBuilder: (Widget child, Animation<double> animation) {
    // Комбинированная анимация - более заметная
    return FadeTransition(
      opacity: animation,
      child: ScaleTransition(
        scale: Tween<double>(begin: 0.8, end: 1.0).animate(
          CurvedAnimation(parent: animation, curve: Curves.easeOutBack),
        ),
        child: child,
      ),
    );
  },
            child: Image.memory(
              key: _imageKey,
              imageData,
              fit: BoxFit.contain,
              errorBuilder: (context, error, stackTrace) 
              {
                return Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const Icon(Icons.error, size: 50, color: Colors.red),
                      const SizedBox(height: 10),
                      const Text('Не удалось загрузить изображение'),
                    ],
                  ),
                );
              },
            ),
          );
        }
    );
  }

  Widget _image()
  {
    if (_isPageLoading || _currentImage == null) 
    {
      return const Center(child: CircularProgressIndicator());
    }
    // Показываем изображение с анимацией
    //FIXME анимация не работает, иправить потом или вообще убрать ее
    return AnimatedSwitcher(
      duration: const Duration(milliseconds: 800), // Увеличили длительность
      transitionBuilder: (Widget child, Animation<double> animation) 
      {
        // Комбинированная анимация - более заметная
        return FadeTransition(
          opacity: animation,
          child: ScaleTransition(
            scale: Tween<double>(begin: 0.8, end: 1.0).animate(
              CurvedAnimation(parent: animation, curve: Curves.easeOutBack),
            ),
            child: child,
          ),
        );
      },
      child: Listener(
          onPointerSignal: (pointerSignal) 
          {
            if (pointerSignal is PointerScrollEvent) 
            {
              if (pointerSignal.scrollDelta.dy < 0) 
              {
                _previousPage();
              } 
              else if (pointerSignal.scrollDelta.dy > 0) 
              {
                _nextPage();
              }
            }
          },
          child: Image.memory(
                key: _imageKey,
                _currentImage!,
                fit: BoxFit.contain,
                errorBuilder: (context, error, stackTrace) 
                {
                  return Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        const Icon(Icons.error, size: 50, color: Colors.red),
                        const SizedBox(height: 10),
                        const Text('Не удалось загрузить изображение'),
                      ],
                    ),
                  );
                },
              ),
            ),
    );
  }

  void _nextPage() 
  {
    if (_currentPage < _maxPage && !_isPageLoading) 
    {
      setState(() 
      {
        _currentPage++;
      });
      _requestPage(_currentPage);
    } 
    else 
    {
      _showSnackBar('Это последняя страница');
    }
  }

  void _previousPage() 
  {
    if (_currentPage > 1 && !_isPageLoading) 
    {
      setState(() 
      {
        _currentPage--;
      });
      _requestPage(_currentPage);
    } 
    else 
    {
      _showSnackBar('Это первая страница');
    }
  }

  void _showSnackBar(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        duration: Duration(seconds: 1),
      ),
    );
  }

  void _goToPage() 
  {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Перейти на страницу'),
        content: TextField(
          keyboardType: TextInputType.number,
          decoration: InputDecoration(
            labelText: 'Номер страницы (1-$_maxPage)',
            border: OutlineInputBorder(),
          ),
          
          onSubmitted: (value) {
            final int? page = int.tryParse(value);
            if (page != null && page >= 1 && page <= _maxPage) 
            {
              setState(() 
              {
                _currentPage = page;
              });
              _requestPage(_currentPage);
              Navigator.pop(context);
            } 
            else 
            {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('Введите число от 1 до $_maxPage')),
              );
            }
          },
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text('Отмена'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) 
  {
    return Column(
      textDirection: TextDirection.ltr,
      children: [
        // Область с изображением
        Expanded(
          flex: 10,
          child: Container(
            padding: EdgeInsets.all(1),
            child: _image(),
          ),
        ),
        
        // Панель управления
        Expanded(
          flex: 1,
            child: Wrap(
          
              children: [
                // Кнопка "Предыдущая"
                Padding(
                padding: EdgeInsets.all(5.0),
                child: ElevatedButton.icon(
                  icon: const Icon(Icons.navigate_before),
                  onPressed: _previousPage,
                  label: const Text('Предыдущая'),
                  style: ElevatedButton.styleFrom(
                    textStyle: TextStyle(fontFamily: 'Roboto'),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.all(Radius.circular(8)),
                    ),
                  ),
                ),
                ),
                
                // Индикатор страницы с возможностью ввода
                Padding(
                padding: EdgeInsets.all(5.0),
                child:  InkWell(
                  onTap: _goToPage,
                  child: Container(
                    height: 33,
                    padding: EdgeInsets.all(2),
                    decoration: BoxDecoration(
                      border: Border.all(color: Colors.grey),
                      borderRadius: BorderRadius.circular(4),
                    ),
                    child: Text(
                      'Страница $_currentPage из $_maxPage',
                      style: TextStyle(fontSize: 16),
                    ),
                  ),
                ),
                ),
                
                // Кнопка "Следующая"
                Padding(
                padding: EdgeInsets.all(5.0),
                child: ElevatedButton.icon(
                  icon: const Icon(Icons.navigate_next),
                  onPressed: _nextPage,
                  label: const Text('Следующая'),
                  style: ElevatedButton.styleFrom(
                    textStyle: TextStyle(fontFamily: 'Roboto'),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.all(Radius.circular(8)),
                    ),
                  ),
                ),
                ),
              ],
            ),
          ),
      ],
    );
  }
}
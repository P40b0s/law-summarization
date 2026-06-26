import 'dart:async';
import 'dart:typed_data';

import 'package:api_client/src/bindings/signals/signals.dart';
import 'package:api_client/src/services.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

class ImageViewer extends StatefulWidget 
{
  const ImageViewer({super.key, });
  @override
  State<ImageViewer> createState() => _ImageViewerState();
}


class _ImageViewerState extends State<ImageViewer> 
{
  @override
  void initState() 
  {
    super.initState();
  }

  // @override
  // Future<void> dispose() async
  // {
  //   await _rustSubscription.cancel();
  //   super.dispose();
  // }

  Widget _image()
  {
    if (context.appServices.imageViewerService.provider.isPageLoading || context.appServices.imageViewerService.provider.currentImage == null) 
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
                context.appServices.imageViewerService.previousPage();
              } 
              else if (pointerSignal.scrollDelta.dy > 0) 
              {
                context.appServices.imageViewerService.nextPage();
              }
            }
          },
          child: Image.memory(
                key: context.appServices.imageViewerService.provider.imageKey,
                context.appServices.imageViewerService.provider.currentImage!,
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


  // void _showSnackBar(String message) {
  //   ScaffoldMessenger.of(context).showSnackBar(
  //     SnackBar(
  //       content: Text(message),
  //       duration: Duration(seconds: 1),
  //     ),
  //   );
  // }

  void _goToPage() 
  {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Перейти на страницу'),
        content: TextField(
          keyboardType: TextInputType.number,
          decoration: InputDecoration(
            labelText: 'Номер страницы (1-${context.appServices.imageViewerService.provider.maxPage})',
            border: OutlineInputBorder(),
          ),
          
          onSubmitted: (value) 
          {
            final int? page = int.tryParse(value);
            if (page != null && page >= 1 && page <= context.appServices.imageViewerService.provider.maxPage) 
            {
              context.appServices.imageViewerService.requestPage(page);
              Navigator.pop(context);
            } 
            else 
            {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('Введите число от 1 до ${context.appServices.imageViewerService.provider.maxPage}')),
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
    return ListenableBuilder(
      listenable: context.appServices.imageViewerService.provider,
      builder: (_, _)
      {
        if (context.appServices.imageViewerService.provider.currentImage != null)
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
                        onPressed: context.appServices.imageViewerService.previousPage,
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
                            'Страница ${context.appServices.imageViewerService.provider.currentPage} из ${context.appServices.imageViewerService.provider.maxPage}',
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
                        onPressed: context.appServices.imageViewerService.nextPage,
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
          else
          {
            return SizedBox.shrink();
          }
        }
      );
  }
}
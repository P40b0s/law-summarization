import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

class DocumentsProgressProvider extends Progress
{
  DocumentsProgressProvider({super.titleText = "Прогресс обработки документов"});
}

class PagesProgressProvider extends Progress
{
  PagesProgressProvider({super.titleText = "Прогресс обработки страниц"});
}


abstract class Progress extends ChangeNotifier
{
  int _count = 0;
  int _progress = 0;
  int get count => _count;
  int get progress => _progress;
  double _animatedProgress = 0.0;
  double get animatedProgress => _animatedProgress;
  late final AnimationController _animationController;
  AnimationController get animationController => _animationController;
  double _startProgress = 0.0;
  double _targetProgress = 0.0;
  String titleText;
  bool _inProgress = false;
  bool get inProgress => _inProgress;
  Progress({required this.titleText})
  {
    _animationController = AnimationController(
      duration: const Duration(milliseconds: 500),
      vsync: const _BackgroundTickerProvider(),
    );
    _animationController.addListener(_onAnimationTick);
  }

  void _onAnimationTick() 
  {
    // Плавно переходим от начального прогресса к целевому 
    // на основе текущего шага анимации (от 0.0 до 1.0)
    _animatedProgress = _startProgress + 
        (_targetProgress - _startProgress) * _animationController.value;
    notifyListeners();
  }

  void _animateToNewProgress() 
  {
    _startProgress = _animatedProgress; // Начинаем движение с того места, где полоса стоит сейчас
    _targetProgress = barProgress;       // Стремимся к новому значению
    _animationController.forward(from: 0.0); // Перезапускаем контроллер
  }

  void changeState(int count, int progress)
  {
    _count = count;
    _progress = progress;
    _inProgress = true;
    _animateToNewProgress(); 
    //notifyListeners();
  }
  
  // Вычисляемое свойство - прогресс от 0.0 до 1.0
  double get barProgress 
  {
    if (_count == 0) return 0.0;
    return (_progress / _count).clamp(0.0, 1.0);
  }
  // Проценты в виде строки
  String get percent => '${(_progress * 100).toStringAsFixed(1)}%';

    // Обновление данных с анимацией
  void updateData({required int count, required int progress}) 
  {
    _count = count;
    _progress = progress.clamp(0, count);
    _animateToNewProgress();
  }

  // Обновление только количества
  void updateCount(int newCount) 
  {
    _count = newCount;
    _progress = _progress.clamp(0, newCount);
    _animateToNewProgress();
  }
  
  // Обновление только прогресса
  void updateProgress(int newProgress) 
  {
    _progress = newProgress.clamp(0, _count);
    _animateToNewProgress();
  }

  void resetProgress() 
  {
    _count = 0;
    _progress = 0;
    _startProgress = 0.0;
    _targetProgress = 0.0;
    _animatedProgress = 0.0;
    _inProgress = false;
    _animationController.reset();
    notifyListeners();
  }

  Color getProgressColor(double progress) 
  {
    if (progress >= 1.0) return Colors.green;
    if (progress >= 0.7) return Colors.blue;
    if (progress >= 0.4) return Colors.orange;
    return Colors.red;
  }

  @override
  void dispose() 
  {
    _animationController.removeListener(_onAnimationTick);
    _animationController.dispose();
    super.dispose();
  }

}

class _BackgroundTickerProvider implements TickerProvider 
{
  const _BackgroundTickerProvider();

  @override
  Ticker createTicker(TickerCallback onTick) => Ticker(onTick);
}
import 'package:api_client/src/providers/task_progress_provider.dart';
import 'package:api_client/src/services.dart';
import 'package:flutter/material.dart';


class TaskProgressStatus extends StatelessWidget 
{
  
  const TaskProgressStatus(
    {
      super.key,
    });
 @override
  Widget build(BuildContext context) 
  {
    return ProgressIcon(icon: Icons.android_sharp, size: 32.0,);
  }
}


class TaskProgress<T extends Progress> extends StatelessWidget 
{
  final double height;
  final double borderRadius;
  final T provider;
  const TaskProgress(
    {
      required this.provider,
      super.key,
      this.height = 12.0,
      this.borderRadius = 8.0,
    });
 @override
  Widget build(BuildContext context) 
  {
    // Получаем доступ к вашему провайдеру страниц (замените на ваш способ: GetIt, Provider, или синглтон)
    // Например: final progressModel = GetIt.I<PageProvider>(); или через статическое поле:
    final progressModel = provider; 

    return ListenableBuilder(
      listenable: progressModel,
      builder: (context, child) 
      {
        final currentProgress = progressModel.animatedProgress;
        final progressColor = progressModel.getProgressColor(progressModel.barProgress);

        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            // Верхняя информационная строка
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  progressModel.titleText,
                  style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14),
                ),
                Text(
                  '${progressModel.progress} / ${progressModel.count} (${(currentProgress * 100).toStringAsFixed(1)}%)',
                  style: const TextStyle(color: Colors.grey, fontSize: 13, fontWeight: FontWeight.w500),
                ),
              ],
            ),
            const SizedBox(height: 8),
            
            // Линия прогресс-бара
            Container(
              height: height,
              decoration: BoxDecoration(
                color: Colors.grey.shade200,
                borderRadius: BorderRadius.circular(borderRadius),
              ),
              child: LayoutBuilder(
                builder: (context, constraints) {
                  return Align(
                    alignment: Alignment.centerLeft,
                    child: Container(
                      // Ширина рассчитывается на основе значения из AnimationController
                      width: constraints.maxWidth * currentProgress,
                      decoration: BoxDecoration(
                        color: progressColor,
                        borderRadius: BorderRadius.circular(borderRadius),
                      ),
                    ),
                  );
                },
              ),
            ),
          ],
        );
      },
    );
  }
}


class ProgressIcon extends StatelessWidget {
  final IconData icon;
  final Color backgroundColor;
  final Color progressColor;
  final double size;

  const ProgressIcon({
    super.key,
    required this.icon,
    this.backgroundColor = Colors.grey,
    this.progressColor = Colors.green,
    this.size = 48.0,
  });

  @override
  Widget build(BuildContext context) {
    // Оборачиваем весь SizedBox в Tooltip
    return Tooltip(
      // Задаем небольшую задержку перед появлением при наведении мыши
      waitDuration: const Duration(milliseconds: 300),
      // Используем richMessage и WidgetSpan для отображения кастомных виджетов
      richMessage: WidgetSpan(
        child: Padding(
          padding: const EdgeInsets.all(8.0), // Отступы внутри тултипа
          child: Column(
            mainAxisSize: MainAxisSize.min, // Чтобы тултип не растягивался на весь экран
            children: [
              TaskProgress(provider: context.appServices.taskProgressService.docsProvider),
              const SizedBox(height: 8), // Небольшой отступ между прогресс-барами
              TaskProgress(provider: context.appServices.taskProgressService.pagesProvider),
            ],
          ),
        ),
      ),
      child: SizedBox(
        width: size,
        height: size,
        child: Stack(
          alignment: Alignment.center,
          children: [
            // 1. Фоновая иконка
            Icon(
              icon,
              size: size,
              color: backgroundColor,
            ),
            // 2. Иконка прогресса
            ListenableBuilder(
              listenable: context.appServices.taskProgressService.docsProvider,
              builder: (context, child) {
                return ClipRect(
                  // Обрезаем вторую иконку по горизонтали в зависимости от прогресса
                  clipper: _LeftToRightClipper(context.appServices.taskProgressService.docsProvider.barProgress),
                  child: Icon(
                    icon,
                    size: size,
                    color: progressColor,
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}

// Класс для вычисления области обрезки
class _ProgressClipper extends CustomClipper<Rect> {
  final double progress;

  _ProgressClipper(this.progress);

  @override
  Rect getClip(Size size) {
    // Обрезаем справа налево (для языков LTR). 
    // Если хотите заполнение слева направо, используйте 0, 0, size.width * progress, size.height
    return Rect.fromLTWH(
      size.width * (1.0 - progress),
      0,
      size.width * progress,
      size.height,
    );
  }

  @override
  bool shouldReclip(covariant _ProgressClipper oldClipper) {
    return oldClipper.progress != progress;
  }
}

// Кастомный клиппер для заполнения слева направо
class _LeftToRightClipper extends CustomClipper<Rect> {
  final double progress;

  _LeftToRightClipper(this.progress);

  @override
  Rect getClip(Size size) {
    // Начинаем от левого края (0, 0) и плавно увеличиваем ширину
    return Rect.fromLTWH(
      0,
      0,
      size.width * progress,
      size.height,
    );
  }

  @override
  bool shouldReclip(covariant _LeftToRightClipper oldClipper) {
    return oldClipper.progress != progress;
  }
}

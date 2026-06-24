import 'package:api_client/src/bindings/bindings.dart';
import 'package:api_client/src/providers/error_provider.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

// ─── Styling helpers ─────────────────────────────────────────────────────────

class _SeverityStyle {
  const _SeverityStyle({
    required this.icon,
    required this.accent,
    required this.background,
  });
  final IconData icon;
  final Color accent;
  final Color background;
}

_SeverityStyle _styleFor(
  ErrorSeverity s,
  ColorScheme scheme,
  Brightness brightness,
) {
  final dark = brightness == Brightness.dark;
  switch (s) {
    case ErrorSeverity.info:
      return _SeverityStyle(
        icon: Icons.info_outline,
        accent: scheme.primary,
        background:
            dark ? scheme.surfaceContainerHigh : scheme.primaryContainer,
      );
    case ErrorSeverity.warning:
      return _SeverityStyle(
        icon: Icons.warning_amber_rounded,
        accent: dark ? Colors.orange.shade300 : Colors.orange.shade800,
        background: dark
            ? Colors.orange.shade900.withOpacity(0.35)
            : Colors.orange.shade50,
      );
    case ErrorSeverity.error:
      return _SeverityStyle(
        icon: Icons.error_outline,
        accent: scheme.error,
        background: dark
            ? scheme.errorContainer.withOpacity(0.45)
            : scheme.errorContainer,
      );
    case ErrorSeverity.success:
      return _SeverityStyle(
        icon: Icons.check_circle_outline,
        accent: dark ? Colors.green.shade300 : Colors.green.shade700,
        background: dark
            ? Colors.green.shade900.withOpacity(0.35)
            : Colors.green.shade50,
      );
  }
}

// ─── ToastOverlay ────────────────────────────────────────────────────────────

/// Wraps the app and shows animated toasts for every new [ErrorProvider]
/// entry. Place it ABOVE the Navigator (via `MaterialApp.builder`) so
/// toasts float over the whole app:
///
/// ```dart
/// MaterialApp(
///   builder: (context, child) => ToastOverlay(child: child!),
///   home: ...,
/// )
/// ```
class ToastOverlay extends StatefulWidget {
  const ToastOverlay({
    super.key,
    required this.child,
    this.toastDuration = const Duration(seconds: 4),
    this.maxVisible = 4,
  });
  final Widget child;
  final Duration toastDuration;
  final int maxVisible;

  @override
  State<ToastOverlay> createState() => _ToastOverlayState();
}

class _ToastOverlayState extends State<ToastOverlay> {
  late ErrorProvider _provider;
  final List<_ToastInstance> _toasts = [];
  int _seenLength = 0;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _provider = Provider.of<ErrorProvider>(context, listen: false);
    _seenLength = _provider.history.length;
    _provider.addListener(_onChange);
  }

  @override
  void dispose() {
    _provider.removeListener(_onChange);
    super.dispose();
  }

  void _onChange() {
    if (!mounted) return;
    final history = _provider.history;
    if (history.length == _seenLength) return;
    if (history.length < _seenLength) {
      _seenLength = history.length; // e.g. after clearHistory()
      return;
    }
    final newEntries = history.take(history.length - _seenLength).toList();
    _seenLength = history.length;
    for (final entry in newEntries) {
      _addToast(entry);
    }
  }

  void _addToast(ErrorEntry entry) {
    setState(() {
      _toasts.add(_ToastInstance(entry: entry, duration: widget.toastDuration));
      // Cap the number of simultaneously visible toasts.
      while (_toasts.length > widget.maxVisible) {
        _toasts.removeAt(0);
      }
    });
  }

  void _removeToast(String id) {
    if (!mounted) return;
    setState(() {
      _toasts.removeWhere((t) => t.entry.id == id);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        widget.child,
        Positioned(
          top: 0,
          left: 0,
          right: 0,
          child: SafeArea(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  for (final t in _toasts)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 8),
                      child: Align(
                        alignment: Alignment.topCenter,
                        child: ConstrainedBox(
                          constraints: const BoxConstraints(maxWidth: 420),
                          child: _ToastCard(
                            instance: t,
                            onDismissed: () => _removeToast(t.entry.id),
                          ),
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}

class _ToastInstance {
  _ToastInstance({required this.entry, required this.duration});
  final ErrorEntry entry;
  final Duration duration;
}

class _ToastCard extends StatefulWidget {
  const _ToastCard({required this.instance, required this.onDismissed});
  final _ToastInstance instance;
  final VoidCallback onDismissed;

  @override
  State<_ToastCard> createState() => _ToastCardState();
}

class _ToastCardState extends State<_ToastCard>
    with TickerProviderStateMixin {
  late final AnimationController _entry;
  late final AnimationController _progress;
  late final Animation<double> _fade;
  late final Animation<Offset> _slide;
  bool _dismissing = false;

  @override
  void initState() {
    super.initState();
    _entry = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 280),
      reverseDuration: const Duration(milliseconds: 200),
    );
    _fade = CurvedAnimation(parent: _entry, curve: Curves.easeOut);
    _slide = Tween<Offset>(
      begin: const Offset(0, -0.45),
      end: Offset.zero,
    ).animate(
      CurvedAnimation(parent: _entry, curve: Curves.easeOutCubic),
    );

    _progress = AnimationController(
      vsync: this,
      duration: widget.instance.duration,
    )
      ..forward()
      ..addStatusListener(_onProgress);

    _entry.forward();
  }

  void _onProgress(AnimationStatus s) {
    if (s == AnimationStatus.completed && !_dismissing && mounted) {
      _dismiss();
    }
  }

  void _dismiss() {
    if (_dismissing || !mounted) return;
    _dismissing = true;
    _progress.stop();
    _entry.reverse().then((_) {
      if (mounted) widget.onDismissed();
    });
  }

  @override
  void dispose() {
    _progress.removeStatusListener(_onProgress);
    _entry.dispose();
    _progress.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    final brightness = Theme.of(context).brightness;
    final style =
        _styleFor(widget.instance.entry.severity, scheme, brightness);
    final entry = widget.instance.entry;

    return SlideTransition(
      position: _slide,
      child: FadeTransition(
        opacity: _fade,
        child: Container(
          decoration: BoxDecoration(
            color: style.background,
            borderRadius: BorderRadius.circular(14),
            border: Border(left: BorderSide(color: style.accent, width: 4)),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withOpacity(0.18),
                blurRadius: 16,
                offset: const Offset(0, 6),
              ),
            ],
          ),
          clipBehavior: Clip.antiAlias,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(12, 10, 4, 8),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Icon(style.icon, color: style.accent, size: 22),
                    const SizedBox(width: 10),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          if (entry.title != null) ...[
                            Text(
                              entry.title!,
                              style: TextStyle(
                                fontWeight: FontWeight.w600,
                                fontSize: 14,
                                color: style.accent,
                                height: 1.2,
                              ),
                            ),
                            const SizedBox(height: 2),
                          ],
                          Text(
                            entry.message,
                            style: TextStyle(
                              fontSize: 13.5,
                              height: 1.3,
                              color: scheme.onSurface.withOpacity(0.85),
                            ),
                          ),
                          if (entry.actionLabel != null)
                            Padding(
                              padding: const EdgeInsets.only(top: 4),
                              child: TextButton(
                                onPressed: () {
                                  entry.onAction?.call();
                                  _dismiss();
                                },
                                style: TextButton.styleFrom(
                                  foregroundColor: style.accent,
                                  padding: const EdgeInsets.symmetric(
                                    horizontal: 8,
                                  ),
                                  minimumSize: const Size(0, 28),
                                  tapTargetSize:
                                      MaterialTapTargetSize.shrinkWrap,
                                  textStyle: const TextStyle(
                                    fontWeight: FontWeight.w600,
                                  ),
                                ),
                                child: Text(entry.actionLabel!),
                              ),
                            ),
                        ],
                      ),
                    ),
                    IconButton(
                      icon: const Icon(Icons.close, size: 18),
                      color: scheme.onSurface.withOpacity(0.4),
                      onPressed: _dismiss,
                      visualDensity: VisualDensity.compact,
                      padding: EdgeInsets.zero,
                      constraints: const BoxConstraints(
                        minWidth: 32,
                        minHeight: 32,
                      ),
                    ),
                  ],
                ),
              ),
              AnimatedBuilder(
                animation: _progress,
                builder: (_, __) => LinearProgressIndicator(
                  value: _progress.value,
                  minHeight: 2,
                  backgroundColor: Colors.transparent,
                  valueColor: AlwaysStoppedAnimation(
                    style.accent.withOpacity(0.7),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// ─── RecentErrorsPanel ───────────────────────────────────────────────────────

/// Bottom sheet listing [ErrorProvider.history]. Open via
/// [RecentErrorsPanel.show].
class RecentErrorsPanel extends StatelessWidget {
  const RecentErrorsPanel({super.key});

  static Future<void> show(BuildContext context) {
    return showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (_) => const RecentErrorsPanel(),
    );
  }

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return DraggableScrollableSheet(
      initialChildSize: 0.6,
      minChildSize: 0.3,
      maxChildSize: 0.95,
      expand: false,
      builder: (context, scrollController) {
        return Container(
          decoration: BoxDecoration(
            color: scheme.surface,
            borderRadius:
                const BorderRadius.vertical(top: Radius.circular(24)),
          ),
          child: Consumer<ErrorProvider>(
            builder: (context, provider, _) {
              final entries = provider.history;
              return Column(
                children: [
                  const SizedBox(height: 10),
                  Container(
                    width: 40,
                    height: 4,
                    decoration: BoxDecoration(
                      color: scheme.onSurface.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(2),
                    ),
                  ),
                  Padding(
                    padding: const EdgeInsets.fromLTRB(20, 16, 8, 8),
                    child: Row(
                      children: [
                        Icon(Icons.history, color: scheme.primary, size: 22),
                        const SizedBox(width: 10),
                        Text(
                          'Recent errors',
                          style: Theme.of(context).textTheme.titleLarge,
                        ),
                        const SizedBox(width: 8),
                        if (entries.isNotEmpty)
                          Container(
                            padding: const EdgeInsets.symmetric(
                              horizontal: 8,
                              vertical: 2,
                            ),
                            decoration: BoxDecoration(
                              color: scheme.primaryContainer,
                              borderRadius: BorderRadius.circular(10),
                            ),
                            child: Text(
                              '${entries.length}',
                              style: TextStyle(
                                fontSize: 12,
                                fontWeight: FontWeight.w600,
                                color: scheme.onPrimaryContainer,
                              ),
                            ),
                          ),
                        const Spacer(),
                        if (entries.isNotEmpty)
                          TextButton.icon(
                            onPressed: provider.clearHistory,
                            icon: const Icon(Icons.delete_sweep, size: 18),
                            label: const Text('Clear'),
                          ),
                      ],
                    ),
                  ),
                  const Divider(height: 1),
                  Expanded(
                    child: entries.isEmpty
                        ? const _EmptyHistory()
                        : ListView.separated(
                            controller: scrollController,
                            padding: const EdgeInsets.symmetric(vertical: 8),
                            itemCount: entries.length,
                            separatorBuilder: (_, __) =>
                                const Divider(height: 1, indent: 64),
                            itemBuilder: (_, i) =>
                                _ErrorTile(entry: entries[i]),
                          ),
                  ),
                ],
              );
            },
          ),
        );
      },
    );
  }
}

class _EmptyHistory extends StatelessWidget {
  const _EmptyHistory();

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 80,
            height: 80,
            decoration: const BoxDecoration(
              color: Color(0xFFE6F4EA),
              shape: BoxShape.circle,
            ),
            child: const Icon(
              Icons.check_rounded,
              size: 40,
              color: Color(0xFF1E8E3E),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            'No errors so far',
            style: Theme.of(context).textTheme.titleMedium,
          ),
          const SizedBox(height: 4),
          Text(
            "You're all clear",
            style:
                TextStyle(color: scheme.onSurface.withOpacity(0.6)),
          ),
        ],
      ),
    );
  }
}

class _ErrorTile extends StatelessWidget {
  const _ErrorTile({required this.entry});
  final ErrorEntry entry;

  String _formatTime(DateTime t) {
    final d = DateTime.now().difference(t);
    if (d.inSeconds < 60) return '${d.inSeconds}s ago';
    if (d.inMinutes < 60) return '${d.inMinutes}m ago';
    if (d.inHours < 24) return '${d.inHours}h ago';
    return '${d.inDays}d ago';
  }

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    final brightness = Theme.of(context).brightness;
    final style = _styleFor(entry.severity, scheme, brightness);

    return ListTile(
      leading: CircleAvatar(
        backgroundColor: style.background,
        child: Icon(style.icon, color: style.accent, size: 20),
      ),
      title: Text(
        entry.title ?? entry.message,
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
        style: const TextStyle(fontWeight: FontWeight.w600),
      ),
      subtitle: entry.title == null
          ? null
          : Text(
              entry.message,
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
      trailing: Text(
        _formatTime(entry.timestamp),
        style: TextStyle(
          fontSize: 12,
          color: scheme.onSurface.withOpacity(0.5),
        ),
      ),
      onTap: () {
        entry.onAction?.call();
      },
    );
  }
}
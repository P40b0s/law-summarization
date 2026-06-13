// import 'package:api_client/src/providers/error_provider.dart';
// import 'package:flutter/material.dart';
// import 'package:provider/provider.dart';


// class SnackBarExample extends StatefulWidget {
//   const SnackBarExample({super.key});

//   @override
//   State<SnackBarExample> createState() => _SnackBarExampleState();
// }

// class _SnackBarExampleState extends State<SnackBarExample> {
//   late final ErrorProvider _errorProvider;
//   bool _wasShown = false; // защита от двойного показа

//   @override
//   void initState() 
//   {
//     super.initState();
//     _errorProvider = context.read<ErrorProvider>();
//     _errorProvider.addListener(_handleError);
//   }

//   @override
//   void dispose() {
//     _errorProvider.removeListener(_handleError);
//     super.dispose();
//   }

//   void _handleError() {
//     if (_errorProvider.isShow && !_wasShown) {
//       _wasShown = true;
//       WidgetsBinding.instance.addPostFrameCallback((_) {
//         if (!mounted) return;
//         final messenger = ScaffoldMessenger.of(context);
//         messenger.hideCurrentSnackBar(); // чтобы новые сообщения не терялись
//         messenger
//             .showSnackBar(
//           SnackBar(
//             key: ValueKey(_errorProvider.error), // уникальность на каждый текст
//             content: Text(_errorProvider.error ?? ''),
//             behavior: SnackBarBehavior.floating,
//             duration: const Duration(seconds: 3),
//           ),
//         )
//             .closed
//             .then((reason) {
//           // когда снэкбар закрылся (по таймеру или свайпом) — сбрасываем провайдер
//           if (mounted) _errorProvider.close();
//         });
//       });
//     } else if (!_errorProvider.isShow) {
//       _wasShown = false;
//     }
//   }

//   @override
//   Widget build(BuildContext context) {
//     // return Center(
//     //   child: ElevatedButton(
//     //     child: const Text('Show Snackbar'),
//     //     onPressed: () {
//     //       context.read<ErrorProvider>().spawnError('Awesome Snackbar!');
//     //     },
//     //   ),
//     // );
//     return SizedBox.shrink();
//   }
// }
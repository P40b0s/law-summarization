import 'package:api_client/src/controls/calendar.dart';
import 'package:api_client/src/controls/docs_list.dart';
import 'package:flutter/material.dart';

class Leftpanel extends StatelessWidget 
{
  const Leftpanel({super.key});
  @override
  Widget build(BuildContext context) 
  {
    return Column(children: [
      Expanded(child: Calendar()),
      Expanded(child: DocumentsList()),
    ],);
              
  }
}
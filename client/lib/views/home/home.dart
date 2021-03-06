import 'package:client/views/home/content_dekstop.dart';
import 'package:client/views/home/content_mobile.dart';
import 'package:flutter/material.dart';
import 'package:responsive_builder/responsive_builder.dart';

class Home extends StatelessWidget {
  const Home({Key key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ScreenTypeLayout(
      mobile: ContentMobile(),
      desktop: ContentDesktop(),
    );
  }
}

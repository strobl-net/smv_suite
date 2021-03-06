import 'package:client/models/navigation_bar_item.dart';
import 'package:client/services/navigation/navigation_service.dart';
import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:provider/provider.dart';
import 'package:responsive_builder/responsive_builder.dart';
import 'package:stacked/stacked.dart';

class NavBarItem extends StatelessWidget {
  final String title;
  final String navigationPath;
  final IconData icon;
  final double fontSize;
  const NavBarItem(this.title, this.navigationPath, {this.icon, this.fontSize});

  @override
  Widget build(BuildContext context) {
    var model = NavBarItemModel(
      title: title,
      navigationPath: navigationPath,
      iconData: icon
    );
    return GestureDetector(
        onTap: () {
          GetIt.I<NavigationService>().navigateTo(navigationPath);
        },
      child: Provider.value(
        value: model,
        child: ScreenTypeLayout(
          tablet: NavBarItemDesktop(),
          mobile: NavBarItemMobile(),
        ),
      )
    );
  }
}

class NavBarItemDesktop extends ViewModelWidget<NavBarItemModel> {
  @override
  Widget build(BuildContext context, NavBarItemModel model) {
    final fontSize = model.fontSize == null ? 18 : model.fontSize;
    return Text(
      model.title,
      style: TextStyle(fontSize: fontSize),
    );
  }
}

class NavBarItemMobile extends ViewModelWidget<NavBarItemModel> {
  @override
  Widget build(BuildContext context, NavBarItemModel model) {
    return Padding(
      padding: const EdgeInsets.only(left: 30, top: 60),
      child: Row(
        children: <Widget>[
          Icon(model.iconData),
          SizedBox(
            width: 30,
          ),
          Text(
            model.title,
            style: TextStyle(fontSize: 18),
          )
        ],
      ),
    );
  }
}

class NavBarLogo extends StatelessWidget {
  const NavBarLogo({Key key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
        height: 80,
        width: 150,
        child: Image.asset('assets/images/logo.png')
    );
  }
}
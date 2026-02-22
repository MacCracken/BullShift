import 'package:flutter/material.dart';
import 'dart:math';

enum DrawingToolType {
  none,
  trendline,
  horizontalLine,
  verticalLine,
  fibonacciRetracement,
  fibonacciExtension,
  rectangle,
  textAnnotation,
}

abstract class DrawingObject {
  String id;
  Color color;
  double strokeWidth;
  bool visible;

  DrawingObject({
    required this.id,
    this.color = Colors.yellow,
    this.strokeWidth = 1.0,
    this.visible = true,
  });

  void draw(Canvas canvas, Size size, double priceScale, double timeScale);

  Map<String, dynamic> toJson();

  static DrawingObject? fromJson(Map<String, dynamic> json) {
    return null;
  }
}

class Trendline extends DrawingObject {
  double startPrice;
  double endPrice;
  DateTime startTime;
  DateTime endTime;

  Trendline({
    required super.id,
    required this.startPrice,
    required this.endPrice,
    required this.startTime,
    required this.endTime,
    super.color,
    super.strokeWidth,
  });

  @override
  void draw(Canvas canvas, Size size, double priceScale, double timeScale) {
    final paint = Paint()
      ..color = color
      ..strokeWidth = strokeWidth
      ..style = PaintingStyle.stroke;

    final startX = startTime.millisecondsSinceEpoch * timeScale;
    final endX = endTime.millisecondsSinceEpoch * timeScale;
    final startY = (priceScale - startPrice) * size.height / priceScale;
    final endY = (priceScale - endPrice) * size.height / priceScale;

    canvas.drawLine(Offset(startX, startY), Offset(endX, endY), paint);

    final dotPaint = Paint()
      ..color = color
      ..style = PaintingStyle.fill;
    canvas.drawCircle(Offset(startX, startY), 4, dotPaint);
    canvas.drawCircle(Offset(endX, endY), 4, dotPaint);
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'trendline',
    'id': id,
    'startPrice': startPrice,
    'endPrice': endPrice,
    'startTime': startTime.millisecondsSinceEpoch,
    'endTime': endTime.millisecondsSinceEpoch,
    'color': color.value,
    'strokeWidth': strokeWidth,
  };
}

class HorizontalLine extends DrawingObject {
  double price;

  HorizontalLine({
    required super.id,
    required this.price,
    super.color,
    super.strokeWidth,
  });

  @override
  void draw(Canvas canvas, Size size, double priceScale, double timeScale) {
    final paint = Paint()
      ..color = color
      ..strokeWidth = strokeWidth
      ..style = PaintingStyle.stroke;

    final y = (priceScale - price) * size.height / priceScale;

    canvas.drawLine(Offset(0, y), Offset(size.width, y), paint);
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'horizontalLine',
    'id': id,
    'price': price,
    'color': color.value,
    'strokeWidth': strokeWidth,
  };
}

class VerticalLine extends DrawingObject {
  DateTime time;

  VerticalLine({
    required super.id,
    required this.time,
    super.color,
    super.strokeWidth,
  });

  @override
  void draw(Canvas canvas, Size size, double priceScale, double timeScale) {
    final paint = Paint()
      ..color = color
      ..strokeWidth = strokeWidth
      ..style = PaintingStyle.stroke;

    final x = time.millisecondsSinceEpoch * timeScale;

    canvas.drawLine(Offset(x, 0), Offset(x, size.height), paint);
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'verticalLine',
    'id': id,
    'time': time.millisecondsSinceEpoch,
    'color': color.value,
    'strokeWidth': strokeWidth,
  };
}

class FibonacciRetracement extends DrawingObject {
  double highPrice;
  double lowPrice;

  FibonacciRetracement({
    required super.id,
    required this.highPrice,
    required this.lowPrice,
    super.color,
    super.strokeWidth,
  });

  static const List<double> levels = [
    0.0,
    0.236,
    0.382,
    0.5,
    0.618,
    0.786,
    1.0,
  ];
  static const List<String> labels = [
    '0%',
    '23.6%',
    '38.2%',
    '50%',
    '61.8%',
    '78.6%',
    '100%',
  ];

  @override
  void draw(Canvas canvas, Size size, double priceScale, double timeScale) {
    final range = highPrice - lowPrice;
    final textPainter = TextPainter(textDirection: TextDirection.ltr);

    for (int i = 0; i < levels.length; i++) {
      final level = levels[i];
      final price = highPrice - (range * level);
      final y = (priceScale - price) * size.height / priceScale;

      final paint = Paint()
        ..color = color.withOpacity(0.7)
        ..strokeWidth = strokeWidth
        ..style = PaintingStyle.stroke;

      canvas.drawLine(Offset(0, y), Offset(size.width, y), paint);

      textPainter.text = TextSpan(
        text: '${labels[i]}: \$${price.toStringAsFixed(2)}',
        style: TextStyle(color: color, fontSize: 10),
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(5, y - textPainter.height / 2));
    }
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'fibonacciRetracement',
    'id': id,
    'highPrice': highPrice,
    'lowPrice': lowPrice,
    'color': color.value,
    'strokeWidth': strokeWidth,
  };
}

class FibonacciExtension extends DrawingObject {
  double lowPrice;
  double highPrice;
  double extensionLevel;

  FibonacciExtension({
    required super.id,
    required this.lowPrice,
    required this.highPrice,
    this.extensionLevel = 1.618,
    super.color,
    super.strokeWidth,
  });

  static const List<double> levels = [
    0.0,
    0.618,
    1.0,
    1.618,
    2.0,
    2.618,
    3.618,
  ];
  static const List<String> labels = [
    '0%',
    '61.8%',
    '100%',
    '161.8%',
    '200%',
    '261.8%',
    '361.8%',
  ];

  @override
  void draw(Canvas canvas, Size size, double priceScale, double timeScale) {
    final range = highPrice - lowPrice;
    final textPainter = TextPainter(textDirection: TextDirection.ltr);

    for (int i = 0; i < levels.length; i++) {
      final level = levels[i];
      final price = lowPrice + (range * level);
      final y = (priceScale - price) * size.height / priceScale;

      final paint = Paint()
        ..color = color.withOpacity(0.7)
        ..strokeWidth = strokeWidth
        ..style = PaintingStyle.stroke;

      canvas.drawLine(Offset(0, y), Offset(size.width, y), paint);

      textPainter.text = TextSpan(
        text: '${labels[i]}: \$${price.toStringAsFixed(2)}',
        style: TextStyle(color: color, fontSize: 10),
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(5, y - textPainter.height / 2));
    }
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'fibonacciExtension',
    'id': id,
    'lowPrice': lowPrice,
    'highPrice': highPrice,
    'extensionLevel': extensionLevel,
    'color': color.value,
    'strokeWidth': strokeWidth,
  };
}

class RectangleDrawing extends DrawingObject {
  double topPrice;
  double bottomPrice;
  DateTime startTime;
  DateTime endTime;

  RectangleDrawing({
    required super.id,
    required this.topPrice,
    required this.bottomPrice,
    required this.startTime,
    required this.endTime,
    super.color,
    super.strokeWidth,
  });

  @override
  void draw(Canvas canvas, Size size, double priceScale, double timeScale) {
    final paint = Paint()
      ..color = color.withOpacity(0.2)
      ..style = PaintingStyle.fill;

    final strokePaint = Paint()
      ..color = color
      ..strokeWidth = strokeWidth
      ..style = PaintingStyle.stroke;

    final topY = (priceScale - topPrice) * size.height / priceScale;
    final bottomY = (priceScale - bottomPrice) * size.height / priceScale;
    final startX = startTime.millisecondsSinceEpoch * timeScale;
    final endX = endTime.millisecondsSinceEpoch * timeScale;

    final rect = Rect.fromLTRB(startX, topY, endX, bottomY);
    canvas.drawRect(rect, paint);
    canvas.drawRect(rect, strokePaint);
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'rectangle',
    'id': id,
    'topPrice': topPrice,
    'bottomPrice': bottomPrice,
    'startTime': startTime.millisecondsSinceEpoch,
    'endTime': endTime.millisecondsSinceEpoch,
    'color': color.value,
    'strokeWidth': strokeWidth,
  };
}

class TextAnnotation extends DrawingObject {
  double price;
  DateTime time;
  String text;

  TextAnnotation({
    required super.id,
    required this.price,
    required this.time,
    required this.text,
    super.color,
    super.strokeWidth,
  });

  @override
  void draw(Canvas canvas, Size size, double priceScale, double timeScale) {
    final textPainter = TextPainter(
      text: TextSpan(
        text: text,
        style: TextStyle(color: color, fontSize: 12),
      ),
      textDirection: TextDirection.ltr,
    );

    textPainter.layout();

    final x = time.millisecondsSinceEpoch * timeScale;
    final y = (priceScale - price) * size.height / priceScale;

    textPainter.paint(canvas, Offset(x, y));
  }

  @override
  Map<String, dynamic> toJson() => {
    'type': 'textAnnotation',
    'id': id,
    'price': price,
    'time': time.millisecondsSinceEpoch,
    'text': text,
    'color': color.value,
    'strokeWidth': strokeWidth,
  };
}

class DrawingToolManager extends ChangeNotifier {
  DrawingToolType _currentTool = DrawingToolType.none;
  List<DrawingObject> _drawings = [];
  DrawingObject? _activeDrawing;
  Color _currentColor = Colors.yellow;
  double _currentStrokeWidth = 1.0;

  DrawingToolType get currentTool => _currentTool;
  List<DrawingObject> get drawings => _drawings;
  DrawingObject? get activeDrawing => _activeDrawing;
  Color get currentColor => _currentColor;
  double get currentStrokeWidth => _currentStrokeWidth;

  void setTool(DrawingToolType tool) {
    _currentTool = tool;
    notifyListeners();
  }

  void setColor(Color color) {
    _currentColor = color;
    notifyListeners();
  }

  void setStrokeWidth(double width) {
    _currentStrokeWidth = width;
    notifyListeners();
  }

  void addDrawing(DrawingObject drawing) {
    _drawings.add(drawing);
    notifyListeners();
  }

  void removeDrawing(String id) {
    _drawings.removeWhere((d) => d.id == id);
    notifyListeners();
  }

  void clearDrawings() {
    _drawings.clear();
    notifyListeners();
  }

  void updateDrawing(DrawingObject drawing) {
    final index = _drawings.indexWhere((d) => d.id == drawing.id);
    if (index != -1) {
      _drawings[index] = drawing;
      notifyListeners();
    }
  }

  Map<String, dynamic> toJson() => {
    'drawings': _drawings.map((d) => d.toJson()).toList(),
    'currentColor': _currentColor.value,
    'currentStrokeWidth': _currentStrokeWidth,
  };
}

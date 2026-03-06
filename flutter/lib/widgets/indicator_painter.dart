import 'package:flutter/material.dart';
import 'dart:math' show sin;
import 'chart_enums.dart';

class IndicatorChartPainter extends CustomPainter {
  final IndicatorType indicator;
  final List<PriceData> data;
  final ChartTheme theme;

  IndicatorChartPainter({
    required this.indicator,
    required this.data,
    required this.theme,
  });

  @override
  void paint(Canvas canvas, Size size) {
    // Background
    final bgPaint = Paint()
      ..color = const Color(0xFF1E1E1E)
      ..style = PaintingStyle.fill;
    canvas.drawRect(Rect.fromLTWH(0, 0, size.width, size.height), bgPaint);

    if (data.isEmpty) return;

    switch (indicator) {
      case IndicatorType.rsi:
        _drawRSI(canvas, size);
        break;
      case IndicatorType.macd:
        _drawMACD(canvas, size);
        break;
      case IndicatorType.stochastic:
        _drawStochastic(canvas, size);
        break;
      default:
        _drawGenericIndicator(canvas, size);
    }

    // Draw overbought/oversold lines
    _drawThresholdLines(canvas, size);

    // Draw indicator name
    final textPainter = TextPainter(
      text: TextSpan(
        text: _getIndicatorName(),
        style: TextStyle(
          color: _getIndicatorColor(),
          fontSize: 10,
          fontWeight: FontWeight.bold,
        ),
      ),
      textDirection: TextDirection.ltr,
    );
    textPainter.layout();
    textPainter.paint(canvas, const Offset(8, 4));
  }

  void _drawRSI(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.purple
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.5;

    final path = Path();
    final spacing = (size.width - 50) / data.length;
    bool started = false;

    for (int i = 14; i < data.length; i++) {
      double rsi = _calculateRSI(data, i);
      final x = 50 + i * spacing + spacing / 2;
      final y = size.height - 10 - (rsi / 100) * (size.height - 20);

      if (!started) {
        path.moveTo(x, y);
        started = true;
      } else {
        path.lineTo(x, y);
      }
    }

    canvas.drawPath(path, paint);
  }

  void _drawMACD(Canvas canvas, Size size) {
    final spacing = (size.width - 50) / data.length;

    // Draw MACD line
    final macdPaint = Paint()
      ..color = Colors.blue
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.5;

    final macdPath = Path();
    bool started = false;

    for (int i = 26; i < data.length; i++) {
      double macd = _calculateMACD(data, i);
      final x = 50 + i * spacing + spacing / 2;
      final y = size.height / 2 - macd * 2;

      if (!started) {
        macdPath.moveTo(x, y);
        started = true;
      } else {
        macdPath.lineTo(x, y);
      }
    }
    canvas.drawPath(macdPath, macdPaint);

    // Draw Signal line
    final signalPaint = Paint()
      ..color = Colors.orange
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.5;

    final signalPath = Path();
    started = false;

    for (int i = 35; i < data.length; i++) {
      double signal = _calculateSignal(data, i);
      final x = 50 + i * spacing + spacing / 2;
      final y = size.height / 2 - signal * 2;

      if (!started) {
        signalPath.moveTo(x, y);
        started = true;
      } else {
        signalPath.lineTo(x, y);
      }
    }
    canvas.drawPath(signalPath, signalPaint);
  }

  void _drawStochastic(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.orange
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.5;

    final path = Path();
    final spacing = (size.width - 50) / data.length;
    bool started = false;

    for (int i = 14; i < data.length; i++) {
      double stoch = _calculateStochastic(data, i);
      final x = 50 + i * spacing + spacing / 2;
      final y = size.height - 10 - (stoch / 100) * (size.height - 20);

      if (!started) {
        path.moveTo(x, y);
        started = true;
      } else {
        path.lineTo(x, y);
      }
    }

    canvas.drawPath(path, paint);
  }

  void _drawGenericIndicator(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = _getIndicatorColor()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.0;

    final path = Path();
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final x = 50 + i * spacing + spacing / 2;
      final y = size.height / 2 + sin(i * 0.1) * size.height * 0.3;

      if (i == 0) {
        path.moveTo(x, y);
      } else {
        path.lineTo(x, y);
      }
    }

    canvas.drawPath(path, paint);
  }

  void _drawThresholdLines(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.grey.withOpacity(0.5)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 0.5;

    // Overbought line (70 for RSI, 80 for Stochastic)
    final overboughtY = size.height * 0.3;
    canvas.drawLine(
      Offset(50, overboughtY),
      Offset(size.width, overboughtY),
      paint,
    );

    // Oversold line (30 for RSI, 20 for Stochastic)
    final oversoldY = size.height * 0.7;
    canvas.drawLine(
      Offset(50, oversoldY),
      Offset(size.width, oversoldY),
      paint,
    );

    // Middle line (50)
    final middleY = size.height / 2;
    canvas.drawLine(
      Offset(50, middleY),
      Offset(size.width, middleY),
      paint..color = Colors.grey.withOpacity(0.3),
    );
  }

  double _calculateRSI(List<PriceData> data, int index) {
    if (index < 14) return 50;

    double gains = 0;
    double losses = 0;

    for (int i = index - 13; i <= index; i++) {
      final change = data[i].close - data[i - 1].close;
      if (change > 0) {
        gains += change;
      } else {
        losses += change.abs();
      }
    }

    final avgGain = gains / 14;
    final avgLoss = losses / 14;

    if (avgLoss == 0) return 100;

    final rs = avgGain / avgLoss;
    return 100 - (100 / (1 + rs));
  }

  double _calculateMACD(List<PriceData> data, int index) {
    if (index < 26) return 0;

    double ema12 = 0;
    double ema26 = 0;

    // Simple EMA calculation
    for (int i = index - 11; i <= index; i++) {
      ema12 += data[i].close;
    }
    ema12 /= 12;

    for (int i = index - 25; i <= index; i++) {
      ema26 += data[i].close;
    }
    ema26 /= 26;

    return ema12 - ema26;
  }

  double _calculateSignal(List<PriceData> data, int index) {
    if (index < 35) return 0;

    double sum = 0;
    for (int i = index - 8; i <= index; i++) {
      sum += _calculateMACD(data, i);
    }
    return sum / 9;
  }

  double _calculateStochastic(List<PriceData> data, int index) {
    if (index < 14) return 50;

    double lowestLow = double.infinity;
    double highestHigh = 0;

    for (int i = index - 13; i <= index; i++) {
      if (data[i].low < lowestLow) lowestLow = data[i].low;
      if (data[i].high > highestHigh) highestHigh = data[i].high;
    }

    final range = highestHigh - lowestLow;
    if (range == 0) return 50;

    return ((data[index].close - lowestLow) / range) * 100;
  }

  Color _getIndicatorColor() {
    switch (indicator) {
      case IndicatorType.rsi:
        return Colors.purple;
      case IndicatorType.macd:
        return Colors.blue;
      case IndicatorType.stochastic:
        return Colors.orange;
      case IndicatorType.williamsR:
        return Colors.red;
      default:
        return Colors.green;
    }
  }

  String _getIndicatorName() {
    switch (indicator) {
      case IndicatorType.rsi:
        return 'RSI (14)';
      case IndicatorType.macd:
        return 'MACD (12,26,9)';
      case IndicatorType.stochastic:
        return 'Stochastic (14,3)';
      case IndicatorType.williamsR:
        return 'Williams %R (14)';
      default:
        return 'Indicator';
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}

import 'package:flutter/material.dart';
import 'chart_enums.dart';
import 'drawing_tools.dart';

class ChartPainter extends CustomPainter {
  final ChartType chartType;
  final List<PriceData> data;
  final List<IndicatorType> indicators;
  final ChartTheme theme;
  final List<DrawingObject> drawings;
  final Map<String, List<PriceData>> comparisonData;
  final bool showComparison;

  ChartPainter({
    required this.chartType,
    required this.data,
    required this.indicators,
    required this.theme,
    this.drawings = const [],
    this.comparisonData = const {},
    this.showComparison = false,
  });

  @override
  void paint(Canvas canvas, Size size) {
    // Background
    final bgPaint = Paint()
      ..color = const Color(0xFF1E1E1E)
      ..style = PaintingStyle.fill;
    canvas.drawRect(Rect.fromLTWH(0, 0, size.width, size.height), bgPaint);

    if (data.isEmpty) return;

    // Calculate price range
    double minPrice = data.map((d) => d.low).reduce(min);
    double maxPrice = data.map((d) => d.high).reduce(max);
    double priceRange = maxPrice - minPrice;
    if (priceRange == 0) priceRange = 1;

    // Draw grid lines
    _drawGrid(canvas, size, minPrice, maxPrice);

    // Draw chart based on type
    switch (chartType) {
      case ChartType.candlestick:
        _drawCandlesticks(canvas, size, minPrice, priceRange);
        break;
      case ChartType.line:
        _drawLineChart(canvas, size, minPrice, priceRange);
        break;
      case ChartType.area:
        _drawAreaChart(canvas, size, minPrice, priceRange);
        break;
      case ChartType.ohlc:
        _drawOHLC(canvas, size, minPrice, priceRange);
        break;
      case ChartType.heikinAshi:
        _drawHeikinAshi(canvas, size, minPrice, priceRange);
        break;
      case ChartType.renko:
        _drawRenko(canvas, size, minPrice, priceRange);
        break;
      case ChartType.pointAndFigure:
        _drawPointAndFigure(canvas, size, minPrice, priceRange);
        break;
      case ChartType.kagi:
        _drawKagi(canvas, size, minPrice, priceRange);
        break;
    }

    // Draw indicators
    _drawIndicators(canvas, size, minPrice, priceRange);

    // Draw axes
    _drawAxes(canvas, size, minPrice, maxPrice);

    // Draw user drawings
    _drawUserDrawings(canvas, size, minPrice, maxPrice);

    // Draw comparison symbols
    if (showComparison && comparisonData.isNotEmpty) {
      _drawComparisonCharts(canvas, size, minPrice, maxPrice);
    }
  }

  void _drawComparisonCharts(
    Canvas canvas,
    Size size,
    double minPrice,
    double maxPrice,
  ) {
    if (data.isEmpty) return;

    final priceRange = maxPrice - minPrice;
    if (priceRange == 0) return;

    final candleWidth = (size.width - 50) / data.length;
    final comparisonColors = [
      Colors.cyan,
      Colors.orange,
      Colors.pink,
      Colors.lime,
      Colors.amber,
    ];
    int colorIndex = 0;

    for (final entry in comparisonData.entries) {
      final symbol = entry.key;
      final compData = entry.value;
      if (compData.isEmpty) continue;

      final color = comparisonColors[colorIndex % comparisonColors.length];
      colorIndex++;

      // Normalize comparison data to match the main chart's time range
      final normalizedData = _normalizeComparisonData(compData);
      if (normalizedData.isEmpty) continue;

      final linePaint = Paint()
        ..color = color
        ..strokeWidth = 2.0
        ..style = PaintingStyle.stroke;

      final path = Path();
      bool started = false;

      for (int i = 0; i < normalizedData.length && i < data.length; i++) {
        final x = 50 + i * candleWidth + candleWidth / 2;

        // Normalize price to percentage of range
        final normalizedPrice = normalizedData[i];
        final y = size.height - 20 - (normalizedPrice * (size.height - 40));

        if (!started) {
          path.moveTo(x, y);
          started = true;
        } else {
          path.lineTo(x, y);
        }
      }

      canvas.drawPath(path, linePaint);

      // Draw legend
      final textPainter = TextPainter(
        text: TextSpan(
          text: symbol,
          style: TextStyle(
            color: color,
            fontSize: 10,
            fontWeight: FontWeight.bold,
          ),
        ),
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(size.width - 50 - (colorIndex * 60), 5));
    }
  }

  List<double> _normalizeComparisonData(List<PriceData> compData) {
    if (compData.isEmpty) return [];

    // Get the first and last close prices for normalization
    final firstClose = compData.first.close;
    final lastClose = compData.last.close;

    if (firstClose == 0) return [];

    // Normalize to percentage change from first price
    return compData.map((d) => (d.close - firstClose) / firstClose).toList();
  }

  void _drawUserDrawings(
    Canvas canvas,
    Size size,
    double minPrice,
    double maxPrice,
  ) {
    if (drawings.isEmpty) return;

    final priceRange = maxPrice - minPrice;
    if (priceRange == 0) return;

    final firstTime = data.isNotEmpty
        ? data.first.timestamp.millisecondsSinceEpoch
        : 0;
    final lastTime = data.isNotEmpty
        ? data.last.timestamp.millisecondsSinceEpoch
        : 1;
    final timeRange = lastTime - firstTime;

    for (final drawing in drawings) {
      if (!drawing.visible) continue;

      switch (drawing) {
        case Trendline trendline:
          _drawTrendline(
            canvas,
            size,
            trendline,
            minPrice,
            priceRange,
            firstTime,
            timeRange,
          );
          break;
        case HorizontalLine hLine:
          _drawHorizontalLine(canvas, size, hLine, minPrice, priceRange);
          break;
        case VerticalLine vLine:
          _drawVerticalLine(canvas, size, vLine, firstTime, timeRange);
          break;
        case FibonacciRetracement fib:
          _drawFibonacci(canvas, size, fib, minPrice, priceRange);
          break;
        case FibonacciExtension fibExt:
          _drawFibonacci(canvas, size, fibExt, minPrice, priceRange);
          break;
        case RectangleDrawing rect:
          _drawRectangle(
            canvas,
            size,
            rect,
            minPrice,
            priceRange,
            firstTime,
            timeRange,
          );
          break;
        case TextAnnotation textAnn:
          _drawTextAnnotation(
            canvas,
            size,
            textAnn,
            minPrice,
            priceRange,
            firstTime,
            timeRange,
          );
          break;
      }
    }
  }

  void _drawTrendline(
    Canvas canvas,
    Size size,
    Trendline line,
    double minPrice,
    double priceRange,
    double firstTime,
    double timeRange,
  ) {
    final paint = Paint()
      ..color = line.color
      ..strokeWidth = line.strokeWidth
      ..style = PaintingStyle.stroke;

    final startX = timeRange > 0
        ? ((line.startTime.millisecondsSinceEpoch - firstTime) / timeRange) *
                  (size.width - 50) +
              50
        : 50;
    final endX = timeRange > 0
        ? ((line.endTime.millisecondsSinceEpoch - firstTime) / timeRange) *
                  (size.width - 50) +
              50
        : size.width;
    final startY =
        ((priceRange - (line.startPrice - minPrice)) / priceRange) *
        size.height;
    final endY =
        ((priceRange - (line.endPrice - minPrice)) / priceRange) * size.height;

    canvas.drawLine(Offset(startX, startY), Offset(endX, endY), paint);

    final dotPaint = Paint()
      ..color = line.color
      ..style = PaintingStyle.fill;
    canvas.drawCircle(Offset(startX, startY), 4, dotPaint);
    canvas.drawCircle(Offset(endX, endY), 4, dotPaint);
  }

  void _drawHorizontalLine(
    Canvas canvas,
    Size size,
    HorizontalLine line,
    double minPrice,
    double priceRange,
  ) {
    final paint = Paint()
      ..color = line.color
      ..strokeWidth = line.strokeWidth
      ..style = PaintingStyle.stroke;

    final y =
        ((priceRange - (line.price - minPrice)) / priceRange) * size.height;
    canvas.drawLine(Offset(50, y), Offset(size.width, y), paint);
  }

  void _drawVerticalLine(
    Canvas canvas,
    Size size,
    VerticalLine line,
    double firstTime,
    double timeRange,
  ) {
    final paint = Paint()
      ..color = line.color
      ..strokeWidth = line.strokeWidth
      ..style = PaintingStyle.stroke;

    final x = timeRange > 0
        ? ((line.time.millisecondsSinceEpoch - firstTime) / timeRange) *
                  (size.width - 50) +
              50
        : 50;
    canvas.drawLine(Offset(x, 0), Offset(x, size.height), paint);
  }

  void _drawFibonacci(
    Canvas canvas,
    Size size,
    dynamic fib,
    double minPrice,
    double priceRange,
  ) {
    List<double> levels;
    if (fib is FibonacciRetracement) {
      levels = FibonacciRetracement.levels;
    } else {
      levels = FibonacciExtension.levels;
    }

    final textPainter = TextPainter(textDirection: TextDirection.ltr);

    for (int i = 0; i < levels.length; i++) {
      final price = fib is FibonacciRetracement
          ? fib.highPrice - (fib.highPrice - fib.lowPrice) * levels[i]
          : fib.lowPrice + (fib.highPrice - fib.lowPrice) * levels[i];
      final y = ((priceRange - (price - minPrice)) / priceRange) * size.height;

      final paint = Paint()
        ..color = fib.color.withOpacity(0.7)
        ..strokeWidth = fib.strokeWidth
        ..style = PaintingStyle.stroke;

      canvas.drawLine(Offset(50, y), Offset(size.width, y), paint);

      final label = fib is FibonacciRetracement
          ? FibonacciRetracement.labels[i]
          : FibonacciExtension.labels[i];
      textPainter.text = TextSpan(
        text: '$label: \$${price.toStringAsFixed(2)}',
        style: TextStyle(color: fib.color, fontSize: 10),
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(5, y - textPainter.height / 2));
    }
  }

  void _drawRectangle(
    Canvas canvas,
    Size size,
    RectangleDrawing rect,
    double minPrice,
    double priceRange,
    double firstTime,
    double timeRange,
  ) {
    final fillPaint = Paint()
      ..color = rect.color.withOpacity(0.2)
      ..style = PaintingStyle.fill;
    final strokePaint = Paint()
      ..color = rect.color
      ..strokeWidth = rect.strokeWidth
      ..style = PaintingStyle.stroke;

    final topY =
        ((priceRange - (rect.topPrice - minPrice)) / priceRange) * size.height;
    final bottomY =
        ((priceRange - (rect.bottomPrice - minPrice)) / priceRange) *
        size.height;
    final startX = timeRange > 0
        ? ((rect.startTime.millisecondsSinceEpoch - firstTime) / timeRange) *
                  (size.width - 50) +
              50
        : 50;
    final endX = timeRange > 0
        ? ((rect.endTime.millisecondsSinceEpoch - firstTime) / timeRange) *
                  (size.width - 50) +
              50
        : size.width;

    final rectToDraw = Rect.fromLTRB(startX, topY, endX, bottomY);
    canvas.drawRect(rectToDraw, fillPaint);
    canvas.drawRect(rectToDraw, strokePaint);
  }

  void _drawTextAnnotation(
    Canvas canvas,
    Size size,
    TextAnnotation ann,
    double minPrice,
    double priceRange,
    double firstTime,
    double timeRange,
  ) {
    final textPainter = TextPainter(
      text: TextSpan(
        text: ann.text,
        style: TextStyle(color: ann.color, fontSize: 12),
      ),
      textDirection: TextDirection.ltr,
    );
    textPainter.layout();

    final x = timeRange > 0
        ? ((ann.time.millisecondsSinceEpoch - firstTime) / timeRange) *
                  (size.width - 50) +
              50
        : 50;
    final y =
        ((priceRange - (ann.price - minPrice)) / priceRange) * size.height;

    textPainter.paint(canvas, Offset(x, y));
  }

  void _drawGrid(Canvas canvas, Size size, double minPrice, double maxPrice) {
    final gridPaint = Paint()
      ..color = Colors.grey.withOpacity(0.2)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 0.5;

    // Horizontal grid lines
    for (int i = 0; i <= 5; i++) {
      final y = size.height * i / 5;
      canvas.drawLine(Offset(50, y), Offset(size.width, y), gridPaint);
    }

    // Vertical grid lines
    final candleWidth = (size.width - 50) / data.length;
    for (int i = 0; i <= 10; i++) {
      final x = 50 + (size.width - 50) * i / 10;
      canvas.drawLine(Offset(x, 0), Offset(x, size.height - 20), gridPaint);
    }
  }

  void _drawCandlesticks(
    Canvas canvas,
    Size size,
    double minPrice,
    double priceRange,
  ) {
    final candleWidth = (size.width - 50) / data.length * 0.8;
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;

      // Calculate y positions
      final highY =
          size.height -
          20 -
          ((d.high - minPrice) / priceRange) * (size.height - 40);
      final lowY =
          size.height -
          20 -
          ((d.low - minPrice) / priceRange) * (size.height - 40);
      final openY =
          size.height -
          20 -
          ((d.open - minPrice) / priceRange) * (size.height - 40);
      final closeY =
          size.height -
          20 -
          ((d.close - minPrice) / priceRange) * (size.height - 40);

      final isGreen = d.close >= d.open;
      final color = isGreen ? Colors.green : Colors.red;

      // Draw wick
      final wickPaint = Paint()
        ..color = color
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1;
      canvas.drawLine(Offset(x, highY), Offset(x, lowY), wickPaint);

      // Draw body
      final bodyPaint = Paint()
        ..color = color
        ..style = isGreen ? PaintingStyle.fill : PaintingStyle.fill;

      final bodyTop = min(openY, closeY);
      final bodyBottom = max(openY, closeY);
      final bodyHeight = max(bodyBottom - bodyTop, 1);

      canvas.drawRect(
        Rect.fromLTWH(x - candleWidth / 2, bodyTop, candleWidth, bodyHeight),
        bodyPaint,
      );

      // Draw border for green candles
      if (isGreen) {
        final borderPaint = Paint()
          ..color = color
          ..style = PaintingStyle.stroke
          ..strokeWidth = 1;
        canvas.drawRect(
          Rect.fromLTWH(x - candleWidth / 2, bodyTop, candleWidth, bodyHeight),
          borderPaint,
        );
      }
    }
  }

  void _drawLineChart(
    Canvas canvas,
    Size size,
    double minPrice,
    double priceRange,
  ) {
    final linePaint = Paint()
      ..color = Colors.blue
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2;

    final path = Path();
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;
      final y =
          size.height -
          20 -
          ((d.close - minPrice) / priceRange) * (size.height - 40);

      if (i == 0) {
        path.moveTo(x, y);
      } else {
        path.lineTo(x, y);
      }
    }

    canvas.drawPath(path, linePaint);

    // Draw dots at data points
    final dotPaint = Paint()
      ..color = Colors.blue
      ..style = PaintingStyle.fill;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;
      final y =
          size.height -
          20 -
          ((d.close - minPrice) / priceRange) * (size.height - 40);

      if (i % 10 == 0) {
        canvas.drawCircle(Offset(x, y), 3, dotPaint);
      }
    }
  }

  void _drawAreaChart(
    Canvas canvas,
    Size size,
    double minPrice,
    double priceRange,
  ) {
    final fillPaint = Paint()
      ..color = Colors.blue.withOpacity(0.3)
      ..style = PaintingStyle.fill;

    final linePaint = Paint()
      ..color = Colors.blue
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2;

    final path = Path();
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;
      final y =
          size.height -
          20 -
          ((d.close - minPrice) / priceRange) * (size.height - 40);

      if (i == 0) {
        path.moveTo(x, y);
      } else {
        path.lineTo(x, y);
      }
    }

    // Close the path for filling
    path.lineTo(
      50 + (data.length - 1) * spacing + spacing / 2,
      size.height - 20,
    );
    path.lineTo(50 + spacing / 2, size.height - 20);
    path.close();

    canvas.drawPath(path, fillPaint);

    // Draw the line again
    final linePath = Path();
    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;
      final y =
          size.height -
          20 -
          ((d.close - minPrice) / priceRange) * (size.height - 40);

      if (i == 0) {
        linePath.moveTo(x, y);
      } else {
        linePath.lineTo(x, y);
      }
    }
    canvas.drawPath(linePath, linePaint);
  }

  void _drawOHLC(Canvas canvas, Size size, double minPrice, double priceRange) {
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;

      final highY =
          size.height -
          20 -
          ((d.high - minPrice) / priceRange) * (size.height - 40);
      final lowY =
          size.height -
          20 -
          ((d.low - minPrice) / priceRange) * (size.height - 40);
      final openY =
          size.height -
          20 -
          ((d.open - minPrice) / priceRange) * (size.height - 40);
      final closeY =
          size.height -
          20 -
          ((d.close - minPrice) / priceRange) * (size.height - 40);

      final isGreen = d.close >= d.open;
      final color = isGreen ? Colors.green : Colors.red;

      final paint = Paint()
        ..color = color
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1.5;

      // Draw high-low line
      canvas.drawLine(Offset(x, highY), Offset(x, lowY), paint);

      // Draw open tick
      canvas.drawLine(Offset(x - 5, openY), Offset(x, openY), paint);

      // Draw close tick
      canvas.drawLine(Offset(x, closeY), Offset(x + 5, closeY), paint);
    }
  }

  void _drawIndicators(
    Canvas canvas,
    Size size,
    double minPrice,
    double priceRange,
  ) {
    final spacing = (size.width - 50) / data.length;

    for (final indicator in indicators) {
      if (indicator == IndicatorType.sma20) {
        _drawSMA(
          canvas,
          size,
          data,
          20,
          Colors.yellow,
          minPrice,
          priceRange,
          spacing,
        );
      } else if (indicator == IndicatorType.sma50) {
        _drawSMA(
          canvas,
          size,
          data,
          50,
          Colors.orange,
          minPrice,
          priceRange,
          spacing,
        );
      }
    }
  }

  void _drawSMA(
    Canvas canvas,
    Size size,
    List<PriceData> data,
    int period,
    Color color,
    double minPrice,
    double priceRange,
    double spacing,
  ) {
    if (data.length < period) return;

    final paint = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.5;

    final path = Path();
    bool started = false;

    for (int i = period - 1; i < data.length; i++) {
      double sum = 0;
      for (int j = 0; j < period; j++) {
        sum += data[i - j].close;
      }
      final sma = sum / period;

      final x = 50 + i * spacing + spacing / 2;
      final y =
          size.height -
          20 -
          ((sma - minPrice) / priceRange) * (size.height - 40);

      if (!started) {
        path.moveTo(x, y);
        started = true;
      } else {
        path.lineTo(x, y);
      }
    }

    canvas.drawPath(path, paint);
  }

  void _drawAxes(Canvas canvas, Size size, double minPrice, double maxPrice) {
    final textPaint = Paint()..color = Colors.grey;

    // Y-axis labels (price)
    for (int i = 0; i <= 5; i++) {
      final price = minPrice + (maxPrice - minPrice) * i / 5;
      final y = size.height - 20 - (size.height - 40) * i / 5;

      final textPainter = TextPainter(
        text: TextSpan(
          text: price.toStringAsFixed(2),
          style: const TextStyle(color: Colors.grey, fontSize: 10),
        ),
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(5, y - textPainter.height / 2));
    }

    // X-axis labels (time)
    for (int i = 0; i < data.length; i += data.length ~/ 5) {
      final x = 50 + (size.width - 50) * i / data.length;
      final time =
          '${data[i].timestamp.hour}:${data[i].timestamp.minute.toString().padLeft(2, '0')}';

      final textPainter = TextPainter(
        text: TextSpan(
          text: time,
          style: const TextStyle(color: Colors.grey, fontSize: 9),
        ),
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();
      textPainter.paint(
        canvas,
        Offset(x - textPainter.width / 2, size.height - 18),
      );
    }
  }

  void _drawHeikinAshi(
    Canvas canvas,
    Size size,
    double minPrice,
    double priceRange,
  ) {
    final candleWidth = (size.width - 50) / data.length * 0.8;
    final spacing = (size.width - 50) / data.length;

    // Calculate Heikin Ashi data
    final haData = <PriceData>[];

    for (int i = 0; i < data.length; i++) {
      final current = data[i];

      if (i == 0) {
        // First candle uses regular data
        haData.add(
          PriceData(
            timestamp: current.timestamp,
            open: (current.open + current.close) / 2,
            high: current.high,
            low: current.low,
            close:
                (current.open + current.high + current.low + current.close) / 4,
            volume: current.volume,
          ),
        );
      } else {
        final prevHA = haData[i - 1];
        final haOpen = (prevHA.open + prevHA.close) / 2;
        final haClose =
            (current.open + current.high + current.low + current.close) / 4;
        final haHigh = max(max(current.high, haOpen), haClose);
        final haLow = min(min(current.low, haOpen), haClose);

        haData.add(
          PriceData(
            timestamp: current.timestamp,
            open: haOpen,
            high: haHigh,
            low: haLow,
            close: haClose,
            volume: current.volume,
          ),
        );
      }
    }

    // Draw Heikin Ashi candlesticks
    for (int i = 0; i < haData.length; i++) {
      final d = haData[i];
      final x = 50 + i * spacing + spacing / 2;

      final highY =
          size.height -
          20 -
          ((d.high - minPrice) / priceRange) * (size.height - 40);
      final lowY =
          size.height -
          20 -
          ((d.low - minPrice) / priceRange) * (size.height - 40);
      final openY =
          size.height -
          20 -
          ((d.open - minPrice) / priceRange) * (size.height - 40);
      final closeY =
          size.height -
          20 -
          ((d.close - minPrice) / priceRange) * (size.height - 40);

      final isGreen = d.close >= d.open;
      final color = isGreen ? Colors.green : Colors.red;

      // Draw wick
      final wickPaint = Paint()
        ..color = color
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1;
      canvas.drawLine(Offset(x, highY), Offset(x, lowY), wickPaint);

      // Draw body
      final bodyPaint = Paint()
        ..color = color
        ..style = isGreen ? PaintingStyle.fill : PaintingStyle.fill;

      final bodyTop = min(openY, closeY);
      final bodyBottom = max(openY, closeY);
      final bodyHeight = max(bodyBottom - bodyTop, 1);

      canvas.drawRect(
        Rect.fromLTWH(x - candleWidth / 2, bodyTop, candleWidth, bodyHeight),
        bodyPaint,
      );

      // Draw border for green candles
      if (isGreen) {
        final borderPaint = Paint()
          ..color = color
          ..style = PaintingStyle.stroke
          ..strokeWidth = 1;
        canvas.drawRect(
          Rect.fromLTWH(x - candleWidth / 2, bodyTop, candleWidth, bodyHeight),
          borderPaint,
        );
      }
    }
  }

  void _drawRenko(
    Canvas canvas,
    Size size,
    double minPrice,
    double priceRange,
  ) {
    final brickSize = (size.width - 50) / 50; // Show 50 bricks max
    final renkoData = _calculateRenkoBricks();

    for (int i = 0; i < renkoData.length; i++) {
      final brick = renkoData[i];
      final x = 50 + i * brickSize;
      final y =
          size.height -
          20 -
          ((brick.price - minPrice) / priceRange) * (size.height - 40);

      final paint = Paint()
        ..color = brick.isBullish ? Colors.green : Colors.red
        ..style = PaintingStyle.fill;

      canvas.drawRect(
        Rect.fromLTWH(x, y - brickSize, brickSize * 0.9, brickSize),
        paint,
      );
    }
  }

  void _drawPointAndFigure(
    Canvas canvas,
    Size size,
    double minPrice,
    double priceRange,
  ) {
    final boxSize = priceRange * 0.02; // 2% box size
    final reversalAmount = boxSize * 3; // 3-box reversal
    final pnfData = _calculatePointAndFigure(boxSize, reversalAmount);

    final cellWidth = (size.width - 50) / 30; // Max 30 columns
    final cellHeight = (size.height - 40) / 30; // Max 30 rows

    for (int i = 0; i < pnfData.length; i++) {
      final point = pnfData[i];
      final x = 50 + point.column * cellWidth;
      final y =
          size.height -
          20 -
          ((point.price - minPrice) / priceRange) * (size.height - 40);

      final paint = Paint()
        ..color = point.isX ? Colors.green : Colors.red
        ..style = PaintingStyle.fill;

      if (point.isX) {
        // Draw X
        canvas.drawLine(
          Offset(x, y - cellHeight / 2),
          Offset(x + cellWidth, y + cellHeight / 2),
          paint..strokeWidth = 2,
        );
        canvas.drawLine(
          Offset(x + cellWidth, y - cellHeight / 2),
          Offset(x, y + cellHeight / 2),
          paint,
        );
      } else {
        // Draw O
        final center = Offset(x + cellWidth / 2, y);
        canvas.drawCircle(center, cellWidth / 3, paint);
      }
    }
  }

  void _drawKagi(Canvas canvas, Size size, double minPrice, double priceRange) {
    final kagiData = _calculateKagiLines();
    final spacing = (size.width - 50) / max(kagiData.length.toDouble(), 1);

    Path kagiPath = Path();
    bool started = false;

    for (int i = 0; i < kagiData.length; i++) {
      final point = kagiData[i];
      final x = 50 + i * spacing;
      final y =
          size.height -
          20 -
          ((point.price - minPrice) / priceRange) * (size.height - 40);

      if (!started) {
        kagiPath.moveTo(x, y);
        started = true;
      } else {
        if (point.isVertical) {
          // Vertical line
          final lastPoint = kagiData[i - 1];
          final lastX = 50 + (i - 1) * spacing;
          kagiPath.lineTo(lastX, y);
        } else {
          // Horizontal line
          kagiPath.lineTo(x, y);
        }
      }

      // Store the current point for thickness calculation
      if (i == kagiData.length - 1 ||
          point.isVertical != kagiData[i + 1].isVertical) {
        // Draw the segment
        final paint = Paint()
          ..color = point.isBullish ? Colors.green : Colors.red
          ..style = PaintingStyle.stroke
          ..strokeWidth = point.isBullish ? 3 : 2;

        canvas.drawPath(kagiPath, paint);
        kagiPath = Path();
        kagiPath.moveTo(x, y);
        started = false;
      }
    }
  }

  // Helper methods for advanced chart calculations

  List<RenkoBrick> _calculateRenkoBricks() {
    if (data.isEmpty) return [];

    final bricks = <RenkoBrick>[];
    final brickSize =
        (data.first.high - data.first.low) * 0.01; // 1% brick size
    double currentPrice = data.first.close;

    for (int i = 1; i < data.length; i++) {
      final price = data[i].close;

      if (price > currentPrice + brickSize) {
        // Up bricks
        final numBricks = ((price - currentPrice) / brickSize).floor();
        for (int j = 0; j < numBricks; j++) {
          currentPrice += brickSize;
          bricks.add(RenkoBrick(currentPrice, true));
        }
      } else if (price < currentPrice - brickSize) {
        // Down bricks
        final numBricks = ((currentPrice - price) / brickSize).floor();
        for (int j = 0; j < numBricks; j++) {
          currentPrice -= brickSize;
          bricks.add(RenkoBrick(currentPrice, false));
        }
      }
    }

    return bricks;
  }

  List<PointAndFigurePoint> _calculatePointAndFigure(
    double boxSize,
    double reversalAmount,
  ) {
    if (data.isEmpty) return [];

    final points = <PointAndFigurePoint>[];
    double currentPrice = data.first.close;
    bool isUpTrend = true;
    int column = 0;

    for (int i = 1; i < data.length; i++) {
      final price = data[i].close;
      final priceChange = price - currentPrice;

      if (isUpTrend && priceChange >= boxSize) {
        // Add X's
        final numBoxes = (priceChange / boxSize).floor();
        for (int j = 0; j < numBoxes; j++) {
          currentPrice += boxSize;
          points.add(PointAndFigurePoint(currentPrice, column, true));
        }
      } else if (!isUpTrend && -priceChange >= boxSize) {
        // Add O's
        final numBoxes = (-priceChange / boxSize).floor();
        for (int j = 0; j < numBoxes; j++) {
          currentPrice -= boxSize;
          points.add(PointAndFigurePoint(currentPrice, column, false));
        }
      } else if ((isUpTrend && -priceChange >= reversalAmount) ||
          (!isUpTrend && priceChange >= reversalAmount)) {
        // Reversal
        isUpTrend = !isUpTrend;
        column++;
        i--; // Re-process this price with new trend
      }
    }

    return points;
  }

  List<KagiPoint> _calculateKagiLines() {
    if (data.isEmpty) return [];

    final points = <KagiPoint>[];
    double currentPrice = data.first.close;
    double previousHigh = data.first.high;
    double previousLow = data.first.low;
    bool isBullish = true;
    bool isVertical = false;

    points.add(KagiPoint(currentPrice, isBullish, isVertical));

    for (int i = 1; i < data.length; i++) {
      final candle = data[i];

      if (isBullish) {
        if (candle.high > previousHigh) {
          // Continue uptrend
          currentPrice = candle.high;
          previousHigh = candle.high;
          isVertical = true;
          points.add(KagiPoint(currentPrice, isBullish, isVertical));
        } else if (candle.low <
            previousLow - (previousHigh - previousLow) * 0.03) {
          // Reversal (more than 3% below low)
          isBullish = false;
          currentPrice = candle.low;
          isVertical = true;
          points.add(KagiPoint(currentPrice, isBullish, isVertical));
        }
      } else {
        if (candle.low < previousLow) {
          // Continue downtrend
          currentPrice = candle.low;
          previousLow = candle.low;
          isVertical = true;
          points.add(KagiPoint(currentPrice, isBullish, isVertical));
        } else if (candle.high >
            previousHigh + (previousHigh - previousLow) * 0.03) {
          // Reversal (more than 3% above high)
          isBullish = true;
          currentPrice = candle.high;
          isVertical = true;
          points.add(KagiPoint(currentPrice, isBullish, isVertical));
        }
      }

      previousHigh = max(previousHigh, candle.high);
      previousLow = min(previousLow, candle.low);
    }

    return points;
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}

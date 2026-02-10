import 'package:flutter/material.dart';
import 'dart:math';

class AdvancedChartingWidget extends StatefulWidget {
  final String symbol;
  final String timeframe;
  final Function(String)? onTimeframeChanged;
  final Map<String, dynamic>? chartSettings;

  const AdvancedChartingWidget({
    super.key,
    required this.symbol,
    this.timeframe = '1D',
    this.onTimeframeChanged,
    this.chartSettings,
  });

  @override
  State<AdvancedChartingWidget> createState() => _AdvancedChartingWidgetState();
}

class _AdvancedChartingWidgetState extends State<AdvancedChartingWidget> {
  ChartType _currentChartType = ChartType.candlestick;
  List<IndicatorType> _activeIndicators = [];
  bool _showVolume = true;
  ChartTheme _theme = ChartTheme.dark;
  String _currentTimeframe = '1D';
  
  @override
  void initState() {
    super.initState();
    _currentTimeframe = widget.timeframe;
    _initializeDefaultIndicators();
  }

  @override
  void didUpdateWidget(AdvancedChartingWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.timeframe != widget.timeframe) {
      setState(() {
        _currentTimeframe = widget.timeframe;
      });
    }
  }

  void _initializeDefaultIndicators() {
    _activeIndicators = [
      IndicatorType.sma20,
      IndicatorType.sma50,
      IndicatorType.rsi,
    ];
  }

  void _onTimeframeChanged(String? newTimeframe) {
    if (newTimeframe != null && newTimeframe != _currentTimeframe) {
      setState(() {
        _currentTimeframe = newTimeframe;
      });
      
      // Notify parent widget
      if (widget.onTimeframeChanged != null) {
        widget.onTimeframeChanged!(newTimeframe);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.all(8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF263238),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Chart Header
          _buildChartHeader(),
          const SizedBox(height: 12),
          // Chart Controls
          _buildChartControls(),
          const SizedBox(height: 12),
          // Main Chart Area
          Expanded(
            flex: 3,
            child: _buildMainChart(),
          ),
          const SizedBox(height: 8),
          // Volume Chart
          if (_showVolume)
            Expanded(
              flex: 1,
              child: _buildVolumeChart(),
            ),
          // Indicator Charts
          ..._buildIndicatorCharts(),
        ],
      ),
    );
  }

  Widget _buildChartHeader() {
    return Row(
      children: [
        Icon(
          Icons.show_chart,
          color: Colors.white,
          size: 20,
        ),
        const SizedBox(width: 8),
        Text(
          '${widget.symbol} Advanced Chart',
          style: const TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const Spacer(),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
          decoration: BoxDecoration(
            color: Colors.blue.withOpacity(0.3),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.blue.withOpacity(0.5)),
          ),
          child: Text(
            _currentTimeframe,
            style: const TextStyle(
              color: Colors.blue,
              fontSize: 12,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        const SizedBox(width: 8),
        PopupMenuButton<ChartType>(
          icon: const Icon(Icons.more_vert, color: Colors.white),
          onSelected: (ChartType type) {
            setState(() {
              _currentChartType = type;
            });
          },
          itemBuilder: (context) => ChartType.values.map((type) {
            return PopupMenuItem(
              value: type,
              child: Row(
                children: [
                  Icon(_getChartTypeIcon(type), size: 16),
                  const SizedBox(width: 8),
                  Text(_getChartTypeName(type)),
                ],
              ),
            );
          }).toList(),
        ),
      ],
    );
  }

  Widget _buildChartControls() {
    return Row(
      children: [
        // Chart Type Selector
        SizedBox(
          height: 32,
          child: ListView.builder(
            scrollDirection: Axis.horizontal,
            itemCount: ChartType.values.length,
            itemBuilder: (context, index) {
              final type = ChartType.values[index];
              final isSelected = _currentChartType == type;
              
              return Padding(
                padding: const EdgeInsets.only(right: 4),
                child: FilterChip(
                  label: Text(_getChartTypeShortName(type)),
                  selected: isSelected,
                  onSelected: (selected) {
                    if (selected) {
                      setState(() {
                        _currentChartType = type;
                      });
                    }
                  },
                  backgroundColor: const Color(0xFF37474F),
                  selectedColor: Colors.blue,
                  labelStyle: TextStyle(
                    color: isSelected ? Colors.white : Colors.grey,
                    fontSize: 12,
                  ),
                ),
              );
            },
          ),
        ),
        const Spacer(),
        // Time Frame Selector
        DropdownButton<String>(
          value: _currentTimeframe,
          dropdownColor: const Color(0xFF37474F),
          style: const TextStyle(color: Colors.white, fontSize: 12),
          items: ['1m', '5m', '15m', '1h', '4h', '1D', '1W', '1M'].map((timeframe) {
            return DropdownMenuItem(
              value: timeframe,
              child: Text(timeframe),
            );
          }).toList(),
          onChanged: _onTimeframeChanged,
        ),
        const SizedBox(width: 8),
        // Volume Toggle
        FilterChip(
          label: const Text('Volume', style: TextStyle(fontSize: 12)),
          selected: _showVolume,
          onSelected: (selected) {
            setState(() {
              _showVolume = selected;
            });
          },
          backgroundColor: const Color(0xFF37474F),
          selectedColor: Colors.green,
          labelStyle: TextStyle(
            color: _showVolume ? Colors.white : Colors.grey,
            fontSize: 12,
          ),
        ),
      ],
    );
  }

  Widget _buildMainChart() {
    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF1E1E1E),
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: Colors.grey.shade700),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(4),
        child: CustomPaint(
          size: Size.infinite,
          painter: ChartPainter(
            chartType: _currentChartType,
            data: _generateSampleData(),
            indicators: _activeIndicators,
            theme: _theme,
          ),
        ),
      ),
    );
  }

  Widget _buildVolumeChart() {
    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF1E1E1E),
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: Colors.grey.shade700),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(4),
        child: CustomPaint(
          size: Size.infinite,
          painter: VolumeChartPainter(
            data: _generateSampleData(),
            theme: _theme,
          ),
        ),
      ),
    );
  }

  List<Widget> _buildIndicatorCharts() {
    final indicatorCharts = <Widget>[];
    
    for (final indicator in _activeIndicators) {
      if (_indicatorNeedsChart(indicator)) {
        indicatorCharts.add(
          Padding(
            padding: const EdgeInsets.only(top: 8),
            child: Container(
              height: 60,
              decoration: BoxDecoration(
                color: const Color(0xFF1E1E1E),
                borderRadius: BorderRadius.circular(4),
                border: Border.all(color: Colors.grey.shade700),
              ),
              child: ClipRRect(
                borderRadius: BorderRadius.circular(4),
                child: CustomPaint(
                  size: Size.infinite,
                  painter: IndicatorChartPainter(
                    indicator: indicator,
                    data: _generateSampleData(),
                    theme: _theme,
                  ),
                ),
              ),
            ),
          ),
        );
      }
    }
    
    return indicatorCharts;
  }

  List<PriceData> _generateSampleData() {
    final random = Random();
    final data = <PriceData>[];
    final now = DateTime.now();
    
    double currentPrice = 150.0;
    
    for (int i = 0; i < 100; i++) {
      final change = (random.nextDouble() - 0.5) * 2.0;
      currentPrice += change;
      
      final open = currentPrice;
      final close = currentPrice + (random.nextDouble() - 0.5) * 1.0;
      final high = max(open, close) + random.nextDouble() * 0.5;
      final low = min(open, close) - random.nextDouble() * 0.5;
      final volume = 1000000 + random.nextInt(2000000);
      
      data.add(PriceData(
        timestamp: now.subtract(Duration(minutes: (100 - i) * 5)),
        open: open,
        high: high,
        low: low,
        close: close,
        volume: volume,
      ));
    }
    
    return data;
  }

  IconData _getChartTypeIcon(ChartType type) {
    switch (type) {
      case ChartType.candlestick:
        return Icons.candlestick_chart;
      case ChartType.line:
        return Icons.show_chart;
      case ChartType.area:
        return Icons.area_chart;
      case ChartType.ohlc:
        return Icons.bar_chart;
      case ChartType.heikinAshi:
        return Icons.fiber_smart_record;
      case ChartType.renko:
        return Icons.view_module;
      case ChartType.pointAndFigure:
        return Icons.scatter_plot;
      case ChartType.kagi:
        return Icons.timeline;
    }
  }

  String _getChartTypeName(ChartType type) {
    switch (type) {
      case ChartType.candlestick:
        return 'Candlestick';
      case ChartType.line:
        return 'Line';
      case ChartType.area:
        return 'Area';
      case ChartType.ohlc:
        return 'OHLC';
      case ChartType.heikinAshi:
        return 'Heikin Ashi';
      case ChartType.renko:
        return 'Renko';
      case ChartType.pointAndFigure:
        return 'Point & Figure';
      case ChartType.kagi:
        return 'Kagi';
    }
  }

  String _getChartTypeShortName(ChartType type) {
    switch (type) {
      case ChartType.candlestick:
        return 'Candles';
      case ChartType.line:
        return 'Line';
      case ChartType.area:
        return 'Area';
      case ChartType.ohlc:
        return 'OHLC';
      case ChartType.heikinAshi:
        return 'Heikin';
      case ChartType.renko:
        return 'Renko';
      case ChartType.pointAndFigure:
        return 'P&F';
      case ChartType.kagi:
        return 'Kagi';
    }
  }

  bool _indicatorNeedsChart(IndicatorType indicator) {
    switch (indicator) {
      case IndicatorType.rsi:
      case IndicatorType.macd:
      case IndicatorType.stochastic:
      case IndicatorType.williamsR:
        return true;
      default:
        return false;
    }
  }
}

enum ChartType {
  candlestick,
  line,
  area,
  ohlc,
  heikinAshi,
  renko,
  pointAndFigure,
  kagi,
}

enum IndicatorType {
  sma20,
  sma50,
  sma200,
  ema12,
  ema26,
  bollingerBands,
  rsi,
  macd,
  stochastic,
  williamsR,
  atr,
  adx,
  obv,
  vwap,
}

enum ChartTheme {
  light,
  dark,
  solarized,
}

class PriceData {
  final DateTime timestamp;
  final double open;
  final double high;
  final double low;
  final double close;
  final int volume;

  PriceData({
    required this.timestamp,
    required this.open,
    required this.high,
    required this.low,
    required this.close,
    required this.volume,
  });
}

// Custom Painters for chart rendering
class ChartPainter extends CustomPainter {
  final ChartType chartType;
  final List<PriceData> data;
  final List<IndicatorType> indicators;
  final ChartTheme theme;

  ChartPainter({
    required this.chartType,
    required this.data,
    required this.indicators,
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
      default:
        _drawCandlesticks(canvas, size, minPrice, priceRange);
    }

    // Draw indicators
    _drawIndicators(canvas, size, minPrice, priceRange);

    // Draw axes
    _drawAxes(canvas, size, minPrice, maxPrice);
  }

  void _drawGrid(Canvas canvas, Size size, double minPrice, double maxPrice) {
    final gridPaint = Paint()
      ..color = Colors.grey.withOpacity(0.2)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 0.5;

    // Horizontal grid lines
    for (int i = 0; i <= 5; i++) {
      final y = size.height * i / 5;
      canvas.drawLine(
        Offset(50, y),
        Offset(size.width, y),
        gridPaint,
      );
    }

    // Vertical grid lines
    final candleWidth = (size.width - 50) / data.length;
    for (int i = 0; i <= 10; i++) {
      final x = 50 + (size.width - 50) * i / 10;
      canvas.drawLine(
        Offset(x, 0),
        Offset(x, size.height - 20),
        gridPaint,
      );
    }
  }

  void _drawCandlesticks(Canvas canvas, Size size, double minPrice, double priceRange) {
    final candleWidth = (size.width - 50) / data.length * 0.8;
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;

      // Calculate y positions
      final highY = size.height - 20 - ((d.high - minPrice) / priceRange) * (size.height - 40);
      final lowY = size.height - 20 - ((d.low - minPrice) / priceRange) * (size.height - 40);
      final openY = size.height - 20 - ((d.open - minPrice) / priceRange) * (size.height - 40);
      final closeY = size.height - 20 - ((d.close - minPrice) / priceRange) * (size.height - 40);

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

  void _drawLineChart(Canvas canvas, Size size, double minPrice, double priceRange) {
    final linePaint = Paint()
      ..color = Colors.blue
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2;

    final path = Path();
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;
      final y = size.height - 20 - ((d.close - minPrice) / priceRange) * (size.height - 40);

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
      final y = size.height - 20 - ((d.close - minPrice) / priceRange) * (size.height - 40);

      if (i % 10 == 0) {
        canvas.drawCircle(Offset(x, y), 3, dotPaint);
      }
    }
  }

  void _drawAreaChart(Canvas canvas, Size size, double minPrice, double priceRange) {
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
      final y = size.height - 20 - ((d.close - minPrice) / priceRange) * (size.height - 40);

      if (i == 0) {
        path.moveTo(x, y);
      } else {
        path.lineTo(x, y);
      }
    }

    // Close the path for filling
    path.lineTo(50 + (data.length - 1) * spacing + spacing / 2, size.height - 20);
    path.lineTo(50 + spacing / 2, size.height - 20);
    path.close();

    canvas.drawPath(path, fillPaint);

    // Draw the line again
    final linePath = Path();
    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final x = 50 + i * spacing + spacing / 2;
      final y = size.height - 20 - ((d.close - minPrice) / priceRange) * (size.height - 40);

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

      final highY = size.height - 20 - ((d.high - minPrice) / priceRange) * (size.height - 40);
      final lowY = size.height - 20 - ((d.low - minPrice) / priceRange) * (size.height - 40);
      final openY = size.height - 20 - ((d.open - minPrice) / priceRange) * (size.height - 40);
      final closeY = size.height - 20 - ((d.close - minPrice) / priceRange) * (size.height - 40);

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

  void _drawIndicators(Canvas canvas, Size size, double minPrice, double priceRange) {
    final spacing = (size.width - 50) / data.length;

    for (final indicator in indicators) {
      if (indicator == IndicatorType.sma20) {
        _drawSMA(canvas, size, data, 20, Colors.yellow, minPrice, priceRange, spacing);
      } else if (indicator == IndicatorType.sma50) {
        _drawSMA(canvas, size, data, 50, Colors.orange, minPrice, priceRange, spacing);
      }
    }
  }

  void _drawSMA(Canvas canvas, Size size, List<PriceData> data, int period, Color color,
      double minPrice, double priceRange, double spacing) {
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
      final y = size.height - 20 - ((sma - minPrice) / priceRange) * (size.height - 40);

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
    final textPaint = Paint()
      ..color = Colors.grey;

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
      final time = '${data[i].timestamp.hour}:${data[i].timestamp.minute.toString().padLeft(2, '0')}';

      final textPainter = TextPainter(
        text: TextSpan(
          text: time,
          style: const TextStyle(color: Colors.grey, fontSize: 9),
        ),
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(x - textPainter.width / 2, size.height - 18));
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}

class VolumeChartPainter extends CustomPainter {
  final List<PriceData> data;
  final ChartTheme theme;

  VolumeChartPainter({
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

    // Find max volume for scaling
    final maxVolume = data.map((d) => d.volume).reduce(max).toDouble();
    if (maxVolume == 0) return;

    // Draw volume bars
    final barWidth = (size.width - 50) / data.length * 0.8;
    final spacing = (size.width - 50) / data.length;

    for (int i = 0; i < data.length; i++) {
      final d = data[i];
      final barHeight = (d.volume / maxVolume) * (size.height - 25);

      final isGreen = d.close >= d.open;
      final barColor = isGreen 
          ? Colors.green.withOpacity(0.7)
          : Colors.red.withOpacity(0.7);

      final paint = Paint()
        ..color = barColor
        ..style = PaintingStyle.fill;

      final x = 50 + i * spacing + (spacing - barWidth) / 2;
      final y = size.height - 25 - barHeight;

      canvas.drawRect(
        Rect.fromLTWH(x, y, barWidth, barHeight),
        paint,
      );
    }

    // Draw volume label
    final labelPainter = TextPainter(
      text: const TextSpan(
        text: 'Volume',
        style: TextStyle(color: Colors.grey, fontSize: 10),
      ),
      textDirection: TextDirection.ltr,
    );
    labelPainter.layout();
    labelPainter.paint(canvas, const Offset(5, 5));

    // Draw max volume value
    final maxVolPainter = TextPainter(
      text: TextSpan(
        text: _formatVolume(maxVolume.toInt()),
        style: const TextStyle(color: Colors.grey, fontSize: 9),
      ),
      textDirection: TextDirection.ltr,
    );
    maxVolPainter.layout();
    maxVolPainter.paint(canvas, Offset(5, size.height - 25 - maxVolPainter.height));
  }

  String _formatVolume(int volume) {
    if (volume >= 1000000) {
      return '${(volume / 1000000).toStringAsFixed(1)}M';
    } else if (volume >= 1000) {
      return '${(volume / 1000).toStringAsFixed(1)}K';
    }
    return volume.toString();
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}

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

double max(double a, double b) => a > b ? a : b;
double min(double a, double b) => a < b ? a : b;

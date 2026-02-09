import 'package:flutter/material.dart';
import 'dart:math';

class AdvancedChartingWidget extends StatefulWidget {
  final String symbol;
  final String timeframe;
  final Map<String, dynamic>? chartSettings;

  const AdvancedChartingWidget({
    super.key,
    required this.symbol,
    this.timeframe = '1D',
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
  
  @override
  void initState() {
    super.initState();
    _initializeDefaultIndicators();
  }

  void _initializeDefaultIndicators() {
    _activeIndicators = [
      IndicatorType.sma20,
      IndicatorType.sma50,
      IndicatorType.rsi,
    ];
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
            widget.timeframe,
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
          value: widget.timeframe,
          dropdownColor: const Color(0xFF37474F),
          style: const TextStyle(color: Colors.white, fontSize: 12),
          items: ['1m', '5m', '15m', '1h', '4h', '1D', '1W', '1M'].map((timeframe) {
            return DropdownMenuItem(
              value: timeframe,
              child: Text(timeframe),
            );
          }).toList(),
          onChanged: (value) {
            // TODO: Update timeframe
          },
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
      child: CustomPaint(
        painter: ChartPainter(
          chartType: _currentChartType,
          data: _generateSampleData(),
          indicators: _activeIndicators,
          theme: _theme,
        ),
        child: Container(),
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
      child: CustomPaint(
        painter: VolumeChartPainter(
          data: _generateSampleData(),
          theme: _theme,
        ),
        child: Container(),
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
              child: CustomPaint(
                painter: IndicatorChartPainter(
                  indicator: indicator,
                  data: _generateSampleData(),
                  theme: _theme,
                ),
                child: Container(),
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
    // TODO: Implement actual chart rendering
    // For now, draw a placeholder
    final paint = Paint()
      ..color = Colors.grey.shade600
      ..style = PaintingStyle.fill;
    
    canvas.drawRect(
      Rect.fromLTWH(0, 0, size.width, size.height),
      paint,
    );
    
    // Draw placeholder text
    final textPainter = TextPainter(
      text: TextSpan(
        text: '${_getChartTypeName(chartType)} Chart\n${data.length} data points',
        style: const TextStyle(
          color: Colors.white,
          fontSize: 14,
        ),
      ),
      textDirection: TextDirection.ltr,
    );
    
    textPainter.layout();
    textPainter.paint(
      canvas,
      Offset(
        (size.width - textPainter.width) / 2,
        (size.height - textPainter.height) / 2,
      ),
    );
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;

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
    // TODO: Implement volume chart rendering
    final paint = Paint()
      ..color = Colors.blue.withOpacity(0.6)
      ..style = PaintingStyle.fill;
    
    // Draw placeholder volume bars
    final barWidth = size.width / data.length;
    for (int i = 0; i < data.length; i++) {
      final barHeight = (data[i].volume / 3000000.0) * size.height;
      final barColor = data[i].close >= data[i].open 
          ? Colors.green.withOpacity(0.7)
          : Colors.red.withOpacity(0.7);
      
      paint.color = barColor;
      canvas.drawRect(
        Rect.fromLTWH(
          i * barWidth,
          size.height - barHeight,
          barWidth - 1,
          barHeight,
        ),
        paint,
      );
    }
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
    // TODO: Implement indicator chart rendering
    final paint = Paint()
      ..color = _getIndicatorColor()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.0;
    
    // Draw placeholder indicator line
    final path = Path();
    for (int i = 0; i < data.length; i++) {
      final x = (i / data.length) * size.width;
      final y = size.height / 2 + sin(i * 0.1) * size.height * 0.3;
      
      if (i == 0) {
        path.moveTo(x, y);
      } else {
        path.lineTo(x, y);
      }
    }
    
    canvas.drawPath(path, paint);
    
    // Draw indicator name
    final textPainter = TextPainter(
      text: TextSpan(
        text: _getIndicatorName(),
        style: TextStyle(
          color: _getIndicatorColor(),
          fontSize: 10,
        ),
      ),
      textDirection: TextDirection.ltr,
    );
    
    textPainter.layout();
    textPainter.paint(canvas, const Offset(8, 4));
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;

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
}
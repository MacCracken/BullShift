import 'package:flutter/material.dart';
import 'dart:math';
import 'drawing_tools.dart';

class AdvancedChartingWidget extends StatefulWidget {
  final String symbol;
  final String timeframe;
  final Function(String)? onTimeframeChanged;
  final Map<String, dynamic>? chartSettings;
  final List<PriceData>? priceData;
  final bool useRealtimeData;

  const AdvancedChartingWidget({
    super.key,
    required this.symbol,
    this.timeframe = '1D',
    this.onTimeframeChanged,
    this.chartSettings,
    this.priceData,
    this.useRealtimeData = false,
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
  List<PriceData>? _externalPriceData;

  // Multi-symbol comparison
  List<String> _comparisonSymbols = [];
  Map<String, List<PriceData>> _comparisonData = {};
  bool _showComparisonChart = false;

  // Drawing tools state
  DrawingToolType _currentDrawingTool = DrawingToolType.none;
  Color _currentDrawingColor = Colors.yellow;
  final DrawingToolManager _drawingManager = DrawingToolManager();

  @override
  void initState() {
    super.initState();
    _currentTimeframe = widget.timeframe;
    _initializeDefaultIndicators();
    _externalPriceData = widget.priceData;
  }

  @override
  void dispose() {
    _drawingManager.dispose();
    super.dispose();
  }

  @override
  void didUpdateWidget(AdvancedChartingWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.timeframe != widget.timeframe) {
      setState(() {
        _currentTimeframe = widget.timeframe;
      });
    }
    if (oldWidget.priceData != widget.priceData) {
      setState(() {
        _externalPriceData = widget.priceData;
      });
    }
  }

  List<PriceData> get _chartData => _externalPriceData ?? _generateSampleData();

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
          Expanded(flex: 3, child: _buildMainChart()),
          const SizedBox(height: 8),
          // Volume Chart
          if (_showVolume) Expanded(flex: 1, child: _buildVolumeChart()),
          // Indicator Charts
          ..._buildIndicatorCharts(),
        ],
      ),
    );
  }

  Widget _buildChartHeader() {
    return Row(
      children: [
        Icon(Icons.show_chart, color: Colors.white, size: 20),
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
          items: ['1m', '5m', '15m', '1h', '4h', '1D', '1W', '1M'].map((
            timeframe,
          ) {
            return DropdownMenuItem(value: timeframe, child: Text(timeframe));
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
        const SizedBox(width: 8),
        // Multi-symbol comparison toggle
        FilterChip(
          label: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(Icons.compare_arrows, size: 14),
              const SizedBox(width: 4),
              const Text('Compare', style: TextStyle(fontSize: 12)),
            ],
          ),
          selected: _showComparisonChart,
          onSelected: (selected) {
            setState(() {
              _showComparisonChart = selected;
              if (selected && _comparisonSymbols.isEmpty) {
                _addDefaultComparisonSymbols();
              }
            });
          },
          backgroundColor: const Color(0xFF37474F),
          selectedColor: Colors.purple,
          labelStyle: TextStyle(
            color: _showComparisonChart ? Colors.white : Colors.grey,
            fontSize: 12,
          ),
        ),
        if (_showComparisonChart) ...[
          const SizedBox(width: 4),
          IconButton(
            icon: const Icon(Icons.add, size: 18),
            tooltip: 'Add Symbol to Compare',
            onPressed: _showAddComparisonDialog,
            color: Colors.white,
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints(minWidth: 32, minHeight: 32),
          ),
        ],
        const SizedBox(width: 8),
        // Drawing Tools Toggle
        PopupMenuButton<DrawingToolType>(
          tooltip: 'Drawing Tools',
          icon: Icon(
            Icons.edit,
            color: _currentDrawingTool != DrawingToolType.none
                ? Colors.orange
                : Colors.white,
            size: 20,
          ),
          onSelected: (DrawingToolType tool) {
            setState(() {
              _currentDrawingTool = tool == _currentDrawingTool
                  ? DrawingToolType.none
                  : tool;
            });
          },
          itemBuilder: (context) => [
            const PopupMenuItem(
              value: DrawingToolType.none,
              child: Row(
                children: [
                  Icon(Icons.close, size: 16),
                  SizedBox(width: 8),
                  Text('None'),
                ],
              ),
            ),
            const PopupMenuDivider(),
            const PopupMenuItem(
              value: DrawingToolType.trendline,
              child: Row(
                children: [
                  Icon(Icons.show_chart, size: 16),
                  SizedBox(width: 8),
                  Text('Trendline'),
                ],
              ),
            ),
            const PopupMenuItem(
              value: DrawingToolType.horizontalLine,
              child: Row(
                children: [
                  Icon(Icons.horizontal_rule, size: 16),
                  SizedBox(width: 8),
                  Text('Horizontal Line'),
                ],
              ),
            ),
            const PopupMenuItem(
              value: DrawingToolType.verticalLine,
              child: Row(
                children: [
                  Icon(Icons.vertical_align_center, size: 16),
                  SizedBox(width: 8),
                  Text('Vertical Line'),
                ],
              ),
            ),
            const PopupMenuItem(
              value: DrawingToolType.fibonacciRetracement,
              child: Row(
                children: [
                  Icon(Icons.timeline, size: 16),
                  SizedBox(width: 8),
                  Text('Fib Retracement'),
                ],
              ),
            ),
            const PopupMenuItem(
              value: DrawingToolType.fibonacciExtension,
              child: Row(
                children: [
                  Icon(Icons.timeline, size: 16),
                  SizedBox(width: 8),
                  Text('Fib Extension'),
                ],
              ),
            ),
            const PopupMenuItem(
              value: DrawingToolType.rectangle,
              child: Row(
                children: [
                  Icon(Icons.crop_square, size: 16),
                  SizedBox(width: 8),
                  Text('Rectangle'),
                ],
              ),
            ),
            const PopupMenuItem(
              value: DrawingToolType.textAnnotation,
              child: Row(
                children: [
                  Icon(Icons.text_fields, size: 16),
                  SizedBox(width: 8),
                  Text('Text'),
                ],
              ),
            ),
          ],
        ),
        if (_currentDrawingTool != DrawingToolType.none) ...[
          const SizedBox(width: 4),
          PopupMenuButton<Color>(
            tooltip: 'Line Color',
            icon: Icon(Icons.palette, color: _currentDrawingColor, size: 20),
            onSelected: (Color color) {
              setState(() {
                _currentDrawingColor = color;
              });
            },
            itemBuilder: (context) =>
                [
                      Colors.yellow,
                      Colors.red,
                      Colors.blue,
                      Colors.green,
                      Colors.orange,
                      Colors.purple,
                      Colors.white,
                    ]
                    .map(
                      (color) => PopupMenuItem(
                        value: color,
                        child: Row(
                          children: [
                            Container(
                              width: 16,
                              height: 16,
                              decoration: BoxDecoration(
                                color: color,
                                shape: BoxShape.circle,
                              ),
                            ),
                            const SizedBox(width: 8),
                            Text(
                              color == Colors.yellow
                                  ? 'Yellow'
                                  : color == Colors.red
                                  ? 'Red'
                                  : color == Colors.blue
                                  ? 'Blue'
                                  : color == Colors.green
                                  ? 'Green'
                                  : color == Colors.orange
                                  ? 'Orange'
                                  : color == Colors.purple
                                  ? 'Purple'
                                  : 'White',
                            ),
                          ],
                        ),
                      ),
                    )
                    .toList(),
          ),
        ],
      ],
    );
  }

  void _addDefaultComparisonSymbols() {
    _comparisonSymbols = ['SPY', 'QQQ'];
    _generateComparisonData();
  }

  void _generateComparisonData() {
    _comparisonData = {};
    for (final symbol in _comparisonSymbols) {
      final random = Random(symbol.hashCode);
      final data = <PriceData>[];

      double currentPrice = 100.0 + random.nextDouble() * 100;

      for (int i = 0; i < _chartData.length; i++) {
        final change = (random.nextDouble() - 0.5) * 2.0;
        currentPrice += change;
        currentPrice = currentPrice.clamp(50.0, 200.0);

        data.add(
          PriceData(
            timestamp: _chartData[i].timestamp,
            open: currentPrice - random.nextDouble() * 0.5,
            high: currentPrice + random.nextDouble() * 0.5,
            low: currentPrice - random.nextDouble() * 0.5,
            close: currentPrice,
            volume: 1000000 + random.nextInt(2000000),
          ),
        );
      }
      _comparisonData[symbol] = data;
    }
  }

  void _showAddComparisonDialog() {
    showDialog(
      context: context,
      builder: (dialogContext) {
        String newSymbol = '';
        return AlertDialog(
          backgroundColor: const Color(0xFF263238),
          title: const Text(
            'Add Symbol to Compare',
            style: TextStyle(color: Colors.white),
          ),
          content: TextField(
            autofocus: true,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              hintText: 'Enter symbol (e.g., SPY)',
              hintStyle: TextStyle(color: Colors.grey),
            ),
            onChanged: (value) => newSymbol = value.toUpperCase(),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(dialogContext),
              child: const Text('Cancel'),
            ),
            TextButton(
              onPressed: () {
                if (newSymbol.isNotEmpty &&
                    !_comparisonSymbols.contains(newSymbol)) {
                  setState(() {
                    _comparisonSymbols = [..._comparisonSymbols, newSymbol];
                    _generateComparisonData();
                  });
                }
                Navigator.pop(dialogContext);
              },
              child: const Text('Add'),
            ),
          ],
        );
      },
    );
  }

  void _removeComparisonSymbol(String symbol) {
    setState(() {
      _comparisonSymbols = _comparisonSymbols
          .where((s) => s != symbol)
          .toList();
      _comparisonData.remove(symbol);
    });
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
        child: GestureDetector(
          onTapDown: _currentDrawingTool != DrawingToolType.none
              ? (details) => _handleChartTap(details, size)
              : null,
          onPanStart: _currentDrawingTool != DrawingToolType.none
              ? (details) => _handleDrawingStart(details, size)
              : null,
          onPanUpdate: _currentDrawingTool != DrawingToolType.none
              ? (details) => _handleDrawingUpdate(details, size)
              : null,
          onPanEnd: _currentDrawingTool != DrawingToolType.none
              ? (details) => _handleDrawingEnd(details)
              : null,
          child: CustomPaint(
            size: Size.infinite,
            painter: ChartPainter(
              chartType: _currentChartType,
              data: _chartData,
              indicators: _activeIndicators,
              theme: _theme,
              drawings: _drawingManager.drawings,
              comparisonData: _comparisonData,
              showComparison: _showComparisonChart,
            ),
          ),
        ),
      ),
    );
  }

  Offset? _drawingStartPoint;
  Size? _chartSize;

  void _handleChartTap(TapDownDetails details, Size size) {
    if (_chartData.isEmpty) return;

    final price = _getPriceFromY(details.localPosition.dy, size.height);
    final time = _getTimeFromX(details.localPosition.dx, size.width);

    switch (_currentDrawingTool) {
      case DrawingToolType.horizontalLine:
        _drawingManager.addDrawing(
          HorizontalLine(
            id: DateTime.now().millisecondsSinceEpoch.toString(),
            price: price,
            color: _currentDrawingColor,
          ),
        );
        break;
      case DrawingToolType.verticalLine:
        _drawingManager.addDrawing(
          VerticalLine(
            id: DateTime.now().millisecondsSinceEpoch.toString(),
            time: time,
            color: _currentDrawingColor,
          ),
        );
        break;
      case DrawingToolType.fibonacciRetracement:
        _drawingManager.addDrawing(
          FibonacciRetracement(
            id: DateTime.now().millisecondsSinceEpoch.toString(),
            highPrice: _chartData.last.high,
            lowPrice: _chartData.last.low,
            color: _currentDrawingColor,
          ),
        );
        break;
      case DrawingToolType.fibonacciExtension:
        _drawingManager.addDrawing(
          FibonacciExtension(
            id: DateTime.now().millisecondsSinceEpoch.toString(),
            lowPrice: _chartData.last.low,
            highPrice: _chartData.last.high,
            color: _currentDrawingColor,
          ),
        );
        break;
      case DrawingToolType.textAnnotation:
        _showTextInputDialog(price, time);
        break;
      default:
        break;
    }
    setState(() {});
  }

  void _handleDrawingStart(DragStartDetails details, Size size) {
    if (_chartData.isEmpty) return;
    _drawingStartPoint = details.localPosition;
    _chartSize = size;
  }

  void _handleDrawingUpdate(DragUpdateDetails details, Size size) {
    // Could add preview drawing here
  }

  void _handleDrawingEnd(DragEndDetails details) {
    if (_drawingStartPoint == null || _chartSize == null || _chartData.isEmpty)
      return;

    final startPrice = _getPriceFromY(
      _drawingStartPoint!.dy,
      _chartSize!.height,
    );
    final endPrice = _getPriceFromY(_drawingStartPoint!.dy, _chartSize!.height);
    final startTime = _getTimeFromX(_drawingStartPoint!.dx, _chartSize!.width);
    final endTime = _getTimeFromX(_drawingStartPoint!.dx, _chartSize!.width);

    switch (_currentDrawingTool) {
      case DrawingToolType.trendline:
        _drawingManager.addDrawing(
          Trendline(
            id: DateTime.now().millisecondsSinceEpoch.toString(),
            startPrice: startPrice,
            endPrice: endPrice,
            startTime: startTime,
            endTime: endTime,
            color: _currentDrawingColor,
          ),
        );
        break;
      case DrawingToolType.rectangle:
        _drawingManager.addDrawing(
          RectangleDrawing(
            id: DateTime.now().millisecondsSinceEpoch.toString(),
            topPrice: startPrice > endPrice ? startPrice : endPrice,
            bottomPrice: startPrice > endPrice ? endPrice : startPrice,
            startTime: startTime,
            endTime: endTime,
            color: _currentDrawingColor,
          ),
        );
        break;
      default:
        break;
    }

    _drawingStartPoint = null;
    _chartSize = null;
    setState(() {});
  }

  double _getPriceFromY(double y, double height) {
    if (_chartData.isEmpty) return 0;
    final minPrice = _chartData.map((d) => d.low).reduce(min);
    final maxPrice = _chartData.map((d) => d.high).reduce(max);
    final priceRange = maxPrice - minPrice;
    return maxPrice - (y / height) * priceRange;
  }

  DateTime _getTimeFromX(double x, double width) {
    if (_chartData.isEmpty) return DateTime.now();
    final firstTime = _chartData.first.timestamp;
    final lastTime = _chartData.last.timestamp;
    final timeRange = lastTime.difference(firstTime).inMilliseconds;
    final xOffset = (x - 50) / (width - 50);
    return firstTime.add(Duration(milliseconds: (timeRange * xOffset).round()));
  }

  void _showTextInputDialog(double price, DateTime time) {
    // Simple implementation - in real app, show a dialog
    final text = 'Note';
    _drawingManager.addDrawing(
      TextAnnotation(
        id: DateTime.now().millisecondsSinceEpoch.toString(),
        price: price,
        time: time,
        text: text,
        color: _currentDrawingColor,
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
          painter: VolumeChartPainter(data: _chartData, theme: _theme),
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
                    data: _chartData,
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

      data.add(
        PriceData(
          timestamp: now.subtract(Duration(minutes: (100 - i) * 5)),
          open: open,
          high: high,
          low: low,
          close: close,
          volume: volume,
        ),
      );
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

enum ChartTheme { light, dark, solarized }

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
    final spacing = (size.width - 50) / max(kagiData.length, 1);

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

class VolumeChartPainter extends CustomPainter {
  final List<PriceData> data;
  final ChartTheme theme;

  VolumeChartPainter({required this.data, required this.theme});

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

      canvas.drawRect(Rect.fromLTWH(x, y, barWidth, barHeight), paint);
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
    maxVolPainter.paint(
      canvas,
      Offset(5, size.height - 25 - maxVolPainter.height),
    );
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

// Helper classes for advanced chart types
class RenkoBrick {
  final double price;
  final bool isBullish;

  RenkoBrick(this.price, this.isBullish);
}

class PointAndFigurePoint {
  final double price;
  final int column;
  final bool isX;

  PointAndFigurePoint(this.price, this.column, this.isX);
}

class KagiPoint {
  final double price;
  final bool isBullish;
  final bool isVertical;

  KagiPoint(this.price, this.isBullish, this.isVertical);
}

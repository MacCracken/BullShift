import 'package:flutter/material.dart';
import 'dart:math';
import 'drawing_tools.dart';
import 'chart_enums.dart';
import 'chart_toolbar.dart';
import 'candlestick_painter.dart';
import 'volume_painter.dart';
import 'indicator_painter.dart';
import 'comparison_chart.dart';

// Re-export chart_enums so existing importers of this file still get
// PriceData, ChartType, IndicatorType, ChartTheme, etc.
export 'chart_enums.dart';

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
          ChartToolbar(
            currentChartType: _currentChartType,
            currentTimeframe: _currentTimeframe,
            showVolume: _showVolume,
            showComparisonChart: _showComparisonChart,
            currentDrawingTool: _currentDrawingTool,
            currentDrawingColor: _currentDrawingColor,
            onChartTypeChanged: (type) {
              setState(() {
                _currentChartType = type;
              });
            },
            onTimeframeChanged: _onTimeframeChanged,
            onVolumeToggled: (selected) {
              setState(() {
                _showVolume = selected;
              });
            },
            onComparisonToggled: (selected) {
              setState(() {
                _showComparisonChart = selected;
                if (selected && _comparisonSymbols.isEmpty) {
                  _addDefaultComparisonSymbols();
                }
              });
            },
            onDrawingToolChanged: (tool) {
              setState(() {
                _currentDrawingTool = tool;
              });
            },
            onDrawingColorChanged: (color) {
              setState(() {
                _currentDrawingColor = color;
              });
            },
            onAddComparisonSymbol: _showAddComparisonDialog,
          ),
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
                  Icon(getChartTypeIcon(type), size: 16),
                  const SizedBox(width: 8),
                  Text(getChartTypeName(type)),
                ],
              ),
            );
          }).toList(),
        ),
      ],
    );
  }

  void _addDefaultComparisonSymbols() {
    _comparisonSymbols = ComparisonChartHelper.getDefaultComparisonSymbols();
    _generateComparisonData();
  }

  void _generateComparisonData() {
    _comparisonData = ComparisonChartHelper.generateComparisonData(
      _comparisonSymbols,
      _chartData,
    );
  }

  void _showAddComparisonDialog() {
    ComparisonChartHelper.showAddComparisonDialog(
      context: context,
      currentSymbols: _comparisonSymbols,
      onSymbolAdded: (newSymbol) {
        setState(() {
          _comparisonSymbols = [..._comparisonSymbols, newSymbol];
          _generateComparisonData();
        });
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
      if (indicatorNeedsChart(indicator)) {
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
}

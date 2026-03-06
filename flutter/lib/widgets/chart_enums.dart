import 'package:flutter/material.dart';

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

// Chart type utility functions
IconData getChartTypeIcon(ChartType type) {
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

String getChartTypeName(ChartType type) {
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

String getChartTypeShortName(ChartType type) {
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

bool indicatorNeedsChart(IndicatorType indicator) {
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

double max(double a, double b) => a > b ? a : b;
double min(double a, double b) => a < b ? a : b;

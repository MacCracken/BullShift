import 'package:flutter/material.dart';
import 'chart_enums.dart';
import 'drawing_tools.dart';

class ChartToolbar extends StatelessWidget {
  final ChartType currentChartType;
  final String currentTimeframe;
  final bool showVolume;
  final bool showComparisonChart;
  final DrawingToolType currentDrawingTool;
  final Color currentDrawingColor;
  final ValueChanged<ChartType> onChartTypeChanged;
  final ValueChanged<String?> onTimeframeChanged;
  final ValueChanged<bool> onVolumeToggled;
  final ValueChanged<bool> onComparisonToggled;
  final ValueChanged<DrawingToolType> onDrawingToolChanged;
  final ValueChanged<Color> onDrawingColorChanged;
  final VoidCallback onAddComparisonSymbol;

  const ChartToolbar({
    super.key,
    required this.currentChartType,
    required this.currentTimeframe,
    required this.showVolume,
    required this.showComparisonChart,
    required this.currentDrawingTool,
    required this.currentDrawingColor,
    required this.onChartTypeChanged,
    required this.onTimeframeChanged,
    required this.onVolumeToggled,
    required this.onComparisonToggled,
    required this.onDrawingToolChanged,
    required this.onDrawingColorChanged,
    required this.onAddComparisonSymbol,
  });

  @override
  Widget build(BuildContext context) {
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
              final isSelected = currentChartType == type;

              return Padding(
                padding: const EdgeInsets.only(right: 4),
                child: FilterChip(
                  label: Text(getChartTypeShortName(type)),
                  selected: isSelected,
                  onSelected: (selected) {
                    if (selected) {
                      onChartTypeChanged(type);
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
          value: currentTimeframe,
          dropdownColor: const Color(0xFF37474F),
          style: const TextStyle(color: Colors.white, fontSize: 12),
          items: ['1m', '5m', '15m', '1h', '4h', '1D', '1W', '1M'].map((
            timeframe,
          ) {
            return DropdownMenuItem(value: timeframe, child: Text(timeframe));
          }).toList(),
          onChanged: onTimeframeChanged,
        ),
        const SizedBox(width: 8),
        // Volume Toggle
        FilterChip(
          label: const Text('Volume', style: TextStyle(fontSize: 12)),
          selected: showVolume,
          onSelected: onVolumeToggled,
          backgroundColor: const Color(0xFF37474F),
          selectedColor: Colors.green,
          labelStyle: TextStyle(
            color: showVolume ? Colors.white : Colors.grey,
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
          selected: showComparisonChart,
          onSelected: onComparisonToggled,
          backgroundColor: const Color(0xFF37474F),
          selectedColor: Colors.purple,
          labelStyle: TextStyle(
            color: showComparisonChart ? Colors.white : Colors.grey,
            fontSize: 12,
          ),
        ),
        if (showComparisonChart) ...[
          const SizedBox(width: 4),
          IconButton(
            icon: const Icon(Icons.add, size: 18),
            tooltip: 'Add Symbol to Compare',
            onPressed: onAddComparisonSymbol,
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
            color: currentDrawingTool != DrawingToolType.none
                ? Colors.orange
                : Colors.white,
            size: 20,
          ),
          onSelected: (DrawingToolType tool) {
            onDrawingToolChanged(
              tool == currentDrawingTool ? DrawingToolType.none : tool,
            );
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
        if (currentDrawingTool != DrawingToolType.none) ...[
          const SizedBox(width: 4),
          PopupMenuButton<Color>(
            tooltip: 'Line Color',
            icon: Icon(Icons.palette, color: currentDrawingColor, size: 20),
            onSelected: onDrawingColorChanged,
            itemBuilder: (context) => [
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
}

import 'package:flutter/material.dart';
import 'chart_enums.dart';

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
      final barColor =
          isGreen ? Colors.green.withOpacity(0.7) : Colors.red.withOpacity(0.7);

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

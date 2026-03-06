/// Safe casting extensions for Map<String, dynamic> values.
///
/// These avoid runtime TypeErrors when map values are null or wrong type.
/// Use these instead of bare `as Type` casts on dynamic map values.
extension SafeMapCast on Map<String, dynamic> {
  double safeDouble(String key, [double fallback = 0.0]) {
    final v = this[key];
    if (v is double) return v;
    if (v is int) return v.toDouble();
    if (v is num) return v.toDouble();
    return fallback;
  }

  int safeInt(String key, [int fallback = 0]) {
    final v = this[key];
    if (v is int) return v;
    if (v is double) return v.toInt();
    if (v is num) return v.toInt();
    return fallback;
  }

  String safeString(String key, [String fallback = '']) {
    final v = this[key];
    if (v is String) return v;
    return v?.toString() ?? fallback;
  }

  bool safeBool(String key, [bool fallback = false]) {
    final v = this[key];
    if (v is bool) return v;
    return fallback;
  }
}

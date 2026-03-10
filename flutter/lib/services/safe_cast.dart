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

  List<T> safeList<T>(String key, [List<T> fallback = const []]) {
    final v = this[key];
    if (v is List) return v.whereType<T>().toList();
    return fallback;
  }

  Map<String, dynamic> safeMap(String key,
      [Map<String, dynamic> fallback = const {}]) {
    final v = this[key];
    if (v is Map<String, dynamic>) return v;
    if (v is Map) return Map<String, dynamic>.from(v);
    return fallback;
  }
}

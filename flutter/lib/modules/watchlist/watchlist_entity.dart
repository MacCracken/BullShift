import 'package:objectbox/objectbox.dart';

@Entity()
class WatchlistEntity {
  @Id()
  int id = 0;

  @Property(unique: true)
  late String symbol;

  @Property()
  late double currentPrice;

  @Property()
  double dayChange = 0.0;

  @Property()
  double dayChangePercent = 0.0;

  @Property()
  int volume = 0;

  @Property()
  double marketCap = 0.0;

  @Property()
  int timestamp = 0;

  @Property()
  double previousPrice = 0.0;

  @Property()
  bool isActive = true; // Soft delete support

  @Property()
  int sortOrder = 0; // Custom ordering

  @Property()
  String? notes; // User notes for the symbol

  @Property()
  String? category; // User-defined category

  @Property()
  bool alertsEnabled = false;

  @Property()
  double? alertPriceHigh;

  @Property()
  double? alertPriceLow;

  @Property()
  double? alertChangePercent;

  @Property()
  DateTime createdAt = DateTime.now();

  @Property()
  DateTime updatedAt = DateTime.now();

  WatchlistEntity();

  // Constructor for creating from map
  WatchlistEntity.fromMap(Map<String, dynamic> map) {
    symbol = map['symbol'] as String;
    currentPrice = (map['currentPrice'] as num).toDouble();
    dayChange = (map['dayChange'] as num?)?.toDouble() ?? 0.0;
    dayChangePercent = (map['dayChangePercent'] as num?)?.toDouble() ?? 0.0;
    volume = (map['volume'] as num?)?.toInt() ?? 0;
    marketCap = (map['marketCap'] as num?)?.toDouble() ?? 0.0;
    timestamp = (map['timestamp'] as DateTime?)?.millisecondsSinceEpoch ??
        DateTime.now().millisecondsSinceEpoch;
    previousPrice = (map['previousPrice'] as num?)?.toDouble() ?? currentPrice;
    isActive = (map['isActive'] as bool?) ?? true;
    sortOrder = (map['sortOrder'] as int?) ?? 0;
    notes = map['notes'] as String?;
    category = map['category'] as String?;
    alertsEnabled = (map['alertsEnabled'] as bool?) ?? false;
    alertPriceHigh = (map['alertPriceHigh'] as num?)?.toDouble();
    alertPriceLow = (map['alertPriceLow'] as num?)?.toDouble();
    alertChangePercent = (map['alertChangePercent'] as num?)?.toDouble();
    createdAt = (map['createdAt'] as DateTime?) ?? DateTime.now();
    updatedAt = (map['updatedAt'] as DateTime?) ?? DateTime.now();
  }

  // Convert to map
  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'symbol': symbol,
      'currentPrice': currentPrice,
      'dayChange': dayChange,
      'dayChangePercent': dayChangePercent,
      'volume': volume,
      'marketCap': marketCap,
      'timestamp': DateTime.fromMillisecondsSinceEpoch(timestamp),
      'previousPrice': previousPrice,
      'isActive': isActive,
      'sortOrder': sortOrder,
      'notes': notes,
      'category': category,
      'alertsEnabled': alertsEnabled,
      'alertPriceHigh': alertPriceHigh,
      'alertPriceLow': alertPriceLow,
      'alertChangePercent': alertChangePercent,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }

  // Update with new data (e.g., from market data refresh)
  void updateFromMap(Map<String, dynamic> map) {
    if (map.containsKey('currentPrice')) {
      previousPrice = currentPrice;
      currentPrice = (map['currentPrice'] as num).toDouble();
    }

    if (map.containsKey('dayChange')) {
      dayChange = (map['dayChange'] as num).toDouble();
    }

    if (map.containsKey('dayChangePercent')) {
      dayChangePercent = (map['dayChangePercent'] as num).toDouble();
    }

    if (map.containsKey('volume')) {
      volume = (map['volume'] as num).toInt();
    }

    if (map.containsKey('marketCap')) {
      marketCap = (map['marketCap'] as num).toDouble();
    }

    timestamp = DateTime.now().millisecondsSinceEpoch;
    updatedAt = DateTime.now();
  }

  // Check if price alerts should trigger
  bool shouldTriggerAlert() {
    if (!alertsEnabled) return false;

    if (alertPriceHigh != null && currentPrice >= alertPriceHigh!) {
      return true;
    }

    if (alertPriceLow != null && currentPrice <= alertPriceLow!) {
      return true;
    }

    if (alertChangePercent != null) {
      final actualChangePercent =
          (currentPrice - previousPrice) / previousPrice * 100;
      if (actualChangePercent.abs() >= alertChangePercent!) {
        return true;
      }
    }

    return false;
  }

  // Get alert message
  String getAlertMessage() {
    if (!alertsEnabled) return '';

    if (alertPriceHigh != null && currentPrice >= alertPriceHigh!) {
      return 'Price alert: $symbol reached high price \$${currentPrice.toStringAsFixed(2)}';
    }

    if (alertPriceLow != null && currentPrice <= alertPriceLow!) {
      return 'Price alert: $symbol reached low price \$${currentPrice.toStringAsFixed(2)}';
    }

    if (alertChangePercent != null) {
      final actualChangePercent =
          (currentPrice - previousPrice) / previousPrice * 100;
      if (actualChangePercent.abs() >= alertChangePercent!) {
        final direction = actualChangePercent >= 0 ? 'increased' : 'decreased';
        return 'Price alert: $symbol $direction by ${actualChangePercent.abs().toStringAsFixed(2)}%';
      }
    }

    return '';
  }

  @override
  String toString() {
    return 'WatchlistEntity{id: $id, symbol: $symbol, currentPrice: \$${currentPrice.toStringAsFixed(2)}, dayChange: ${dayChange.toStringAsFixed(2)}%}';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is WatchlistEntity && other.symbol == symbol;
  }

  @override
  int get hashCode => symbol.hashCode;
}

// ObjectBox query helper methods
extension WatchlistEntityQueries on Box<WatchlistEntity> {
  // Get all active watchlist items
  List<WatchlistEntity> getAllActive() {
    return query(WatchlistEntity_.isActive.equals(true))
        .order(WatchlistEntity_.sortOrder)
        .build()
        .find();
  }

  // Get by symbol
  WatchlistEntity? getBySymbol(String symbol) {
    return query(WatchlistEntity_.symbol.equals(symbol.toUpperCase()))
        .build()
        .findFirst();
  }

  // Get by category
  List<WatchlistEntity> getByCategory(String category) {
    return query(WatchlistEntity_.category.equals(category))
        .order(WatchlistEntity_.sortOrder)
        .build()
        .find();
  }

  // Get items with alerts enabled
  List<WatchlistEntity> getWithAlertsEnabled() {
    return query(WatchlistEntity_.alertsEnabled.equals(true)).build().find();
  }

  // Soft delete by symbol
  bool softDeleteBySymbol(String symbol) {
    final entity = getBySymbol(symbol);
    if (entity != null) {
      entity.isActive = false;
      put(entity);
      return true;
    }
    return false;
  }

  // Update sort order for all items
  void updateSortOrder(List<String> symbolsInOrder) {
    final entities = getAll();
    for (int i = 0; i < symbolsInOrder.length; i++) {
      final symbol = symbolsInOrder[i];
      final entity = entities.firstWhere((e) => e.symbol == symbol,
          orElse: () => WatchlistEntity());
      if (entity.id > 0) {
        entity.sortOrder = i;
        put(entity);
      }
    }
  }

  // Search symbols
  List<WatchlistEntity> searchSymbols(String query) {
    if (query.isEmpty) return getAllActive();

    return query(WatchlistEntity_.symbol
            .contains(query.toLowerCase(), caseSensitive: false))
        .order(WatchlistEntity_.sortOrder)
        .build()
        .find();
  }

  // Get symbols that need price updates
  List<WatchlistEntity> getNeedingUpdate() {
    final now = DateTime.now().millisecondsSinceEpoch;
    final oneMinuteAgo = now - 60000; // 1 minute

    return query(WatchlistEntity_.timestamp.lessThan(oneMinuteAgo))
        .build()
        .find();
  }
}

import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

typedef SubmitOrderFunc = Bool Function(Pointer<TradeOrder>);
typedef SubmitOrder = bool Function(Pointer<TradeOrder>);

typedef GetPositionsFunc = Pointer<Position> Function();
typedef GetPositions = Pointer<Position> Function();

typedef ConnectMarketDataFunc = Bool Function(Pointer<Utf8>);
typedef ConnectMarketData = bool Function(Pointer<Utf8>);

typedef GetAccountBalanceFunc = Double Function();
typedef GetAccountBalance = double Function();

class TradeOrder extends Struct {
  external Pointer<Utf8> symbol;
  external Pointer<Utf8> side;
  external double quantity;
  external Pointer<Utf8> orderType;
  external Pointer<Double> price;
}

class Position extends Struct {
  external Pointer<Utf8> symbol;
  external double quantity;
  external double entryPrice;
  external double currentPrice;
  external double unrealizedPnl;
}

class MarketData extends Struct {
  external Pointer<Utf8> symbol;
  external double price;
  external double volume;
  external int timestamp;
}

class RustTradingEngine {
  late DynamicLibrary _lib;
  late SubmitOrder _submitOrder;
  late GetPositions _getPositions;
  late ConnectMarketData _connectMarketData;
  late GetAccountBalance _getAccountBalance;

  RustTradingEngine() {
    final libPath = Platform.isLinux
        ? './rust/target/release/libbullshift_core.so'
        : './rust/target/release/libbullshift_core.dylib';

    _lib = DynamicLibrary.open(libPath);

    _submitOrder =
        _lib.lookupFunction<SubmitOrderFunc, SubmitOrder>('submit_order');
    _getPositions =
        _lib.lookupFunction<GetPositionsFunc, GetPositions>('get_positions');
    _connectMarketData =
        _lib.lookupFunction<ConnectMarketDataFunc, ConnectMarketData>(
            'connect_market_data');
    _getAccountBalance =
        _lib.lookupFunction<GetAccountBalanceFunc, GetAccountBalance>(
            'get_account_balance');
  }

  bool submitOrder({
    required String symbol,
    required String side,
    required double quantity,
    required String orderType,
    double? price,
  }) {
    final order = calloc<TradeOrder>();
    final symbolPtr = symbol.toNativeUtf8();
    final sidePtr = side.toNativeUtf8();
    final orderTypePtr = orderType.toNativeUtf8();
    Pointer<Double>? pricePtr;

    order.ref.symbol = symbolPtr;
    order.ref.side = sidePtr;
    order.ref.quantity = quantity;
    order.ref.orderType = orderTypePtr;

    if (price != null) {
      pricePtr = calloc<Double>();
      pricePtr.value = price;
      order.ref.price = pricePtr;
    }

    final result = _submitOrder(order);

    // Free all allocated memory
    calloc.free(symbolPtr);
    calloc.free(sidePtr);
    calloc.free(orderTypePtr);
    if (pricePtr != null) {
      calloc.free(pricePtr);
    }
    calloc.free(order);
    return result;
  }

  Pointer<Position> getPositions() {
    return _getPositions();
  }

  bool connectMarketData(String symbol) {
    final symbolPtr = symbol.toNativeUtf8();
    final result = _connectMarketData(symbolPtr);
    calloc.free(symbolPtr);
    return result;
  }

  double getAccountBalance() {
    return _getAccountBalance();
  }

  void dispose() {
    // DynamicLibrary doesn't have an explicit close in dart:ffi,
    // but clearing references allows GC to release the handle.
    // On Linux, dlclose is not called automatically — this is a known
    // Dart FFI limitation. For long-running apps this is acceptable
    // since the library lives for the app lifetime.
  }
}

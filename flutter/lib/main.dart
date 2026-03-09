import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/rust_trading_engine.dart';
import '../modules/core_trading/trading_provider.dart';
import '../modules/core_trading/core_trading_view.dart';
import '../modules/trendsetter/trendsetter_provider.dart';
import '../modules/trendsetter/trendsetter_view.dart';
import '../modules/bullrunnr/bullrunnr_provider.dart';
import '../modules/bullrunnr/bullrunnr_view.dart';
import '../modules/bearly_managed/bearly_managed_provider.dart';
import '../modules/bearly_managed/bearly_managed_view.dart';
import '../modules/paper_hands/paper_hands_provider.dart';
import '../modules/paper_hands/paper_hands_view.dart';
import '../modules/watchlist/watchlist_provider.dart';
import '../modules/market_data/market_data_provider.dart';
import '../modules/watchlist/watchlist_view.dart';
import '../widgets/order_dialog.dart';

class BullShiftApp extends StatelessWidget {
  const BullShiftApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'BullShift Trading Platform',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF1B5E20),
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      home: const TradingDashboard(),
    );
  }
}

class TradingDashboard extends StatefulWidget {
  const TradingDashboard({super.key});

  @override
  State<TradingDashboard> createState() => _TradingDashboardState();
}

class _TradingDashboardState extends State<TradingDashboard> {
  final RustTradingEngine _rustEngine = RustTradingEngine();
  double _accountBalance = 0.0;
  int _selectedIndex = 0;

  @override
  void initState() {
    super.initState();
    _loadAccountBalance();

    // Initialize providers after first frame
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        Provider.of<TrendSetterProvider>(context, listen: false).initialize();
        Provider.of<WatchlistProvider>(context, listen: false).initialize();
      }
    });
  }

  void _loadAccountBalance() {
    setState(() {
      _accountBalance = _rustEngine.getAccountBalance();
    });
  }

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => TradingProvider(_rustEngine)),
        ChangeNotifierProvider(create: (_) => TrendSetterProvider()),
        ChangeNotifierProvider(create: (_) => BullRunnrProvider()),
        ChangeNotifierProvider(create: (_) => BearlyManagedProvider()),
        ChangeNotifierProvider(create: (_) => PaperHandsProvider()),
        ChangeNotifierProvider(create: (_) => WatchlistProvider(_rustEngine)),
        ChangeNotifierProvider(create: (_) => MarketDataProvider(_rustEngine)),
      ],
      child: Scaffold(
        appBar: AppBar(
          title: Text(_getAppBarTitle()),
          backgroundColor: const Color(0xFF1B5E20),
          actions: [
            IconButton(
              icon: const Icon(Icons.account_balance_wallet),
              onPressed: _loadAccountBalance,
              tooltip: 'Refresh Account Balance',
            ),
          ],
        ),
        body: Row(
          children: [
            // Sidebar
            SizedBox(
              width: 250,
              child: NavigationRail(
                backgroundColor: const Color(0xFF263238),
                selectedIndex: _selectedIndex,
                onDestinationSelected: (index) {
                  setState(() {
                    _selectedIndex = index;
                  });
                },
                destinations: const [
                  NavigationRailDestination(
                    icon: Icon(Icons.trending_up),
                    label: Text('Core Trading'),
                  ),
                  NavigationRailDestination(
                    icon: Icon(Icons.star_border),
                    label: Text('Watchlist'),
                  ),
                  NavigationRailDestination(
                    icon: Icon(Icons.show_chart),
                    label: Text('TrendSetter'),
                  ),
                  NavigationRailDestination(
                    icon: Icon(Icons.article),
                    label: Text('BullRunnr'),
                  ),
                  NavigationRailDestination(
                    icon: Icon(Icons.smart_toy),
                    label: Text('BearlyManaged'),
                  ),
                  NavigationRailDestination(
                    icon: Icon(Icons.school),
                    label: Text('PaperHands'),
                  ),
                ],
              ),
            ),
            // Main Content
            Expanded(
              child: Column(
                children: [
                  // Account Bar
                  Container(
                    padding: const EdgeInsets.all(16),
                    color: const Color(0xFF37474F),
                    child: Row(
                      children: [
                        Text(
                          'Account Balance: \$${_accountBalance.toStringAsFixed(2)}',
                          style: const TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            color: Colors.white,
                          ),
                        ),
                        const Spacer(),
                        if (_selectedIndex == 0) // Only show on Core Trading
                          ElevatedButton(
                            onPressed: () => _showOrderDialog(context),
                            child: const Text('New Order'),
                          ),
                      ],
                    ),
                  ),
                  // Current View
                  Expanded(child: _getCurrentView()),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _getAppBarTitle() {
    switch (_selectedIndex) {
      case 0:
        return 'BullShift - Core Trading';
      case 1:
        return 'BullShift - Watchlist';
      case 2:
        return 'BullShift - TrendSetter';
      case 3:
        return 'BullShift - BullRunnr';
      case 4:
        return 'BullShift - BearlyManaged';
      case 5:
        return 'BullShift - PaperHands';
      default:
        return 'BullShift Trading Platform';
    }
  }

  Widget _getCurrentView() {
    switch (_selectedIndex) {
      case 0:
        return const CoreTradingView();
      case 1:
        return const WatchlistView();
      case 2:
        return const TrendSetterView();
      case 3:
        return const BullRunnrView();
      case 4:
        return const BearlyManagedView();
      case 5:
        return const PaperHandsView();
      default:
        return const CoreTradingView();
    }
  }

  void _showOrderDialog(BuildContext context) {
    showDialog(context: context, builder: (context) => const OrderDialog());
  }
}

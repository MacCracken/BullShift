import 'package:flutter/material.dart';
import '../services/rust_trading_engine.dart';
import '../modules/core_trading/trading_provider.dart';
import '../widgets/notes_panel.dart';
import '../widgets/advanced_charting_widget.dart';

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

  @override
  void initState() {
    super.initState();
    _loadAccountBalance();
  }

  void _loadAccountBalance() {
    setState(() {
      _accountBalance = _rustEngine.getAccountBalance();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('BullShift Trading'),
        backgroundColor: const Color(0xFF1B5E20),
        actions: [
          IconButton(
            icon: const Icon(Icons.account_balance_wallet),
            onPressed: _loadAccountBalance,
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
              selectedIndex: 0,
              onDestinationSelected: (index) {
                // Handle navigation
              },
              destinations: const [
                NavigationRailDestination(
                  icon: Icon(Icons.trending_up),
                  label: Text('Core Trading'),
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
                      ElevatedButton(
                        onPressed: () => _showOrderDialog(context),
                        child: const Text('New Order'),
                      ),
                    ],
                  ),
                ),
                // Trading View
                Expanded(
                  child: const CoreTradingView(),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  void _showOrderDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => const OrderDialog(),
    );
  }
}
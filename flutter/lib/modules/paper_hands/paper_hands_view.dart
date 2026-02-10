import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../services/rust_trading_engine.dart';
import '../core_trading/trading_provider.dart';
import '../../widgets/advanced_charting_widget.dart';
import 'paper_hands_provider.dart';

class PaperHandsView extends StatelessWidget {
  const PaperHandsView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<PaperHandsProvider>(
      builder: (context, provider, child) {
        return Row(
          children: [
            // Portfolio Management
            Expanded(
              flex: 1,
              child: PortfolioManagementPanel(provider: provider),
            ),
            // Paper Trading with Advanced Charting
            Expanded(
              flex: 2,
              child: PaperTradingPanel(provider: provider),
            ),
            // Performance Analytics
            Expanded(
              flex: 1,
              child: PerformanceAnalyticsPanel(provider: provider),
            ),
          ],
        );
      },
    );
  }
}

class PortfolioManagementPanel extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PortfolioManagementPanel({
    super.key,
    required this.provider,
  });

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
          Row(
            children: [
              const Icon(Icons.account_balance_wallet, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              const Text(
                '🎮 Paper Portfolios',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.add, color: Colors.white),
                onPressed: () => _showCreatePortfolioDialog(context),
                tooltip: 'Create Portfolio',
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Portfolio list
          Expanded(
            child: provider.paperPortfolios.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.account_balance_wallet_outlined,
                          color: Colors.grey,
                          size: 48,
                        ),
                        SizedBox(height: 16),
                        Text(
                          'No paper portfolios',
                          style: TextStyle(color: Colors.grey),
                        ),
                        SizedBox(height: 8),
                        Text(
                          'Create a portfolio to start paper trading',
                          style: TextStyle(
                            color: Colors.grey,
                            fontSize: 12,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: provider.paperPortfolios.length,
                    itemBuilder: (context, index) {
                      final portfolio = provider.paperPortfolios[index];
                      return PortfolioCard(
                        portfolio: portfolio,
                        onSelect: () => provider.selectPortfolio(portfolio['id']),
                        onDelete: () => _deletePortfolio(context, portfolio),
                        onViewDetails: () => _showPortfolioDetails(context, portfolio),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }

  void _showCreatePortfolioDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => CreatePortfolioDialog(provider: provider),
    );
  }

  void _deletePortfolio(BuildContext context, Map<String, dynamic> portfolio) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Portfolio'),
        content: Text('Are you sure you want to delete ${portfolio['name']}?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              provider.deletePortfolio(portfolio['id']);
              Navigator.of(context).pop();
            },
            style: TextButton.styleFrom(foregroundColor: Colors.red),
            child: const Text('Delete'),
          ),
        ],
      ),
    );
  }

  void _showPortfolioDetails(BuildContext context, Map<String, dynamic> portfolio) {
    showDialog(
      context: context,
      builder: (context) => PortfolioDetailsDialog(portfolio: portfolio),
    );
  }
}

class PortfolioCard extends StatelessWidget {
  final Map<String, dynamic> portfolio;
  final VoidCallback onSelect;
  final VoidCallback onDelete;
  final VoidCallback onViewDetails;

  const PortfolioCard({
    super.key,
    required this.portfolio,
    required this.onSelect,
    required this.onDelete,
    required this.onViewDetails,
  });

  @override
  Widget build(BuildContext context) {
    final name = portfolio['name'] as String;
    final initialBalance = portfolio['initialBalance'] as double;
    final currentBalance = portfolio['currentBalance'] as double;
    final totalReturn = portfolio['totalReturn'] as double;
    final winRate = (portfolio['winRate'] as double?) ?? 0.0;
    final isActive = portfolio['isActive'] as bool;
    
    final returnPercentage = ((currentBalance - initialBalance) / initialBalance * 100);
    final returnColor = returnPercentage >= 0 ? Colors.green : Colors.red;

    return Card(
      color: isActive ? const Color(0xFF37474F) : const Color(0xFF2E2E2E),
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    name,
                    style: TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 16,
                    ),
                  ),
                ),
                if (isActive)
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                    decoration: BoxDecoration(
                      color: Colors.green.withOpacity(0.3),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: Colors.green.withOpacity(0.5)),
                    ),
                    child: const Text(
                      'ACTIVE',
                      style: TextStyle(
                        color: Colors.green,
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Text(
                  '\$${currentBalance.toStringAsFixed(2)}',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const Spacer(),
                Text(
                  '${returnPercentage.toStringAsFixed(2)}%',
                  style: TextStyle(
                    color: returnColor,
                    fontWeight: FontWeight.bold,
                    fontSize: 16,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Text(
                  'Initial: \$${initialBalance.toStringAsFixed(2)}',
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
                const Spacer(),
                if (winRate > 0)
                  Text(
                    'Win Rate: ${(winRate * 100).toInt()}%',
                    style: TextStyle(
                      color: winRate > 0.5 ? Colors.green : Colors.orange,
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
              ],
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: onSelect,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blue,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Trade'),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: ElevatedButton(
                    onPressed: onViewDetails,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.purple,
                      padding: const EdgeInsets.symmetric(vertical: 8),
                    ),
                    child: const Text('Details'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class PaperTradingPanel extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PaperTradingPanel({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    final selectedPortfolio = provider.selectedPortfolio;
    
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
          Row(
            children: [
              const Icon(Icons.trending_up, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              Text(
                selectedPortfolio != null 
                    ? '📈 Paper Trading - ${selectedPortfolio['name']}'
                    : '📈 Paper Trading',
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              if (selectedPortfolio != null) ...[
                IconButton(
                  icon: const Icon(Icons.refresh, color: Colors.white),
                  onPressed: () => provider.refreshPortfolioData(),
                  tooltip: 'Refresh Data',
                ),
                IconButton(
                  icon: const Icon(Icons.settings, color: Colors.white),
                  onPressed: () => _showSettingsDialog(context),
                  tooltip: 'Settings',
                ),
              ],
            ],
          ),
          const SizedBox(height: 16),
          if (selectedPortfolio != null) ...[
            // Advanced Charting
            Expanded(
              flex: 2,
              child: AdvancedChartingWidget(
                symbol: provider.currentSymbol.isEmpty ? 'AAPL' : provider.currentSymbol,
                timeframe: provider.selectedTimeframe,
              ),
            ),
            const SizedBox(height: 8),
            // Paper Trading Controls
            Expanded(
              flex: 1,
              child: PaperTradingControls(provider: provider),
            ),
          ] else ...[
            // No portfolio selected
            const Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.trending_up_outlined,
                      color: Colors.grey,
                      size: 48,
                    ),
                    SizedBox(height: 16),
                    Text(
                      'Select a portfolio to start paper trading',
                      style: TextStyle(color: Colors.grey),
                    ),
                    SizedBox(height: 8),
                    Text(
                      'Create a portfolio from the left panel',
                      style: TextStyle(
                        color: Colors.grey,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }

  void _showSettingsDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => PaperTradingSettingsDialog(provider: provider),
    );
  }
}

class PaperTradingControls extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PaperTradingControls({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF37474F),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Trading Controls',
            style: TextStyle(
              color: Colors.white,
              fontSize: 16,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 16),
          // Quick Order Entry
          Row(
            children: [
              Expanded(
                child: TextField(
                  decoration: const InputDecoration(
                    labelText: 'Symbol',
                    hintText: 'e.g. AAPL',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  style: const TextStyle(color: Colors.white),
                  onChanged: (value) => provider.setSymbol(value),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: TextField(
                  decoration: const InputDecoration(
                    labelText: 'Quantity',
                    hintText: '0',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  style: const TextStyle(color: Colors.white),
                  keyboardType: TextInputType.number,
                  onChanged: (value) => provider.setQuantity(double.tryParse(value) ?? 0.0),
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          // Order Type Selection
          Row(
            children: [
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Order Type',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.selectedOrderType,
                  items: ['Market', 'Limit', 'Stop', 'StopLimit'].map((type) {
                    return DropdownMenuItem(
                      value: type,
                      child: Text(type, style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setOrderType(value);
                    }
                  },
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Side',
                    border: OutlineInputBorder(),
                    labelStyle: TextStyle(color: Colors.white),
                  ),
                  value: provider.selectedSide,
                  items: ['Buy', 'Sell'].map((side) {
                    return DropdownMenuItem(
                      value: side,
                      child: Text(side, style: const TextStyle(color: Colors.white)),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      provider.setSide(value);
                    }
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          // Price for limit orders
          if (provider.selectedOrderType == 'Limit' || provider.selectedOrderType == 'StopLimit')
            TextField(
              decoration: const InputDecoration(
                labelText: 'Price',
                hintText: '0.00',
                border: OutlineInputBorder(),
                labelStyle: TextStyle(color: Colors.white),
              ),
              style: const TextStyle(color: Colors.white),
              keyboardType: TextInputType.number,
              onChanged: (value) => provider.setPrice(double.tryParse(value)),
            ),
          const SizedBox(height: 16),
          // Action Buttons
          Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: provider.canPlaceOrder ? () => _placePaperOrder(context) : null,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: provider.selectedSide == 'Buy' ? Colors.green : Colors.red,
                    padding: const EdgeInsets.symmetric(vertical: 12),
                  ),
                  child: Text(
                    provider.selectedSide == 'Buy' ? 'Paper Buy' : 'Paper Sell',
                  ),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: ElevatedButton(
                  onPressed: () => _showPositionsDialog(context),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.blue,
                    padding: const EdgeInsets.symmetric(vertical: 12),
                  ),
                  child: const Text('Positions'),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  void _placePaperOrder(BuildContext context) {
    provider.placePaperOrder();
    
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Paper ${provider.selectedSide} order placed'),
        backgroundColor: Colors.green,
      ),
    );
  }

  void _showPositionsDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => PaperPositionsDialog(provider: provider),
    );
  }
}

class PerformanceAnalyticsPanel extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PerformanceAnalyticsPanel({
    super.key,
    required this.provider,
  });

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
          Row(
            children: [
              const Icon(Icons.analytics, color: Colors.white, size: 20),
              const SizedBox(width: 8),
              const Text(
                '📊 Performance Analytics',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.refresh, color: Colors.white),
                onPressed: () => provider.refreshAnalytics(),
                tooltip: 'Refresh Analytics',
              ),
            ],
          ),
          const SizedBox(height: 16),
          // Performance Overview
          if (provider.selectedPortfolio != null)
            PerformanceOverviewCard(portfolio: provider.selectedPortfolio!),
          if (provider.selectedPortfolio != null) ...[
            const SizedBox(height: 16),
            // Trade History
            Expanded(
              child: TradeHistoryList(provider: provider),
            ),
          ] else ...[
            const Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.analytics_outlined,
                      color: Colors.grey,
                      size: 48,
                    ),
                    SizedBox(height: 16),
                    Text(
                      'No performance data',
                      style: TextStyle(color: Colors.grey),
                    ),
                    SizedBox(height: 8),
                    Text(
                      'Select a portfolio to view analytics',
                      style: TextStyle(
                        color: Colors.grey,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

class PerformanceOverviewCard extends StatelessWidget {
  final Map<String, dynamic> portfolio;
  
  const PerformanceOverviewCard({
    super.key,
    required this.portfolio,
  });

  @override
  Widget build(BuildContext context) {
    final totalReturn = portfolio['totalReturn'] as double;
    final totalReturnPercentage = portfolio['totalReturnPercentage'] as double;
    final winRate = (portfolio['winRate'] as double?) ?? 0.0;
    final sharpeRatio = (portfolio['sharpeRatio'] as double?) ?? 0.0;
    final maxDrawdown = (portfolio['maxDrawdown'] as double?) ?? 0.0;
    final totalTrades = portfolio['totalTrades'] as int;
    
    return Card(
      color: const Color(0xFF37474F),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            Row(
              children: [
                _buildMetricCard('Total Return', '${totalReturnPercentage.toStringAsFixed(2)}%', 
                  totalReturn >= 0 ? Colors.green : Colors.red),
                const SizedBox(width: 8),
                _buildMetricCard('Win Rate', '${(winRate * 100).toInt()}%', 
                  winRate > 0.5 ? Colors.green : Colors.orange),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                _buildMetricCard('Sharpe Ratio', sharpeRatio.toStringAsFixed(2), 
                  sharpeRatio > 1.0 ? Colors.green : Colors.grey),
                const SizedBox(width: 8),
                _buildMetricCard('Max Drawdown', '${maxDrawdown.toStringAsFixed(2)}%', 
                  Colors.red),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                _buildMetricCard('Total Trades', totalTrades.toString(), Colors.blue),
                const SizedBox(width: 8),
                _buildMetricCard('Active Days', '45', Colors.purple),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMetricCard(String label, String value, Color color) {
    return Expanded(
      child: Container(
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: color.withOpacity(0.1),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: color.withOpacity(0.3)),
        ),
        child: Column(
          children: [
            Text(
              value,
              style: TextStyle(
                color: color,
                fontSize: 16,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              label,
              style: const TextStyle(
                color: Colors.grey,
                fontSize: 12,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class TradeHistoryList extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const TradeHistoryList({
    super.key,
    required this.provider,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Recent Trades',
          style: TextStyle(
            color: Colors.white,
            fontSize: 14,
            fontWeight: FontWeight.bold,
          ),
        ),
        const SizedBox(height: 8),
        Expanded(
          child: ListView.builder(
            itemCount: provider.recentTrades.length,
            itemBuilder: (context, index) {
              final trade = provider.recentTrades[index];
              return PaperTradeCard(trade: trade);
            },
          ),
        ),
      ],
    );
  }
}

class PaperTradeCard extends StatelessWidget {
  final Map<String, dynamic> trade;
  
  const PaperTradeCard({
    super.key,
    required this.trade,
  });

  @override
  Widget build(BuildContext context) {
    final symbol = trade['symbol'] as String;
    final side = trade['side'] as String;
    final quantity = trade['quantity'] as double;
    final entryPrice = trade['entryPrice'] as double;
    final exitPrice = trade['exitPrice'] as double?;
    final pnl = trade['pnl'] as double?;
    final status = trade['status'] as String;
    final timestamp = trade['timestamp'] as DateTime;
    
    final sideColor = side == 'Buy' ? Colors.green : Colors.red;
    final pnlColor = (pnl ?? 0.0) >= 0 ? Colors.green : Colors.red;

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            Container(
              width: 4,
              height: 40,
              decoration: BoxDecoration(
                color: sideColor,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(
                        symbol,
                        style: const TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const Spacer(),
                      Text(
                        side,
                        style: TextStyle(
                          color: sideColor,
                          fontWeight: FontWeight.bold,
                          fontSize: 12,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      Text(
                        '$quantity @ \$${entryPrice.toStringAsFixed(2)}',
                        style: const TextStyle(
                          color: Colors.grey,
                          fontSize: 12,
                        ),
                      ),
                      if (exitPrice != null) ...[
                        const SizedBox(width: 8),
                        Text(
                          '→ \$${exitPrice.toStringAsFixed(2)}',
                          style: const TextStyle(
                            color: Colors.grey,
                            fontSize: 12,
                          ),
                        ),
                      ],
                      const Spacer(),
                      if (pnl != null)
                        Text(
                          '\$${pnl!.toStringAsFixed(2)}',
                          style: TextStyle(
                            color: pnlColor,
                            fontWeight: FontWeight.bold,
                            fontSize: 12,
                          ),
                        ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// Dialog classes would go here...
class CreatePortfolioDialog extends StatefulWidget {
  final PaperHandsProvider provider;
  
  const CreatePortfolioDialog({super.key, required this.provider});

  @override
  State<CreatePortfolioDialog> createState() => _CreatePortfolioDialogState();
}

class _CreatePortfolioDialogState extends State<CreatePortfolioDialog> {
  final _nameController = TextEditingController();
  final _balanceController = TextEditingController();
  
  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Create Paper Portfolio'),
      content: SizedBox(
        width: 400,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _nameController,
              decoration: const InputDecoration(
                labelText: 'Portfolio Name',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _balanceController,
              decoration: const InputDecoration(
                labelText: 'Initial Balance',
                border: OutlineInputBorder(),
                prefixText: '\$',
              ),
              keyboardType: TextInputType.number,
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            final name = _nameController.text.trim();
            final balance = double.tryParse(_balanceController.text) ?? 10000.0;
            
            if (name.isNotEmpty) {
              widget.provider.createPortfolio(name, balance);
              Navigator.of(context).pop();
            }
          },
          child: const Text('Create'),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _nameController.dispose();
    _balanceController.dispose();
    super.dispose();
  }
}

class PortfolioDetailsDialog extends StatelessWidget {
  final Map<String, dynamic> portfolio;
  
  const PortfolioDetailsDialog({super.key, required this.portfolio});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(portfolio['name']),
      content: SizedBox(
        width: 500,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Initial Balance: \$${portfolio['initialBalance']}'),
            const SizedBox(height: 8),
            Text('Current Balance: \$${portfolio['currentBalance']}'),
            const SizedBox(height: 8),
            Text('Total Return: ${portfolio['totalReturnPercentage']}%'),
            const SizedBox(height: 8),
            Text('Win Rate: ${((portfolio['winRate'] ?? 0.0) * 100).toInt()}%'),
            const SizedBox(height: 8),
            Text('Total Trades: ${portfolio['totalTrades']}'),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    );
  }
}

class PaperTradingSettingsDialog extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PaperTradingSettingsDialog({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Paper Trading Settings'),
      content: const Text('Settings dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Save'),
        ),
      ],
    );
  }
}

class PaperPositionsDialog extends StatelessWidget {
  final PaperHandsProvider provider;
  
  const PaperPositionsDialog({super.key, required this.provider});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Current Positions'),
      content: const Text('Positions dialog implementation...'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Close'),
        ),
      ],
    );
  }
}
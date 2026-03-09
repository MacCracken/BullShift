import 'package:flutter/material.dart';
import '../../paper_hands_provider.dart';
import '../cards/portfolio_card.dart';
import '../dialogs/create_portfolio_dialog.dart';
import '../dialogs/portfolio_details_dialog.dart';

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
              const Icon(Icons.account_balance_wallet,
                  color: Colors.white, size: 20),
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
                        onSelect: () =>
                            provider.selectPortfolio(portfolio['id']),
                        onDelete: () => _deletePortfolio(context, portfolio),
                        onViewDetails: () =>
                            _showPortfolioDetails(context, portfolio),
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

  void _showPortfolioDetails(
      BuildContext context, Map<String, dynamic> portfolio) {
    showDialog(
      context: context,
      builder: (context) => PortfolioDetailsDialog(portfolio: portfolio),
    );
  }
}

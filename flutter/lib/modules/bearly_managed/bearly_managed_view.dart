import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'bearly_managed_provider.dart';
import 'widgets/panels/provider_setup_panel.dart';
import 'widgets/panels/strategy_generation_panel.dart';
import 'widgets/panels/prompt_management_panel.dart';

class BearlyManagedView extends StatelessWidget {
  const BearlyManagedView({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<BearlyManagedProvider>(
      builder: (context, provider, child) {
        return Row(
          children: [
            // AI Provider Setup
            Expanded(
              flex: 1,
              child: ProviderSetupPanel(provider: provider),
            ),
            // Strategy Generation
            Expanded(
              flex: 1,
              child: StrategyGenerationPanel(provider: provider),
            ),
            // AI Prompt Management
            Expanded(
              flex: 1,
              child: PromptManagementPanel(provider: provider),
            ),
          ],
        );
      },
    );
  }
}

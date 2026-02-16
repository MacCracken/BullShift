# Troubleshooting Guide

## Overview

This guide helps you diagnose and resolve common issues with BullShift Trading Platform. Problems are organized by category with step-by-step solutions.

## Table of Contents

1. [Installation & Setup Issues](#installation--setup-issues)
2. [Connection & Network Problems](#connection--network-problems)
3. [Trading & Order Issues](#trading--order-issues)
4. [Data & Market Feed Problems](#data--market-feed-problems)
5. [AI Provider Issues](#ai-provider-issues)
6. [Security & Authentication Problems](#security--authentication-problems)
7. [Performance Issues](#performance-issues)
8. [Mobile App Issues](#mobile-app-issues)
9. [Platform-Specific Issues](#platform-specific-issues)
10. [Getting Help](#getting-help)

---

## Installation & Setup Issues

### Problem: Application Won't Start

#### Linux
**Symptoms:** Nothing happens when running `./bullshift` or `flutter run`

**Solutions:**
1. **Check Dependencies**
   ```bash
   # Verify required packages
   dpkg -l | grep -E "(libsecret|webkit|gtk)"
   
   # Install missing dependencies
   sudo apt install libsecret-1-dev libwebkit2gtk-4.0-dev
   ```

2. **Check Permissions**
   ```bash
   # Make executable
   chmod +x bullshift
   
   # Check if binary exists
   ls -la bullshift
   ```

3. **Verify Flutter Installation**
   ```bash
   flutter doctor -v
   # Fix any issues shown
   ```

#### macOS
**Symptoms:** "BullShift can't be opened" or crash on launch

**Solutions:**
1. **Allow App from Unknown Developer**
   ```
   System Preferences → Security & Privacy → General
   Click "Open Anyway" for BullShift
   ```

2. **Check System Integrity**
   ```bash
   # Verify app signature
   codesign -vv /Applications/BullShift.app
   ```

3. **Reinstall Dependencies**
   ```bash
   # Reinstall with Homebrew
   brew reinstall cmake pkg-config
   ```

#### Windows
**Symptoms:** "BullShift.exe has stopped working" or missing DLL errors

**Solutions:**
1. **Install Visual C++ Redistributable**
   - Download from Microsoft website
   - Install both x86 and x64 versions

2. **Check Windows Version**
   ```
   # Requires Windows 10/11 (64-bit)
   winver
   ```

3. **Run as Administrator**
   - Right-click BullShift.exe
   - "Run as administrator"

### Problem: Build Fails

#### Rust Compilation Errors
**Symptoms:** `cargo build` fails with errors

**Solutions:**
1. **Update Rust Toolchain**
   ```bash
   rustup update
   rustup component add rustfmt clippy
   ```

2. **Clean Build Cache**
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Check Target Platform**
   ```bash
   # Verify target is installed
   rustup target list --installed
   rustup target add x86_64-unknown-linux-gnu  # Linux
   rustup target add x86_64-apple-darwin       # macOS
   rustup target add x86_64-pc-windows-msvc     # Windows
   ```

#### Flutter Build Errors
**Symptoms:** `flutter build` fails

**Solutions:**
1. **Clean Flutter Cache**
   ```bash
   flutter clean
   flutter pub get
   flutter doctor
   ```

2. **Check Platform Support**
   ```bash
   # Verify platform is enabled
   flutter config --enable-linux-desktop
   flutter config --enable-macos-desktop
   flutter config --enable-windows-desktop
   ```

3. **Reinstall Flutter**
   ```bash
   # Last resort
   rm -rf ~/.flutter
   # Reinstall Flutter SDK
   ```

---

## Connection & Network Problems

### Problem: Cannot Connect to Broker

#### Alpaca Connection Failed
**Symptoms:** "Authentication failed" or "Connection timeout"

**Solutions:**
1. **Verify API Credentials**
   ```bash
   # Test with curl
   curl -X GET "https://paper-api.alpaca.markets/v2/account" \
     -H "APCA-API-KEY-ID: PK_YOUR_KEY" \
     -H "APCA-API-SECRET-KEY: sk_YOUR_SECRET"
   ```

2. **Check Environment**
   - Ensure using paper trading URL for paper accounts
   - Verify account is funded and active
   - Check API key permissions

3. **Network Diagnostics**
   ```bash
   # Test connectivity
   ping paper-api.alpaca.markets
   nslookup paper-api.alpaca.markets
   
   # Test HTTPS
   curl -I https://paper-api.alpaca.markets
   ```

#### Interactive Brokers Connection Failed
**Symptoms:** "Cannot connect to TWS" or "Socket error"

**Solutions:**
1. **Start TWS/Gateway**
   - Launch Trader Workstation
   - Enable API connections: Configure → API → Settings
   - Set socket port (default: 7497)

2. **Check Local Firewall**
   ```bash
   # Test local connection
   telnet 127.0.0.1 7497
   
   # Check if port is listening
   netstat -an | grep 7497
   ```

3. **Verify API Settings**
   - Enable "Enable ActiveX and Socket Clients"
   - Set "Read-Only API" to "No" for trading
   - Create API password if required

### Problem: WebSocket Data Not Updating

#### Market Data Stale
**Symptoms:** Prices not updating in real-time

**Solutions:**
1. **Check WebSocket Connection**
   ```bash
   # Test with wscat
   wscat -c wss://stream.data.alpaca.markets/v2/iex
   ```

2. **Verify Subscription**
   ```dart
   // Check if symbols are subscribed
   final subscription = marketDataProvider.subscribeToQuotes(['AAPL']);
   subscription.listen((quote) => print('Received: $quote'));
   ```

3. **Network Diagnostics**
   ```bash
   # Check WebSocket port
   telnet stream.data.alpaca.markets 443
   
   # Test with OpenSSL
   openssl s_client -connect stream.data.alpaca.markets:443
   ```

#### Connection Drops Frequently
**Symptoms:** Real-time data disconnects periodically

**Solutions:**
1. **Increase Timeout Settings**
   ```json
   {
     "network": {
       "timeout": 60,
       "keep_alive": true,
       "retry_attempts": 5
     }
   }
   ```

2. **Check Network Stability**
   ```bash
   # Test connection stability
   ping -c 100 8.8.8.8
   
   # Check packet loss
   mtr google.com
   ```

3. **Proxy Configuration**
   - Check if corporate proxy interferes
   - Configure proxy settings if needed
   - Use direct connection if possible

---

## Trading & Order Issues

### Problem: Order Rejected

#### Insufficient Buying Power
**Symptoms:** "Order rejected: Insufficient buying power"

**Solutions:**
1. **Check Account Balance**
   ```dart
   final account = await tradingProvider.getAccount();
   print('Buying Power: ${account.buyingPower}');
   print('Cash: ${account.cash}');
   ```

2. **Verify Order Size**
   ```dart
   // Calculate required buying power
   final required = order.quantity * order.price;
   final available = account.buyingPower;
   
   if (required > available) {
     print('Insufficient funds: need $required, have $available');
   }
   ```

3. **Check Margin Requirements**
   - Verify margin account status
   - Check pattern day trader rules
   - Review overnight margin requirements

#### Market Hours Issue
**Symptoms:** "Order rejected: Market closed"

**Solutions:**
1. **Check Market Hours**
   ```dart
   final marketHours = await marketDataProvider.getMarketHours();
   print('Market Open: ${marketHours.isOpen}');
   print('Next Open: ${marketHours.nextOpen}');
   ```

2. **Use Extended Hours**
   ```dart
   final order = OrderRequest(
     // ... other fields
     extendedHours: true, // Enable pre/after market
   );
   ```

3. **Schedule Order**
   - Use limit orders for market open
   - Set order duration to "DAY" or "GTC"

#### Invalid Symbol
**Symptoms:** "Order rejected: Symbol not found"

**Solutions:**
1. **Verify Symbol Format**
   ```dart
   // Search for correct symbol
   final symbols = await tradingProvider.searchSymbols('Apple');
   print('Correct symbol: ${symbols.first.symbol}');
   ```

2. **Check Exchange**
   - Verify symbol trades on supported exchange
   - Check if symbol is delisted or suspended
   - Use full symbol with exchange suffix if needed

### Problem: Position Not Updating

#### Stale Position Data
**Symptoms:** Position P&L not updating or showing old data

**Solutions:**
1. **Refresh Positions**
   ```dart
   await tradingProvider.refreshPositions();
   final positions = await tradingProvider.getPositions();
   ```

2. **Check Data Source**
   ```dart
   // Verify position data source
   for (final position in positions) {
     print('${position.symbol}: ${position.source}');
   }
   ```

3. **Manual Sync**
   - Click refresh button in UI
   - Restart application
   - Clear cache and re-sync

---

## Data & Market Feed Problems

### Problem: No Market Data

#### Data Subscription Issue
**Symptoms:** No price data or "No data available" messages

**Solutions:**
1. **Check Data Subscription**
   ```dart
   // Verify data provider status
   final providers = await marketDataProvider.getProviders();
   for (final provider in providers) {
     print('${provider.name}: ${provider.status}');
   }
   ```

2. **Test Data Access**
   ```dart
   // Test with known symbol
   final quote = await marketDataProvider.getQuote('AAPL');
   if (quote == null) {
     print('No data available for AAPL');
   }
   ```

3. **Check API Limits**
   - Verify data API key is active
   - Check usage limits
   - Review billing status

#### Historical Data Missing
**Symptoms:** Historical charts empty or incomplete

**Solutions:**
1. **Check Date Range**
   ```dart
   // Verify date range is valid
   final now = DateTime.now();
   final start = now.subtract(Duration(days: 30));
   
   if (start.isAfter(now)) {
     print('Invalid date range');
   }
   ```

2. **Test Different Timeframes**
   ```dart
   // Try different timeframes
   for (final tf in [TimeFrame.oneDay, TimeFrame.oneHour, TimeFrame.fiveMinute]) {
     final bars = await marketDataProvider.getHistoricalBars(
       symbol: 'AAPL',
       timeframe: tf,
       start: start,
       end: now,
     );
     print('$tf: ${bars.length} bars');
   }
   ```

3. **Check Symbol History**
   - Verify symbol has sufficient trading history
   - Check for corporate actions (splits, dividends)
   - Try different symbols for testing

---

## AI Provider Issues

### Problem: AI Requests Failing

#### OpenAI API Error
**Symptoms:** "AI request failed" or "Invalid API key"

**Solutions:**
1. **Verify API Key**
   ```bash
   # Test OpenAI API
   curl -X POST "https://api.openai.com/v1/chat/completions" \
     -H "Authorization: Bearer sk-your-key" \
     -H "Content-Type: application/json" \
     -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello"}]}'
   ```

2. **Check Billing Status**
   - Visit OpenAI dashboard
   - Verify payment method on file
   - Check credit balance

3. **Test Different Model**
   ```dart
   // Try with different model
   final response = await provider.sendPrompt(
     providerId: 'openai',
     prompt: 'Test message',
     model: 'gpt-3.5-turbo', // Instead of gpt-4
   );
   ```

#### Ollama Connection Failed
**Symptoms:** "Cannot connect to Ollama" or "Model not found"

**Solutions:**
1. **Start Ollama Server**
   ```bash
   # Start Ollama
   ollama serve
   
   # Test connection
   curl http://localhost:11434/api/tags
   ```

2. **Download Required Models**
   ```bash
   # List available models
   ollama list
   
   # Download model if missing
   ollama pull llama2
   ```

3. **Check Configuration**
   ```json
   {
     "ai_providers": {
       "ollama": {
         "api_endpoint": "http://localhost:11434",
         "model": "llama2",
         "timeout": 60
       }
     }
   }
   ```

### Problem: AI Responses Slow or Incomplete

#### Performance Issues
**Symptoms:** AI requests taking too long or timing out

**Solutions:**
1. **Increase Timeout**
   ```json
   {
     "ai_providers": {
       "openai": {
         "timeout": 120, // Increase to 2 minutes
         "max_tokens": 2048 // Reduce if needed
       }
     }
   }
   ```

2. **Optimize Prompts**
   ```dart
   // Use shorter, more specific prompts
   final prompt = "Analyze AAPL fundamentals in 100 words";
   
   // Instead of long, vague prompts
   final badPrompt = "Please provide a comprehensive analysis of Apple Inc. including its financial performance, competitive position, future prospects, and investment recommendation...";
   ```

3. **Choose Faster Models**
   ```dart
   // Use faster model for quick analysis
   final response = await provider.sendPrompt(
     model: 'gpt-3.5-turbo', // Faster than gpt-4
     prompt: 'Quick market summary',
   );
   ```

---

## Security & Authentication Problems

### Problem: Cannot Store Credentials

#### Keychain/Keyring Issues
**Symptoms:** "Failed to store credentials" or "Keychain access denied"

**Solutions:**
1. **macOS Keychain**
   ```bash
   # Unlock keychain
   security unlock-keychain ~/Library/Keychains/login.keychain-db
   
   # Test keychain access
   security add-generic-password -a test -s test -p test
   ```

2. **Linux Keyring**
   ```bash
   # Test libsecret
   secret-tool store --label=test test test
   
   # Check if keyring is unlocked
   gnome-keyring-daemon --start
   ```

3. **Windows Credential Manager**
   ```cmd
   # Test credential storage
   cmdkey /add:test /user:test /pass:test
   cmdkey /list:test
   ```

#### Master Password Issues
**Symptoms:** "Invalid master password" or "Cannot decrypt credentials"

**Solutions:**
1. **Reset Master Password**
   ```bash
   # Backup current config
   cp ~/.config/bullshift/config.json ~/.config/bullshift/config.backup
   
   # Remove encrypted credentials
   rm ~/.config/bullshift/credentials.enc
   
   # Restart and set new master password
   ```

2. **Verify Password**
   - Check for typos in master password
   - Ensure caps lock is off
   - Try password recovery if available

### Problem: API Keys Exposed

#### Accidental Commit
**Symptoms:** API keys committed to version control

**Solutions:**
1. **Remove from History**
   ```bash
   # Remove sensitive file from history
   git filter-branch --force --index-filter \
     'git rm --cached --ignore-unmatch config.json' \
     --prune-empty --tag-name-filter cat -- --all
   
   # Force push
   git push origin --force --all
   ```

2. **Regenerate API Keys**
   - Revoke old API keys immediately
   - Generate new API keys
   - Update configuration with new keys

3. **Set Up Git Hooks**
   ```bash
   # Add pre-commit hook to check for API keys
   cat > .git/hooks/pre-commit << 'EOF'
   #!/bin/sh
   if git diff --cached --name-only | xargs grep -l "sk-\|PK-" 2>/dev/null; then
     echo "API keys detected! Remove them before committing."
     exit 1
   fi
   EOF
   chmod +x .git/hooks/pre-commit
   ```

---

## Performance Issues

### Problem: Application Slow

#### High CPU Usage
**Symptoms:** Application using excessive CPU

**Solutions:**
1. **Profile Application**
   ```bash
   # Flutter performance profiling
   flutter run --profile
   flutter run --trace-startup --profile
   ```

2. **Check for Infinite Loops**
   ```dart
   // Look for patterns like this
   while (true) {
     await someAsyncOperation(); // Should have break condition
   }
   ```

3. **Optimize UI Updates**
   ```dart
   // Use const widgets where possible
   const MyWidget({Key? key}) : super(key: key);
   
   // Avoid unnecessary rebuilds
   class MyProvider extends ChangeNotifier {
     Timer? _timer;
     
     void startUpdates() {
       _timer?.cancel(); // Cancel previous timer
       _timer = Timer.periodic(Duration(seconds: 1), (_) {
         notifyListeners();
       });
     }
   }
   ```

#### High Memory Usage
**Symptoms:** Application memory usage growing continuously

**Solutions:**
1. **Check for Memory Leaks**
   ```dart
   // Dispose resources properly
   class MyWidget extends StatefulWidget {
     @override
     _MyWidgetState createState() => _MyWidgetState();
   }
   
   class _MyWidgetState extends State<MyWidget> {
     StreamSubscription? _subscription;
     
     @override
     void dispose() {
       _subscription?.cancel(); // Cancel subscription
       super.dispose();
     }
   }
   ```

2. **Optimize Image Loading**
   ```dart
   // Use cached images
   Image.network(
     'https://example.com/image.jpg',
     cacheWidth: 300, // Limit image size
     cacheHeight: 300,
   )
   ```

3. **Monitor Memory Usage**
   ```bash
   # Monitor with system tools
   top -p $(pgrep bullshift)
   htop -p $(pgrep bullshift)
   
   # Or with Dart Observatory
   flutter run --profile
   # Open http://localhost:8080 in browser
   ```

---

## Mobile App Issues

### Problem: Mobile App Crashes

#### iOS App Crashes
**Symptoms:** App crashes on launch or during use

**Solutions:**
1. **Check Device Compatibility**
   - Verify iOS version (15+ required)
   - Check device storage space
   - Test on different iOS versions

2. **Review Crash Logs**
   ```
   Xcode → Window → Devices and Simulators
   Select device → View Device Logs
   ```

3. **Rebuild App**
   ```bash
   flutter clean
   flutter pub get
   flutter build ios --release
   ```

#### Android App Crashes
**Symptoms:** App crashes or ANR (Application Not Responding)

**Solutions:**
1. **Check Android Version**
   - Verify Android version (8.0+ required)
   - Check available RAM
   - Test on different Android versions

2. **Review Logcat**
   ```bash
   adb logcat | grep bullshift
   ```

3. **Check Permissions**
   ```xml
   <!-- AndroidManifest.xml -->
   <uses-permission android:name="android.permission.INTERNET" />
   <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
   ```

### Problem: Sync Issues

#### Desktop-Mobile Sync Failed
**Symptoms:** Changes on desktop not appearing on mobile

**Solutions:**
1. **Check Cloud Sync Status**
   ```dart
   // Verify sync is enabled
   final syncStatus = await cloudSyncProvider.getStatus();
   print('Sync enabled: ${syncStatus.enabled}');
   print('Last sync: ${syncStatus.lastSync}');
   ```

2. **Force Manual Sync**
   ```dart
   await cloudSyncProvider.forceSync();
   ```

3. **Check Network Connection**
   - Verify mobile device has internet
   - Check cloud service status
   - Try switching between WiFi and cellular

---

## Platform-Specific Issues

### Linux Issues

#### Wayland Compatibility
**Symptoms:** Window not rendering or input not working on Wayland

**Solutions:**
1. **Force X11 Backend**
   ```bash
   export GDK_BACKEND=x11
   flutter run -d linux
   ```

2. **Use XWayland**
   ```bash
   # Install XWayland if not present
   sudo apt install xwayland
   ```

#### Package Dependencies
**Symptoms:** Missing library errors

**Solutions:**
1. **Install Missing Packages**
   ```bash
   # Ubuntu/Debian
   sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev
   
   # Fedora
   sudo dnf install gtk3-devel webkit2gtk3-devel
   
   # Arch
   sudo pacman -S gtk3 webkit2gtk
   ```

### macOS Issues

#### Apple Silicon Compatibility
**Symptoms:** "Architecture mismatch" or "Cannot run on this Mac"

**Solutions:**
1. **Build for Apple Silicon**
   ```bash
   # Build for arm64
   flutter build macos --release --arch=arm64
   
   # Or universal binary
   flutter build macos --release --arch=arm64 --arch=x64
   ```

2. **Check Rosetta**
   ```bash
   # Install Rosetta 2 if needed
   softwareupdate --install-rosetta --agree-to-license
   
   # Run with Rosetta
   arch -x86_64 flutter run -d macos
   ```

#### Notarization Issues
**Symptoms:** "App cannot be opened because Apple cannot check it"

**Solutions:**
1. **Bypass Gatekeeper**
   ```bash
   # Allow app to run
   xattr -rd com.apple.quarantine /Applications/BullShift.app
   ```

2. **Sign Application**
   ```bash
   # Sign with developer certificate
   codesign --force --deep --sign "Developer ID" BullShift.app
   ```

### Windows Issues

#### DLL Missing Errors
**Symptoms:** "DLL not found" or "MSVCR120.dll missing"

**Solutions:**
1. **Install Visual C++ Redistributable**
   - Download from Microsoft website
   - Install both x86 and x64 versions

2. **Use Static Linking**
   ```toml
   # Cargo.toml
   [dependencies]
   # Use static versions where possible
   ```

#### Windows Defender Blocking
**Symptoms:** "Windows Defender prevented this app from running"

**Solutions:**
1. **Add Exclusion**
   ```
   Windows Security → Virus & threat protection
   → Manage settings → Add or remove exclusions
   Add folder: C:\Program Files\BullShift
   ```

2. **Submit for Review**
   - Submit application to Microsoft for review
   - Request removal from false positive list

---

## Getting Help

### Self-Service Resources

#### Documentation
- **[User Manual](user_manual.md)** - Complete usage guide
- **[Configuration Guide](configuration_guide.md)** - Setup instructions
- **[API Reference](api_reference.md)** - Developer documentation
- **[Security Guide](security.md)** - Security policies

#### Diagnostic Tools
```bash
# System information
flutter doctor -v
rustc --version
cargo --version

# Network diagnostics
ping -c 3 google.com
nslookup api.bullshift.io

# Application logs
tail -f ~/.local/share/bullshift/logs/app.log
```

### Community Support

#### GitHub Issues
- **[Bug Reports](https://github.com/yourrepo/bullshift/issues)** - Report bugs
- **[Feature Requests](https://github.com/yourrepo/bullshift/issues)** - Request features
- **[Discussions](https://github.com/yourrepo/bullshift/discussions)** - Community help

#### Forums and Chat
- **[Discord Server](https://discord.gg/bullshift)** - Real-time chat
- **[Reddit Community](https://reddit.com/r/bullshift)** - User discussions
- **[Stack Overflow](https://stackoverflow.com/questions/tagged/bullshift)** - Technical questions

### Professional Support

#### Contact Information
- **Email Support:** support@bullshift.io
- **Urgent Issues:** urgent@bullshift.io
- **Security Issues:** security@bullshift.io

#### Support Hours
- **Standard Support:** Monday-Friday, 9 AM - 5 PM EST
- **Urgent Support:** 24/7 for critical trading issues
- **Security Issues:** Immediate response

### Reporting Issues

#### Bug Report Template
```markdown
## Description
Brief description of the issue

## Steps to Reproduce
1. Go to...
2. Click on...
3. See error

## Expected Behavior
What should have happened

## Actual Behavior
What actually happened

## Environment
- OS: [e.g., Ubuntu 22.04]
- Version: [e.g., 1.0.0]
- Platform: [e.g., Desktop, Mobile]

## Logs
[Attach relevant log files]

## Additional Context
[Any other relevant information]
```

#### Feature Request Template
```markdown
## Feature Description
Clear description of the feature

## Problem Statement
What problem does this solve?

## Proposed Solution
How should this work?

## Alternatives Considered
Other approaches considered

## Additional Context
Any other relevant information
```

---

## Quick Fixes

### Common Quick Solutions

| Problem | Quick Fix |
|---------|-----------|
| App won't start | `flutter clean && flutter pub get` |
| No data | Check internet connection and API keys |
| Orders rejected | Verify buying power and market hours |
| Slow performance | Restart application and check resources |
| Sync issues | Force manual sync and check network |
| Mobile crashes | Clear app cache and reinstall |

### Emergency Procedures

#### Trading Emergency
1. **Stop all automated strategies**
2. **Close all positions manually if needed**
3. **Contact broker directly if platform unavailable**
4. **Document issue for support ticket**

#### Security Emergency
1. **Revoke all API keys immediately**
2. **Change master password**
3. **Contact security team**
4. **Monitor account for unauthorized activity**

---

**Troubleshooting Guide Version: 1.0**  
**Last Updated: February 10, 2026**

For additional help, see our [Support Center](https://support.bullshift.io) or contact us directly.
# BullShift User Manual

## 📚 Table of Contents

1. [Getting Started](#getting-started)
2. [Account Setup](#account-setup)
3. [Core Trading](#core-trading)
4. [Market Analytics](#market-analytics)
5. [News & Sentiment](#news--sentiment)
6. [AI Integration](#ai-integration)
7. [Paper Trading](#paper-trading)
8. [Mobile Apps](#mobile-apps)
9. [Security](#security)
10. [Troubleshooting](#troubleshooting)

---

## Getting Started

### System Requirements

**Desktop:**
- **Linux**: Ubuntu 20.04+, libsecret-1-dev
- **macOS**: macOS 11+ (Intel + Apple Silicon)
- **Windows**: Windows 10/11 (64-bit)

**Mobile:**
- **iOS**: iOS 15+ (iPhone/iPad)
- **Android**: Android 8.0+ (API 26+)

### Installation

#### Option 1: Pre-built Binary
1. Download from [Releases](https://github.com/yourrepo/bullshift/releases)
2. Install appropriate version for your platform
3. Launch application

#### Option 2: Build from Source
1. [Set up development environment](development_setup.md)
2. Run `./build.sh`
3. Launch with `flutter run -d [platform]`

### First Launch

1. **Welcome Screen** - Choose your trading experience level
2. **Security Setup** - Create master password for credential encryption
3. **Broker Selection** - Choose and configure your trading broker
4. **Tutorial** - Complete interactive tutorial (recommended)

---

## Account Setup

### Trading Broker Configuration

#### Supported Brokers
- **Alpaca Markets** - Stocks, ETFs, Crypto
- **Interactive Brokers** - Global markets (coming soon)

#### Alpaca Setup
1. **Create Alpaca Account**
   - Visit [alpaca.markets](https://alpaca.markets)
   - Choose Paper or Live trading account
   - Complete identity verification

2. **Generate API Keys**
   ```text
   Login to Alpaca → Settings → API Keys
   Click "Generate New Key"
   Save API Key and Secret securely
   ```

3. **Configure in BullShift**
   - Open **Settings** → **Trading Accounts**
   - Click **Add Broker**
   - Select **Alpaca**
   - Enter API Key and Secret
   - Test connection
   - Save configuration

### AI Provider Setup

#### Supported AI Providers
- **OpenAI** - GPT-4, GPT-3.5
- **Anthropic** - Claude 3
- **Ollama** - Local LLM models
- **Custom** - Any OpenAI-compatible API

#### OpenAI Setup
1. **Create OpenAI Account**
   - Visit [openai.com](https://openai.com)
   - Verify email and phone

2. **Generate API Key**
   ```text
   OpenAI Dashboard → API Keys
   Click "Create new secret key"
   Copy key immediately (won't show again)
   ```

3. **Configure in BullShift**
   - **AI Module** → **Provider Setup**
   - **Add Provider** → **OpenAI**
   - Enter API Key
   - Select Model (GPT-4 recommended)
   - Test connection
   - Save configuration

---

## Core Trading

### Dashboard Overview

The main trading dashboard consists of:

- **Watchlist** - Your monitored symbols
- **Position Panel** - Current positions with P&L
- **Order Entry** - Quick order placement
- **Chart View** - Real-time price charts
- **News Feed** - Market news and sentiment

### Placing Orders

#### Market Orders (Fastest Execution)
1. Enter symbol in quick search
2. Select **Market** order type
3. Enter quantity
4. Choose **Buy** or **Sell**
5. Click **Submit Order**
6. Confirm order in dialog

#### Limit Orders (Price Control)
1. Enter symbol
2. Select **Limit** order type
3. Enter quantity
4. Set limit price
5. Choose **Buy** or **Sell**
6. Submit and confirm

#### Stop Orders (Risk Management)
1. Enter symbol
2. Select **Stop** order type
3. Enter quantity
4. Set stop price
5. Choose **Buy** or **Sell**
6. Submit and confirm

#### Stop-Limit Orders (Advanced)
1. Enter symbol
2. Select **Stop-Limit** order type
3. Enter quantity
4. Set stop price (triggers limit order)
5. Set limit price (execution price)
6. Choose direction
7. Submit and confirm

### Position Management

#### Viewing Positions
- **Position Panel** shows all current holdings
- **Real-time P&L** updates continuously
- **Position Details** include:
  - Entry price and quantity
  - Current market value
  - Unrealized P&L
  - Daily change

#### Managing Positions

**Close Position:**
1. Click position in panel
2. Click **Close Position**
3. Choose quantity (default: all)
4. Submit market order

**Add to Position:**
1. Click position
2. Click **Add Shares**
3. Enter additional quantity
4. Place order as usual

**Reduce Position:**
1. Click position
2. Click **Reduce Shares**
3. Enter quantity to sell
4. Submit sell order

### Trading Notes

#### Adding Notes
1. Click any symbol or position
2. Click **Add Note**
3. Enter your trading notes
4. Add hashtags (e.g., #earnings, #technical)
5. Save note

#### Viewing Notes
- Notes appear in symbol details
- Filter by hashtags
- Search through all notes
- Export notes for analysis

### Risk Management

#### Position Sizing
- **Default**: Risk 1% of portfolio per trade
- **Custom**: Set risk percentage per symbol
- **Fixed**: Set fixed dollar amount per trade

#### Stop Loss Orders
- **Automatic**: Set default stop-loss percentage
- **Manual**: Add stop-loss to any position
- **Trailing**: Use trailing stops for trending markets

---

## Market Analytics

### TrendSetter Module

#### Momentum Scanner
The momentum scanner identifies stocks with strong price and volume momentum:

**Metrics Tracked:**
- **Price Momentum** - 5-day, 20-day returns
- **Volume Spike** - Unusual volume increase
- **Relative Strength** - Performance vs market
- **Social Sentiment** - Twitter/Reddit buzz

**Using Momentum Scanner:**
1. Navigate to **TrendSetter** module
2. Select **Momentum Scanner**
3. Set filters:
   - Market cap range
   - Volume minimum
   - Price range
   - Sector (optional)
4. Click **Scan**
5. Review results with scores

#### Heat Map
Visual representation of market performance:

- **Color Coding**: Green (up), Red (down)
- **Size**: Market cap
- **Intensity**: Price change percentage
- **Grouping**: By sector or industry

**Using Heat Map:**
1. **TrendSetter** → **Heat Map**
2. Select grouping (Sector/Industry)
3. Choose timeframe (1D, 1W, 1M)
4. Click sectors for drill-down
5. Click symbols for details

#### Shift Alerts
Automated notifications for market changes:

**Alert Types:**
- **Volume Spike** - Unusual trading volume
- **Momentum Shift** - Trend direction change
- **Price Breakout** - Key level broken
- **Sentiment Change** - News/social media shift

**Setting Up Alerts:**
1. **TrendSetter** → **Shift Alerts**
2. Click **Create Alert**
3. Select alert type
4. Set parameters
5. Choose notification method
6. Save alert

---

## News & Sentiment

### BullRunnr Module

#### Real-time News Feed
Aggregated news from multiple sources:

**News Sources:**
- Reuters
- Bloomberg
- Associated Press
- MarketWatch
- Yahoo Finance

**News Features:**
- **Real-time updates** - News as it happens
- **Sentiment tagging** - Bullish/Bearish/Neutral
- **Symbol relevance** - News matched to your positions
- **Filter options** - By source, sentiment, importance

#### Sentiment Analysis
NLP-powered sentiment analysis:

**Sentiment Metrics:**
- **Overall Score** - Bullish/Bearish/Neutral
- **Confidence Level** - Accuracy of sentiment
- **Aspect Analysis** - Revenue/earnings/growth sentiment
- **Sector Impact** - Industry-wide sentiment

#### Market Sentiment Dashboard
Overall market fear and greed indicators:

**Market Indicators:**
- **Fear & Greed Index** - Overall market sentiment
- **Sector Sentiment** - Industry-specific analysis
- **Social Buzz** - Social media volume and sentiment
- **Volatility Index** - Market fear indicator

---

## AI Integration

### BearlyManaged Module

#### AI Strategy Generation
Use AI to generate trading strategies:

**Strategy Types:**
- **Momentum** - Trend-following strategies
- **Mean Reversion** - Counter-trend strategies
- **Value** - Fundamental-based strategies
- **Arbitrage** - Market neutral strategies

**Generating Strategies:**
1. **BearlyManaged** → **Strategy Generation**
2. Select strategy type
3. Choose symbols/universe
4. Set timeframe
5. Select risk level
6. Choose AI provider
7. Click **Generate Strategy**
8. Review and save strategy

#### Custom AI Prompts
Create custom analysis prompts:

**Pre-built Templates:**
- **Market Analysis** - Overall market conditions
- **Sector Analysis** - Industry-specific insights
- **Earnings Analysis** - Earnings call insights
- **Technical Analysis** - Chart pattern analysis

**Creating Custom Prompts:**
1. **Prompt Management** → **Add Prompt**
2. Enter prompt template
3. Define variables (e.g., {symbol}, {timeframe})
4. Save template
5. Execute with specific variables

#### AI Usage Tracking
Monitor AI usage and costs:

**Usage Metrics:**
- **Token Usage** - Input/output tokens per request
- **Cost Tracking** - Estimated cost per provider
- **Response Time** - AI provider performance
- **Success Rate** - Failed vs successful requests

---

## Paper Trading

### PaperHands Module

#### Creating Paper Portfolio
Practice trading without risk:

**Portfolio Setup:**
1. **PaperHands** → **Create Portfolio**
2. Enter portfolio name
3. Set initial balance (default: $100,000)
4. Choose commission structure
5. Create portfolio

#### Paper Trading Features
- **Real-time prices** - Live market data
- **Order types** - All supported order types
- **Portfolio tracking** - Performance metrics
- **Risk metrics** - Drawdown, volatility
- **Trade history** - Complete transaction log

#### Strategy Backtesting
Test strategies historically:

**Backtesting Setup:**
1. **PaperHands** → **Backtest**
2. Select strategy or create rules
3. Choose symbols
4. Set date range
5. Configure parameters
6. Run backtest

**Backtesting Results:**
- **Performance Metrics** - Returns, Sharpe ratio
- **Risk Analysis** - Drawdown, volatility
- **Trade Analysis** - Win rate, average loss
- **Benchmark Comparison** - vs market index

#### Performance Analytics
Detailed portfolio analysis:

**Metrics Tracked:**
- **Total Return** - Overall performance
- **Risk-Adjusted Return** - Sharpe, Sortino ratios
- **Maximum Drawdown** - Largest peak-to-trough loss
- **Win Rate** - Percentage of profitable trades
- **Average Win/Loss** - Trade performance

---

## Mobile Apps

### iOS App

#### Installation
1. App Store search "BullShift"
2. Install app (requires iOS 15+)
3. Sign in with existing account
4. Enable biometric authentication

#### iOS Features
- **Face ID/Touch ID** - Secure login
- **Apple Watch** - Quick position view
- **Siri Shortcuts** - Voice commands
- **Widget Support** - Home screen widgets
- **iCloud Sync** - Cross-device sync

### Android App

#### Installation
1. Google Play Store search "BullShift"
2. Install app (requires Android 8.0+)
3. Sign in with existing account
4. Enable biometric authentication

#### Android Features
- **Biometric Login** - Fingerprint/face unlock
- **Android Wear** - Smartwatch support
- **Widget Support** - Home screen widgets
- **Google Assistant** - Voice commands
- **Cloud Sync** - Cross-device sync

### Mobile Trading

#### Quick Order Entry
- **One-tap orders** - Pre-configured sizes
- **Voice commands** - "Buy 10 shares of AAPL"
- **Swipe gestures** - Fast order placement
- **Watch complications** - Quick access

#### Mobile Notifications
- **Price Alerts** - Custom price notifications
- **Order Confirmations** - Real-time order status
- **Portfolio Updates** - P&L changes
- **Market News** - Breaking news alerts

---

## Security

### Credential Security

#### Encryption
- **AES-256-GCM** encryption for all stored data
- **Platform Native Storage** - macOS Keychain, Linux libsecret
- **Secure Memory** - Temporary credentials wiped from memory
- **Zero-Knowledge** - Even we can't access your credentials

#### Best Practices
- **Master Password** - Strong, unique password
- **Two-Factor Authentication** - Enable where available
- **Regular Updates** - Keep app updated
- **Secure Network** - Avoid public WiFi for trading

### Trading Security

#### Order Security
- **Order Confirmation** - All orders require confirmation
- **Position Limits** - Automatic risk controls
- **Time-Based Expiry** - Orders expire after set time
- **Audit Trail** - Complete transaction history

#### Account Security
- **Session Management** - Auto-logout after inactivity
- **Device Recognition** - New device notifications
- **IP Monitoring** - Suspicious login detection
- **Backup Codes** - Account recovery options

---

## Troubleshooting

### Common Issues

#### Connection Problems
**Problem:** "Cannot connect to broker"
- **Solution:** Check internet connection
- **Solution:** Verify API credentials
- **Solution:** Check broker service status

#### Data Not Updating
**Problem:** Prices not updating in real-time
- **Solution:** Refresh data manually
- **Solution:** Check WebSocket connection
- **Solution:** Restart application

#### Order Rejections
**Problem:** Orders being rejected
- **Solution:** Check account balance
- **Solution:** Verify market hours
- **Solution:** Check order parameters

#### Mobile Sync Issues
**Problem:** Desktop and mobile not syncing
- **Solution:** Check cloud sync settings
- **Solution:** Sign out and sign back in
- **Solution:** Check internet connection

### Getting Help

#### Resources
- **[Documentation](.)** - Comprehensive guides
- **[Community Forum](https://github.com/yourrepo/bullshift/discussions)** - Community support

#### Contact Support
- **Email:** support@bullshift.io
- **In-App Help:** Settings → Help & Support
- **Bug Reports:** [GitHub Issues](https://github.com/yourrepo/bullshift/issues)

#### Emergency Issues
For critical trading issues:
- **Urgent Support:** urgent@bullshift.io
- **Security Issues:** security@bullshift.io
- **Live Chat:** Available during market hours

---

## Advanced Features

### Custom Workflows

#### Multi-Timeframe Analysis
- **Chart Overlays** - Multiple timeframes
- **Correlation Analysis** - Symbol relationships
- **Sector Rotation** - Industry performance tracking

#### Automated Strategies
- **Strategy Bots** - Automated execution
- **Risk Management** - Dynamic position sizing
- **Performance Attribution** - Strategy breakdown

### Integration Options

#### Third-Party Tools
- **Spreadsheet Export** - Excel/CSV export
- **API Access** - Developer API (coming soon)
- **Webhook Support** - Custom notifications

#### Brokerage Extensions
- **Additional Brokers** - Interactive Brokers, etc.
- **International Markets** - Global trading access
- **Options Trading** - Options strategies (coming soon)

---

**User Manual Version: 1.0**  
**Last Updated: February 10, 2026**

For the latest updates and features, visit [bullshift.io](https://bullshift.io)
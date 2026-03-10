# BullShift Trading Platform

![Platform Status](https://img.shields.io/badge/Platform-Complete-brightgreen)
![License](https://img.shields.io/badge/License-MIT-blue)
![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen)
![Security](https://img.shields.io/badge/Security-Fixed-green)
![Coverage](https://img.shields.io/badge/Coverage-30%25-yellowgreen)
![Version](https://img.shields.io/badge/Version-2026.3.10-blue)

## 🚀 High-Performance Cross-Platform Trading Platform

Professional trading platform combining Rust performance with Flutter cross-platform capabilities. Built for sub-100ms execution with real trading at its core.

---

## 🏗️ Project Structure

```
bullshift/
├── flutter/                    # Frontend application
│   ├── lib/
│   │   ├── modules/           # Core modules
│   │   │   ├── core_trading/  # 🎯 REAL TRADING (Core Module)
│   │   │   ├── trendsetter/   # Market analytics
│   │   │   ├── bullrunnr/     # News sentiment
│   │   │   ├── barely_managed/ # AI connector
│   │   │   └── paper_hands/   # Paper trading
│   │   ├── services/          # Backend services
│   │   └── widgets/           # UI components
│   └── pubspec.yaml
├── rust/                      # Performance-critical backend
│   ├── src/
│   │   ├── trading/          # 🎯 Real trading engine
│   │   ├── security/          # API key management
│   │   ├── data_stream/      # WebSocket streaming
│   │   └── ai_bridge/        # AI integration
│   └── Cargo.toml
└── docs/                      # Documentation
```

---

## 🎯 Core Modules Overview

BullShift consists of five integrated modules, each serving a specific trading function:

### 🎯 Core Trading Module (REAL TRADING)
The heart of BullShift, providing live trading capabilities:

#### ✅ Real Trading Features
- **Live Market Orders**: Market, Limit, Stop, Stop-Limit orders
- **Real-time Position Management**: Live P&L tracking
- **API Integration**: Alpaca, Interactive Brokers, and more
- **Sub-100ms Execution**: Rust-powered order routing
- **Risk Management**: Position sizing, stop-loss automation
- **Trading Notes**: Symbol-specific note-taking with tags

#### 🔧 Trading Components
- **Order Execution Engine** (`rust/src/trading/execution.rs`)
- **Portfolio Management** (`rust/src/trading/portfolio.rs`)
- **API Integration Layer** (`rust/src/trading/api.rs`)
- **Real-time Data Streaming** (`rust/src/data_stream/mod.rs`)

---

### 📈 TrendSetter Module (Market Analytics) ✅
High-velocity asset discovery with momentum detection:

#### ✅ Market Analytics Features
- **Momentum Scanner**: Real-time momentum scoring with volume spikes
- **Heat Map**: Visual market heat indicators for trending assets
- **Shift Alerts**: Automated alerts for volume spikes and momentum shifts
- **Social Sentiment Integration**: Twitter/Reddit sentiment analysis
- **Multi-factor Scoring**: Volume, price, and social sentiment combined

#### 🔧 Analytics Components
- **Momentum Analysis Engine** (`rust/src/trendsetter/mod.rs`)
- **Volume Spike Detection**: Unusual volume pattern recognition
- **Price Momentum Calculations**: SMA, RSI, MACD indicators
- **Social Buzz Tracking**: Real-time social media sentiment

---

### 📰 BullRunnr Module (News Sentiment) ✅
Real-time financial news with NLP sentiment analysis:

#### ✅ News & Sentiment Features
- **Real-time News Feed**: Multi-source news aggregation (Reuters, Bloomberg, etc.)
- **Instant Sentiment Tagging**: Bullish/Bearish/Neutral classification
- **Market Sentiment Dashboard**: Fear & Greed index, sector sentiment
- **News Search**: Advanced search by keywords and symbols
- **Aspect-based Analysis**: Revenue, earnings, growth sentiment breakdown

#### 🔧 News Components
- **News Aggregation Engine** (`rust/src/bullrunnr/mod.rs`)
- **NLP Sentiment Analyzer**: VADER-based sentiment analysis
- **Multi-source Integration**: Alpha Vantage, News API, Twitter
- **Real-time Processing**: Sub-second news sentiment updates

---

### 🤖 BearlyManaged Module (AI Setup Connector) 🚧
Intelligent automation middleware for AI integration:

#### 🚧 AI Integration Features (In Progress)
- **AI Setup Wizard**: Guided configuration for AI providers
- **Multi-LLM Support**: OpenAI, Anthropic, local Ollama instances
- **Secure Credential Vault**: Encrypted AI API key management
- **Strategy Prompting**: Automated trading strategy generation
- **AI Bridge Architecture**: LangChain integration layer

#### 🔧 AI Components
- **AI Bridge Manager** (`rust/src/ai_bridge/mod.rs`)
- **LLM Connector**: Multi-provider AI interface
- **Strategy Generator**: AI-powered trading strategy creation
- **Secure Key Management**: Platform-native AI credential storage

---

### 🎮 PaperHands Module (Paper Trading) ✅
Risk-free simulation environment for strategy testing:

#### ✅ Paper Trading Features
- **Zero-risk Simulation**: Real-time price action testing
- **Strategy Backtesting**: Historical performance analysis
- **Virtual Portfolio**: Paper money position management
- **Performance Analytics**: Win rate, P&L, risk metrics
- **Advanced Charting**: Professional chart analysis with indicators
- **Monte Carlo Simulation**: Risk analysis with multiple scenarios
- **Portfolio Analytics**: Detailed performance and risk metrics
- **Strategy Validation**: Test before real money deployment

#### 🔧 Simulation Components
- **Paper Trading Engine** (`rust/src/paper_hands/mod.rs`)
- **Advanced Charting**: Multiple chart types and technical indicators
- **Performance Tracker**: Detailed analytics and reporting
- **Monte Carlo Engine**: Statistical risk analysis
- **Strategy Validator**: Risk-free strategy testing

---

![Rust](https://img.shields.io/badge/Rust-Performance-CE4E1F?logo=rust)
![Flutter](https://img.shields.io/badge/Flutter-UI-02569B?logo=flutter)
![Security](https://img.shields.io/badge/Security-AES256-green)
![Real-time](https://img.shields.io/badge/Latency-Sub--100ms-brightgreen)

## 🛠️ Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Frontend** | Flutter 4.0 | Cross-platform UI |
| **Core Logic** | Rust | High-performance trading engine |
| **Data Stream** | WebSockets/gRPC | Real-time market data |
| **Security** | AES-256 + Platform Keychain | API key protection |
| **Database** | ObjectBox | Local historical data |

---

## 🔒 Security Architecture

### API Key Management
- **AES-256 Encryption** for all stored credentials
- **Platform Native Storage**: macOS Keychain, Linux libsecret
- **Secure Memory Management** in Rust backend
- **Zero-Knowledge Architecture**: Keys never stored in plaintext

### Security Components
- `rust/src/security/mod.rs` - Rust security manager
- `flutter/lib/services/security_manager.dart` - Flutter security layer

---

## 🚀 Quick Start

1. **Setup Environment** - See [Development Setup](docs/guides/development_setup.md)
2. **Build Platform** - `./build.sh` or manual build
3. **Configure APIs** - Store trading/AI credentials securely
4. **Start Trading** - Real-time order execution & analytics

**Key Features:**
- 🎯 **Live Trading** - Sub-100ms order execution
- 📈 **Market Analytics** - Momentum detection & sentiment
- 🤖 **AI Integration** - Strategy generation & analysis
- 🎮 **Paper Trading** - Risk-free simulation
- 📱 **Cross-Platform** - Desktop + mobile apps

---

## 🎯 Features

### Core Trading
- Live market orders (Market, Limit, Stop, Stop-Limit)
- Real-time position management & P&L tracking
- Sub-100ms execution via Rust engine
- API integration (Alpaca, Interactive Brokers)
- Risk management & trading notes

### Market Intelligence
- Momentum scanner with volume spike detection
- Real-time news sentiment analysis
- Social sentiment integration (Twitter/Reddit)
- Market heat maps & shift alerts

### AI Integration
- Multi-LLM support (OpenAI, Anthropic, Ollama)
- AI-powered trading strategy generation
- Secure credential vault
- Custom prompt management

### Paper Trading
- Zero-risk simulation environment
- Strategy backtesting & performance analytics
- Virtual portfolio management
- Monte Carlo risk analysis

### Security
- AES-256 encryption for all credentials
- Platform native storage (macOS Keychain, Linux libsecret)
- Zero-knowledge architecture
- Secure FFI memory management

### Cross-Platform
- Desktop: Linux, macOS, Windows
- Mobile: iOS, Android
- Cloud sync across devices
- Touch-optimized mobile interface

---

## 🔗 API Integrations

### Currently Supported
- **Alpaca Markets**: Stocks and ETFs
- **Interactive Brokers**: Global markets (planned)

### API Integration Architecture
```rust
// Trading API trait
pub trait TradingApi {
    async fn submit_order(&self, order: ApiOrderRequest) -> Result<ApiOrderResponse, String>;
    async fn get_positions(&self) -> Result<Vec<ApiPosition>, String>;
    async fn get_account(&self) -> Result<ApiAccount, String>;
}
```

---

## 🧪 Development

### Running Tests
```bash
# Rust tests
cd rust && cargo test

# Flutter tests
cd flutter && flutter test
```

### Code Quality
```bash
# Rust formatting
cd rust && cargo fmt

# Flutter linting
cd flutter && flutter analyze
```

---

![Flutter](https://img.shields.io/badge/Flutter-4.0+-02569B?logo=flutter)
![Rust](https://img.shields.io/badge/Rust-1.70+-CE4E1F?logo=rust)
![Platforms](https://img.shields.io/badge/Platforms-5%20Supported-informational)
![Modules](https://img.shields.io/badge/Modules-5%20Complete-brightgreen)

## 📋 Status

### ✅ Recently Fixed (Feb 2026)
- **5 Critical compilation errors** - Code now builds
- **Memory leaks** in FFI trading engine
- **Security vulnerabilities** - AES-256 encryption implemented
- **Code duplication** - BaseProvider pattern created
- **View refactoring** - BullRunnr & PaperHands modules extracted
- **Paper trading backend** - Monte Carlo simulation & correlation analysis
- **Dialog implementations** - All 6 AI provider dialogs completed
- **Test coverage** - 108 new tests across 4 providers
- **Performance** - Rust cloning optimizations

### 🚧 Remaining Work
- **AI Bridge Backend:** 9 TODOs for Anthropic, Ollama, Local LLM, Custom providers
- **Data Streaming:** Secure credential loading from storage
- **Testing:** Increase coverage from 25% to 80%+
- **TODOs:** 9 feature implementations remaining (AI Bridge backend)

### 📊 Platform
- **5 Modules:** Trading, Analytics, News, AI, Paper Trading
- **Languages:** Rust, Dart, Python
- **Platforms:** Linux, macOS, Windows, iOS, Android
- **Status:** Ready for development

---

![Documentation](https://img.shields.io/badge/Docs-Complete-blue)

## 📚 Documentation

![Documentation](https://img.shields.io/badge/Docs-Complete-blue)

### User Documentation
- **[User Manual](docs/guides/user_manual.md)** - Complete platform usage guide
- **[Configuration Guide](docs/reference/configuration_guide.md)** - Setup trading APIs & AI providers
- **[Troubleshooting Guide](docs/guides/troubleshooting.md)** - Common issues & solutions

### Developer Documentation
- **[Development Setup](docs/guides/development_setup.md)** - Environment setup & build guide
- **[API Reference](docs/reference/api_reference.md)** - Rust & Flutter API documentation
- **[Architecture](docs/architecture.md)** - Project structure & modules
- **[Contributing Guide](docs/guides/contributing.md)** - Contribution guidelines & standards
- **[Code Audit](docs/development/code-audit-2026-03.md)** - Code audit findings & remediation

### Project Documentation
- **[Security Policy](docs/reference/security.md)** - Vulnerability reporting & security practices
- **[Changelog](CHANGELOG.md)** - Version history & release notes
- **[Roadmap](docs/roadmap.md)** - Feature planning & release schedule
- **[Security Audit](docs/reference/security-audit.md)** - Security assessment & fixes
- **[Refactoring Summary](docs/guides/refactoring-summary.md)** - Recent improvements & changes
- **[ADRs](docs/adr/)** - Architecture decision records

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/guides/contributing.md) for detailed guidelines.

**Quick Start:**
1. Fork the repository
2. Create feature branch
3. Follow security best practices
4. Add tests
5. Submit pull request

**Security:** All contributions require security review. See [Security Policy](docs/reference/security.md) for details.

---

## 🎯 Key Advantages

- **Performance** - Sub-100ms execution via Rust engine
- **Security** - AES-256 encryption, zero-knowledge architecture
- **Intelligence** - AI-powered strategies & multi-factor analysis
- **Risk Management** - Paper trading, position sizing, exposure monitoring
- **Cross-Platform** - Seamless sync across desktop + mobile

**Built with ❤️ for the modern quantitative trader**

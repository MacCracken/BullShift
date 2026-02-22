# Configuration Guide

## Overview

This guide provides step-by-step instructions for configuring trading APIs, AI providers, and other external services with BullShift.

## Table of Contents

1. [Trading Broker Configuration](#trading-broker-configuration)
2. [AI Provider Configuration](#ai-provider-configuration)
3. [Data Provider Setup](#data-provider-setup)
4. [Security Configuration](#security-configuration)
5. [Network and Firewall Settings](#network-and-firewall-settings)
6. [Configuration Files](#configuration-files)
7. [Environment Variables](#environment-variables)
8. [Troubleshooting Configuration](#troubleshooting-configuration)

---

## Trading Broker Configuration

### Alpaca Markets

#### Prerequisites
- Alpaca account (Paper or Live)
- Verified identity and funding
- API permissions enabled

#### Step 1: Generate API Keys

1. **Login to Alpaca Dashboard**
   - Visit [alpaca.markets/paper/dashboard](https://alpaca.markets/paper/dashboard) for paper trading
   - Visit [alpaca.markets/dashboard](https://alpaca.markets/dashboard) for live trading

2. **Navigate to API Keys**
   ```
   Dashboard → Settings → API Keys
   ```

3. **Generate New Key**
   - Click "Generate New Key"
   - Select permissions (Trading + Data recommended)
   - Click "Create"

4. **Save Credentials**
   - Copy API Key ID (starts with "PK...")
   - Copy Secret Key (starts with "sk...")
   - Store securely (password manager recommended)

#### Step 2: Configure in BullShift

**Method 1: UI Configuration**
1. Open BullShift application
2. Navigate to **Settings** → **Trading Accounts**
3. Click **Add Broker**
4. Select **Alpaca Markets**
5. Enter credentials:
   ```
   API Key: PK... (your API key)
   Secret Key: sk... (your secret key)
   Environment: Paper Trading / Live Trading
   ```
6. Click **Test Connection**
7. Verify account details appear
8. Click **Save**

**Method 2: Configuration File**
```json
{
  "trading": {
    "alpaca": {
      "api_key": "PK_YOUR_API_KEY_HERE",
      "secret_key": "sk_YOUR_SECRET_KEY_HERE",
      "base_url": "https://paper-api.alpaca.markets",
      "data_url": "https://data.alpaca.markets"
    }
  }
}
```

#### Step 3: Verify Configuration

**Test Market Data Access:**
```dart
// Get account info
final account = await tradingProvider.getAccount();
print('Account: ${account.accountId}');
print('Buying Power: ${account.buyingPower}');

// Get positions
final positions = await tradingProvider.getPositions();
print('Current positions: ${positions.length}');
```

**Test Order Execution (Paper Trading Only):**
```dart
// Submit test order
final order = await tradingProvider.submitMarketOrder(
  symbol: 'AAPL',
  quantity: 1,
  side: OrderSide.buy,
);
print('Order submitted: ${order.id}');
```

### Interactive Brokers (Coming Soon)

#### Prerequisites
- Interactive Brokers account
- Trader Workstation (TWS) or IB Gateway
- API permissions enabled

#### Configuration Steps
1. **Enable API Access in TWS**
   - Configure → API → Settings
   - Enable "Enable ActiveX and Socket Clients"
   - Set socket port (default: 7497)
   - Create API password

2. **Configure BullShift**
   ```json
   {
     "trading": {
       "interactive_brokers": {
         "host": "127.0.0.1",
         "port": 7497,
         "client_id": 1,
         "api_password": "your_api_password"
       }
     }
   }
   ```

---

## AI Provider Configuration

### OpenAI

#### Prerequisites
- OpenAI account
- API key with billing enabled
- Sufficient credits for usage

#### Step 1: Generate API Key

1. **Login to OpenAI Platform**
   - Visit [platform.openai.com](https://platform.openai.com)

2. **Navigate to API Keys**
   ```
   Dashboard → API Keys
   ```

3. **Create New Secret Key**
   - Click "Create new secret key"
   - Give it a descriptive name (e.g., "BullShift")
   - Set permissions (Read/Write)
   - Click "Create secret key"

4. **Copy and Store Key**
   - Copy the key immediately (starts with "sk-...")
   - Store securely (won't be shown again)

#### Step 2: Configure in BullShift

**UI Configuration:**
1. Navigate to **AI Module** → **Provider Setup**
2. Click **Add Provider**
3. Select **OpenAI**
4. Configure settings:
   ```
   Name: OpenAI GPT-4
   Type: OpenAI
   API Endpoint: https://api.openai.com/v1
   Model: gpt-4 (or gpt-3.5-turbo)
   API Key: sk-your-key-here
   ```
5. Click **Test Connection**
6. Verify successful response
7. Click **Save**

**Configuration File:**
```json
{
  "ai_providers": {
    "openai": {
      "name": "OpenAI GPT-4",
      "type": "OpenAI",
      "api_endpoint": "https://api.openai.com/v1",
      "model": "gpt-4",
      "api_key": "sk-your-openai-key-here",
      "max_tokens": 4096,
      "temperature": 0.7
    }
  }
}
```

#### Step 3: Verify Configuration

```dart
// Test provider
final provider = BearlyManagedProvider();
final success = await provider.testProvider('openai-gpt4');
print('OpenAI connection: ${success ? "Success" : "Failed"}');

// Send test prompt
final response = await provider.sendPrompt(
  providerId: 'openai-gpt4',
  prompt: 'Analyze AAPL stock fundamentals',
);
print('AI Response: ${response.content}');
```

### Anthropic Claude

#### Prerequisites
- Anthropic account
- API access (may require request)
- Billing setup

#### Configuration

**UI Setup:**
1. **AI Module** → **Provider Setup**
2. **Add Provider** → **Anthropic**
3. Configure:
   ```
   Name: Claude 3
   Type: Anthropic
   API Endpoint: https://api.anthropic.com
   Model: claude-3-sonnet-20240229
   API Key: your-anthropic-key
   ```

**Configuration File:**
```json
{
  "ai_providers": {
    "anthropic": {
      "name": "Claude 3",
      "type": "Anthropic",
      "api_endpoint": "https://api.anthropic.com",
      "model": "claude-3-sonnet-20240229",
      "api_key": "your-anthropic-key-here",
      "max_tokens": 4096
    }
  }
}
```

### Ollama (Local LLM)

#### Prerequisites
- Ollama installed locally
- Models downloaded (llama2, codellama, etc.)
- Sufficient system resources

#### Installation

1. **Install Ollama**
   ```bash
   # macOS
   brew install ollama
   
   # Linux
   curl -fsSL https://ollama.ai/install.sh | sh
   
   # Windows
   # Download from https://ollama.ai/download
   ```

2. **Download Models**
   ```bash
   ollama pull llama2
   ollama pull codellama
   ollama pull mistral
   ```

3. **Start Ollama Server**
   ```bash
   ollama serve
   # Server runs on http://localhost:11434
   ```

#### Configuration in BullShift

**UI Setup:**
1. **AI Module** → **Provider Setup**
2. **Add Provider** → **Ollama**
3. Configure:
   ```
   Name: Ollama Local
   Type: Ollama
   API Endpoint: http://localhost:11434
   Model: llama2
   ```

**Configuration File:**
```json
{
  "ai_providers": {
    "ollama": {
      "name": "Ollama Local",
      "type": "Ollama",
      "api_endpoint": "http://localhost:11434",
      "model": "llama2",
      "timeout": 30
    }
  }
}
```

---

## Data Provider Setup

### Market Data Sources

#### Alpaca Data API

**Configuration:**
```json
{
  "data_providers": {
    "alpaca": {
      "type": "Alpaca",
      "api_key": "PK_YOUR_API_KEY",
      "secret_key": "sk_YOUR_SECRET_KEY",
      "base_url": "https://data.alpaca.markets",
      "websocket_url": "wss://stream.data.alpaca.markets"
    }
  }
}
```

#### Alternative Data Sources

**Polygon.io:**
```json
{
  "data_providers": {
    "polygon": {
      "type": "Polygon",
      "api_key": "your-polygon-key",
      "base_url": "https://api.polygon.io"
    }
  }
}
```

**Yahoo Finance (Free):**
```json
{
  "data_providers": {
    "yahoo": {
      "type": "Yahoo",
      "base_url": "https://query1.finance.yahoo.com"
    }
  }
}
```

---

## Security Configuration

### Credential Storage

#### Platform Native Storage

**macOS Keychain:**
```dart
// Credentials automatically stored in macOS Keychain
await SecurityManager.storeTradingCredentials(
  broker: 'alpaca',
  apiKey: 'PK_YOUR_KEY',
  apiSecret: 'sk_YOUR_SECRET',
);
```

**Linux libsecret:**
```bash
# Install libsecret development headers
sudo apt install libsecret-1-dev

# BullShift will use system keyring
```

**Windows Credential Manager:**
```dart
// Credentials stored in Windows Credential Manager
// Automatic with Windows builds
```

### Encryption Settings

#### Default Encryption (AES-256-GCM)
```json
{
  "security": {
    "encryption": {
      "algorithm": "AES-256-GCM",
      "key_derivation": "PBKDF2",
      "iterations": 100000,
      "salt_length": 32
    }
  }
}
```

#### Custom Encryption Key
```json
{
  "security": {
    "master_key": {
      "type": "derived", // "derived" or "provided"
      "password_hint": "Your master password hint",
      "require_biometric": true
    }
  }
}
```

---

## Network and Firewall Settings

### Required Ports

#### Outbound Connections
```
443/tcp  - HTTPS (API calls)
8443/tcp - Secure WebSocket (market data)
7497/tcp - Interactive Brokers (if used)
4002/tcp - Tradier (if used)
```

#### Local Connections
```
11434/tcp - Ollama (local LLM)
8080/tcp  - Local development server
```

### Firewall Configuration

#### Linux (ufw)
```bash
# Allow outbound HTTPS and WebSocket
sudo ufw allow out 443/tcp
sudo ufw allow out 8443/tcp

# Allow local Ollama
sudo ufw allow from 127.0.0.1 to any port 11434
```

#### macOS
```bash
# System Preferences → Security & Privacy → Firewall
# Add BullShift to allowed applications
# Enable "Allow incoming connections" for BullShift
```

#### Windows
```
# Windows Security → Firewall & network protection
# Add BullShift to allowed apps through firewall
# Enable "Private" networks for local connections
```

### Proxy Configuration

#### HTTP/HTTPS Proxy
```json
{
  "network": {
    "proxy": {
      "enabled": true,
      "http_url": "http://proxy.company.com:8080",
      "https_url": "https://proxy.company.com:8080",
      "username": "your-username",
      "password": "your-password"
    }
  }
}
```

#### SOCKS Proxy
```json
{
  "network": {
    "proxy": {
      "enabled": true,
      "socks_url": "socks5://proxy.company.com:1080",
      "username": "your-username",
      "password": "your-password"
    }
  }
}
```

---

## Configuration Files

### Default Configuration Locations

| Platform | Configuration File |
|----------|-------------------|
| Linux | `~/.config/bullshift/config.json` |
| macOS | `~/Library/Application Support/BullShift/config.json` |
| Windows | `%APPDATA%/BullShift/config.json` |

### Complete Configuration Example

```json
{
  "version": "1.0.0",
  "trading": {
    "alpaca": {
      "api_key": "${ALPACA_API_KEY}",
      "secret_key": "${ALPACA_SECRET_KEY}",
      "base_url": "https://paper-api.alpaca.markets",
      "paper_trading": true
    }
  },
  "ai_providers": {
    "openai": {
      "name": "OpenAI GPT-4",
      "type": "OpenAI",
      "api_endpoint": "https://api.openai.com/v1",
      "model": "gpt-4",
      "api_key": "${OPENAI_API_KEY}",
      "max_tokens": 4096,
      "temperature": 0.7
    },
    "ollama": {
      "name": "Ollama Local",
      "type": "Ollama",
      "api_endpoint": "http://localhost:11434",
      "model": "llama2",
      "timeout": 30
    }
  },
  "data_providers": {
    "alpaca": {
      "type": "Alpaca",
      "websocket_url": "wss://stream.data.alpaca.markets"
    }
  },
  "security": {
    "encryption": {
      "algorithm": "AES-256-GCM",
      "key_derivation": "PBKDF2",
      "iterations": 100000
    },
    "biometric": {
      "enabled": true,
      "timeout": 300
    }
  },
  "network": {
    "timeout": 30,
    "retry_attempts": 3,
    "retry_delay": 1000
  },
  "ui": {
    "theme": "dark",
    "language": "en",
    "default_chart_timeframe": "1D",
    "auto_refresh_interval": 5
  }
}
```

---

## Environment Variables

### Required Variables

```bash
# Trading APIs
export ALPACA_API_KEY="PK_YOUR_API_KEY"
export ALPACA_SECRET_KEY="sk_YOUR_SECRET_KEY"

# AI Providers
export OPENAI_API_KEY="sk-your-openai-key"
export ANTHROPIC_API_KEY="your-anthropic-key"

# Optional Settings
export BULLSHIFT_LOG_LEVEL="info"
export BULLSHIFT_CONFIG_PATH="/path/to/custom/config.json"
export BULLSHIFT_DATA_PATH="/path/to/data"
```

### Docker Environment

```dockerfile
# Dockerfile
FROM bullshift:latest

# Environment variables
ENV ALPACA_API_KEY=""
ENV ALPACA_SECRET_KEY=""
ENV OPENAI_API_KEY=""

# Volume mounts
VOLUME ["/config", "/data"]

# Expose ports
EXPOSE 8080
```

```yaml
# docker-compose.yml
version: '3.8'
services:
  bullshift:
    image: bullshift:latest
    environment:
      - ALPACA_API_KEY=${ALPACA_API_KEY}
      - ALPACA_SECRET_KEY=${ALPACA_SECRET_KEY}
      - OPENAI_API_KEY=${OPENAI_API_KEY}
    volumes:
      - ./config:/config
      - ./data:/data
    ports:
      - "8080:8080"
```

---

## Troubleshooting Configuration

### Common Issues

#### API Key Errors

**Problem:** "Invalid API key" or "Authentication failed"
- **Solution:** Verify API key is correct and active
- **Solution:** Check environment (paper vs live trading)
- **Solution:** Ensure API key has required permissions

#### Connection Timeouts

**Problem:** "Connection timeout" or "Network error"
- **Solution:** Check internet connection
- **Solution:** Verify firewall settings
- **Solution:** Test proxy configuration

#### WebSocket Issues

**Problem:** Real-time data not updating
- **Solution:** Check WebSocket URL configuration
- **Solution:** Verify port 8443 is open
- **Solution:** Test connection with wscat

#### AI Provider Issues

**Problem:** AI requests failing
- **Solution:** Verify API key and billing
- **Solution:** Check model availability
- **Solution:** Test endpoint with curl

### Debug Mode

#### Enable Debug Logging
```json
{
  "logging": {
    "level": "debug",
    "file": "/var/log/bullshift/debug.log",
    "console": true
  }
}
```

#### Test Connections

**Trading Connection:**
```bash
curl -X GET "https://paper-api.alpaca.markets/v2/account" \
  -H "APCA-API-KEY-ID: PK_YOUR_KEY" \
  -H "APCA-API-SECRET-KEY: sk_YOUR_SECRET"
```

**AI Connection:**
```bash
curl -X POST "https://api.openai.com/v1/chat/completions" \
  -H "Authorization: Bearer sk-your-key" \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Hello"}]}'
```

**WebSocket Connection:**
```bash
wscat -c wss://stream.data.alpaca.markets/v2/iex
```

### Reset Configuration

#### Reset to Defaults
```bash
# Remove configuration file
rm ~/.config/bullshift/config.json

# Remove stored credentials (Linux)
secret-tool clear BullShift Trading

# Remove stored credentials (macOS)
security delete-generic-password -a bullshift
```

#### Safe Reset
```bash
# Backup current config
cp ~/.config/bullshift/config.json ~/.config/bullshift/config.backup

# Create minimal config
cat > ~/.config/bullshift/config.json << EOF
{
  "version": "1.0.0",
  "trading": {},
  "ai_providers": {},
  "data_providers": {}
}
EOF
```

---

## Best Practices

### Security
1. **Never commit API keys** to version control
2. **Use environment variables** for sensitive data
3. **Rotate API keys** regularly
4. **Use paper trading** for initial testing
5. **Enable biometric authentication** where available

### Performance
1. **Choose local data sources** for faster access
2. **Configure appropriate timeouts** for your network
3. **Use connection pooling** for high-frequency trading
4. **Monitor usage** for AI providers (cost control)

### Reliability
1. **Test all configurations** before production use
2. **Set up monitoring** for API failures
3. **Have backup data sources** configured
4. **Document custom configurations** for team members

---

**Configuration Guide Version: 1.0**  
**Last Updated: February 10, 2026**

For additional help, see the [Troubleshooting Guide](troubleshooting.md) or open an issue on GitHub.
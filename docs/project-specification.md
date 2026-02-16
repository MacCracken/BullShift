# MAIN.md - BullShift Project Specification

## 🚀 Project Vision
**BullShift** is a high-performance, cross-platform trading ecosystem designed for the modern investor. It facilitates a seamless transition from market research to AI-assisted strategy execution. The application prioritizes Linux and MacOS environments for its initial launch, with a focus on low-latency data and secure AI integration.

---

## 🛠️ Core Modules

### 📈 TrendSetter (Market Analytics)
* **Goal:** High-velocity asset discovery.
* **Functionality:** A proprietary labeling system that identifies stocks gaining momentum based on real-time volume spikes and social sentiment.
* **UI Element:** Visual "heat" indicators and "Shift" alerts for assets entering a high-trend state.

### 📰 BullRunnr (News Section)
* **Goal:** Real-time information edge.
* **Functionality:** A high-speed financial news feed utilizing Natural Language Processing (NLP).
* **Key Feature:** Instant sentiment tagging (Bullish/Bearish/Neutral) on global headlines to inform quick decision-making.

### 🤖 BearlyManaged (AI Setup Connector)
* **Goal:** Intelligent automation.
* **Functionality:** A middleware "wizard" that bridges user API keys to Large Language Models (LLMs) like OpenAI, Anthropic, or local instances via Ollama.
* **Key Feature:** Secure vaulting for encrypted credentials and automated strategy prompting.

### 🎮 PaperHands (Paper Trading)
* **Goal:** Risk-free execution.
* **Functionality:** A zero-risk simulation environment using real-time price action.
* **Key Feature:** Allows users to live-test strategies derived from "TrendSetter" and "BullRunnr" before committing real capital.

---

## 💻 Technical Stack (2026)

| Layer | Technology | Rationale |
| :--- | :--- | :--- |
| **Frontend** | **Flutter 4.0** | Native UI consistency across Linux, MacOS, and Mobile. |
| **Logic/Speed** | **Rust (Tauri/FFI)** | Memory safety and raw speed for processing price feeds. |
| **Data Stream** | **gRPC / WebSockets** | Real-time, sub-100ms market data delivery. |
| **Database** | **ObjectBox** | Ultra-fast local NoSQL storage for historical data. |
| **AI Bridge** | **LangChain / Ollama** | Flexible architecture for local or cloud-based AI. |

---

## 📱 Platform Availability & Roadmap

### Phase 1: Unix-Based Launch (Current)
* **Desktop:** MacOS (Apple Silicon & Intel), Linux (Flatpak/AppImage for Ubuntu, Fedora, Arch).
* **Mobile:** iOS and Android.

### Phase 2: Windows Expansion
* **Desktop:** Windows (Win32/DirectX optimization) using the existing abstracted logic layer.

---

## 🔒 Security & Constraints
> [!IMPORTANT]
> **Data Privacy:** All API keys managed via **BearlyManaged** must be encrypted using **AES-256**.
> **System Integrity:** The application must utilize platform-native secure enclaves (MacOS Keychain / Linux libsecret).
> **Performance:** The **PaperHands** simulation engine must maintain sub-100ms latency for price updates.

---

## 📅 Initial Development Roadmap

1.  **Foundations:** Establish the Flutter-Rust bridge and WebSocket data ingestion.
2.  **Simulation:** Deploy the `PaperHands` sandbox with real-time data streaming.
3.  **Intelligence:** Integrate `BullRunnr` sentiment analysis and the `BearlyManaged` AI connection wizard.
4.  **Optimization:** Refine `TrendSetter` screening algorithms and finalize cross-platform distribution builds.

---
*Created for the BullShift Development Team - 2026*

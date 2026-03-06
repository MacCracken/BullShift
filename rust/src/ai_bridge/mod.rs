use crate::error::BullShiftError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use reqwest::Client;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProvider {
    pub id: Uuid,
    pub name: String,
    pub provider_type: AIProviderType,
    pub api_endpoint: String,
    pub api_key: String,
    pub model_name: String,
    pub is_configured: bool,
    pub is_active: bool,
    pub max_tokens: u32,
    pub temperature: f64,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProviderType {
    OpenAI,
    Anthropic,
    Ollama,
    LocalLLM,
    SecureYeoman,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfiguration {
    pub provider_id: Uuid,
    pub api_key: String, // Encrypted
    pub organization_id: Option<String>, // Encrypted
    pub custom_headers: HashMap<String, String>, // Encrypted
    pub rate_limit: RateLimit,
    pub cost_tracking: CostTracking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub tokens_per_minute: u32,
    pub current_usage: CurrentUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUsage {
    pub requests_this_minute: u32,
    pub tokens_this_minute: u32,
    pub reset_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostTracking {
    pub cost_per_1k_tokens: f64,
    pub total_tokens_used: u64,
    pub total_cost: f64,
    pub daily_limit: f64,
    pub daily_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingStrategy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub provider_id: Uuid,
    pub strategy_type: StrategyType,
    pub parameters: StrategyParameters,
    pub prompt_template: String,
    pub is_active: bool,
    pub performance: StrategyPerformance,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyType {
    Momentum,
    MeanReversion,
    Breakout,
    SentimentBased,
    Arbitrage,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyParameters {
    pub symbols: Vec<String>,
    pub timeframe: String,
    pub risk_level: RiskLevel,
    pub position_sizing: PositionSizing,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub max_positions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Conservative,
    Moderate,
    Aggressive,
    VeryAggressive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionSizing {
    Fixed,
    Percentage,
    VolatilityBased,
    KellyCriterion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyPerformance {
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
    pub win_rate: f64,
    pub total_return: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub avg_trade_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPrompt {
    pub id: Uuid,
    pub name: String,
    pub category: PromptCategory,
    pub template: String,
    pub variables: Vec<String>,
    pub provider_id: Uuid,
    pub is_system_prompt: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptCategory {
    MarketAnalysis,
    StrategyGeneration,
    RiskAssessment,
    SentimentAnalysis,
    TechnicalAnalysis,
    NewsInterpretation,
    PortfolioOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub prompt_id: Uuid,
    pub response: String,
    pub tokens_used: u32,
    pub cost: f64,
    pub response_time_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

pub struct BearlyManaged {
    client: Client,
    providers: HashMap<Uuid, AIProvider>,
    configurations: HashMap<Uuid, AIConfiguration>,
    strategies: HashMap<Uuid, TradingStrategy>,
    prompts: HashMap<Uuid, AIPrompt>,
    responses: std::collections::VecDeque<AIResponse>,
    security_manager: crate::security::SecurityManager,
}

impl BearlyManaged {
    pub fn new(security_manager: crate::security::SecurityManager) -> Self {
        Self {
            client: Client::new(),
            providers: HashMap::new(),
            configurations: HashMap::new(),
            strategies: HashMap::new(),
            prompts: HashMap::new(),
            responses: std::collections::VecDeque::with_capacity(1000),
            security_manager,
        }
    }

    // Provider Management
    pub async fn add_provider(&mut self, mut provider: AIProvider) -> Result<Uuid, BullShiftError> {
        let provider_id = provider.id;

        // Validate provider configuration
        self.validate_provider(&provider)?;

        // Encrypt and store the API key via SecurityManager
        if !provider.api_key.is_empty() {
            self.security_manager.store_api_key(&provider.name, &provider.api_key)?;
            // Clear plaintext key from the in-memory provider struct
            provider.api_key = String::new();
        }

        // Store provider
        self.providers.insert(provider_id, provider);

        log::info!("Added AI provider: {:?}", provider_id);
        Ok(provider_id)
    }

    pub async fn configure_provider(&mut self, provider_id: Uuid, config: AIConfiguration) -> Result<(), BullShiftError> {
        // Validate provider exists
        if !self.providers.contains_key(&provider_id) {
            return Err(BullShiftError::AiBridge("Provider not found".to_string()));
        }

        // Encrypt sensitive data
        let encrypted_config = self.encrypt_configuration(&config)?;
        
        // Store configuration
        self.configurations.insert(provider_id, encrypted_config);
        
        // Mark provider as configured
        if let Some(provider) = self.providers.get_mut(&provider_id) {
            provider.is_configured = true;
        }
        
        log::info!("Configured AI provider: {:?}", provider_id);
        Ok(())
    }

    pub async fn test_provider_connection(&self, provider_id: Uuid) -> Result<bool, BullShiftError> {
        let provider = self.providers.get(&provider_id)
            .ok_or_else(|| BullShiftError::AiBridge("Provider not found".to_string()))?;
        
        let config = self.configurations.get(&provider_id)
            .ok_or_else(|| BullShiftError::AiBridge("Provider not configured".to_string()))?;

        match provider.provider_type {
            AIProviderType::OpenAI => {
                self.test_openai_connection(provider, config).await
            }
            AIProviderType::Anthropic => {
                self.test_anthropic_connection(provider, config).await
            }
            AIProviderType::Ollama => {
                self.test_ollama_connection(provider, config).await
            }
            AIProviderType::LocalLLM => {
                self.test_local_llm_connection(provider, config).await
            }
            AIProviderType::SecureYeoman => {
                self.test_secureyeoman_connection(provider, config).await
            }
            AIProviderType::Custom => {
                self.test_custom_connection(provider, config).await
            }
        }
    }

    // Strategy Management
    pub async fn create_strategy(&mut self, strategy: TradingStrategy) -> Result<Uuid, BullShiftError> {
        let strategy_id = strategy.id;
        
        // Validate strategy
        self.validate_strategy(&strategy)?;
        
        // Generate AI-enhanced strategy if needed
        let enhanced_strategy = if strategy.prompt_template.contains("{{ai_generated}}") {
            self.generate_ai_strategy(&strategy).await?
        } else {
            strategy
        };
        
        // Store strategy
        self.strategies.insert(strategy_id, enhanced_strategy);
        
        log::info!("Created trading strategy: {:?}", strategy_id);
        Ok(strategy_id)
    }

    pub async fn generate_ai_strategy(&self, strategy: &TradingStrategy) -> Result<TradingStrategy, BullShiftError> {
        let provider = self.providers.get(&strategy.provider_id)
            .ok_or_else(|| BullShiftError::AiBridge("Provider not found".to_string()))?;
        
        let prompt = self.build_strategy_generation_prompt(strategy);
        
        let response = self.send_ai_request(provider, &prompt).await?;
        
        if !response.success {
            return Err(BullShiftError::AiBridge(response.error_message.unwrap_or("AI request failed".to_string())));
        }
        
        // Parse AI response and update strategy
        let mut enhanced_strategy = strategy.clone();
        enhanced_strategy.description = response.response;
        enhanced_strategy.last_updated = Utc::now();
        
        Ok(enhanced_strategy)
    }

    // Prompt Management
    pub async fn add_prompt(&mut self, prompt: AIPrompt) -> Result<Uuid, BullShiftError> {
        let prompt_id = prompt.id;
        
        // Validate prompt
        self.validate_prompt(&prompt)?;
        
        // Store prompt
        self.prompts.insert(prompt_id, prompt);
        
        log::info!("Added AI prompt: {:?}", prompt_id);
        Ok(prompt_id)
    }

    pub async fn execute_prompt(&mut self, prompt_id: Uuid, variables: HashMap<String, String>) -> Result<AIResponse, BullShiftError> {
        let prompt = self.prompts.get(&prompt_id)
            .ok_or_else(|| BullShiftError::AiBridge("Prompt not found".to_string()))?;
        
        let provider = self.providers.get(&prompt.provider_id)
            .ok_or_else(|| BullShiftError::AiBridge("Provider not found".to_string()))?;
        
        // Build final prompt with variables
        let final_prompt = self.substitute_prompt_variables(&prompt.template, &variables);
        
        // Send AI request
        let response = self.send_ai_request(provider, &final_prompt).await?;
        
        // Store response (bounded to last 1000)
        if self.responses.len() >= 1000 {
            self.responses.pop_front();
        }
        self.responses.push_back(response.clone());
        
        Ok(response)
    }

    /// Resolve the decrypted API key for a provider.
    /// If the provider has a key stored in SecurityManager, decrypt and return it.
    /// Otherwise fall back to the (possibly empty) in-memory key.
    fn resolve_api_key(&self, provider: &AIProvider) -> Result<String, BullShiftError> {
        if self.security_manager.has_api_key(&provider.name) {
            self.security_manager.get_api_key(&provider.name)
        } else {
            Ok(provider.api_key.clone())
        }
    }

    // AI Request Execution
    async fn send_ai_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, BullShiftError> {
        let start_time = std::time::Instant::now();

        // Decrypt API key for this request
        let decrypted_key = self.resolve_api_key(provider)?;
        let mut provider_with_key = provider.clone();
        provider_with_key.api_key = decrypted_key;
        let provider = &provider_with_key;

        let response = match provider.provider_type {
            AIProviderType::OpenAI => {
                self.send_openai_request(provider, prompt).await
            }
            AIProviderType::Anthropic => {
                self.send_anthropic_request(provider, prompt).await
            }
            AIProviderType::Ollama => {
                self.send_ollama_request(provider, prompt).await
            }
            AIProviderType::LocalLLM => {
                self.send_local_llm_request(provider, prompt).await
            }
            AIProviderType::SecureYeoman => {
                self.send_secureyeoman_request(provider, prompt).await
            }
            AIProviderType::Custom => {
                self.send_custom_request(provider, prompt).await
            }
        };
        
        let response_time = start_time.elapsed().as_millis() as u64;
        
        match response {
            Ok(mut ai_response) => {
                ai_response.response_time_ms = response_time;
                Ok(ai_response)
            }
            Err(e) => {
                Ok(AIResponse {
                    id: Uuid::new_v4(),
                    provider_id: provider.id,
                    prompt_id: Uuid::new_v4(),
                    response: String::new(),
                    tokens_used: 0,
                    cost: 0.0,
                    response_time_ms: response_time,
                    success: false,
                    error_message: Some(e.to_string()),
                    timestamp: Utc::now(),
                })
            }
        }
    }

    // Generic connection test — sends GET to the given health URL
    async fn test_endpoint_connection(&self, url: &str, provider_name: &str) -> Result<bool, BullShiftError> {
        match self.client.get(url).send().await {
            Ok(response) => Ok(response.status().is_success() || response.status() == reqwest::StatusCode::UNAUTHORIZED),
            Err(e) => Err(BullShiftError::AiBridge(format!("{} connection test failed: {}", provider_name, e))),
        }
    }

    // Generic AI request — handles the common post→parse→build response pattern
    async fn post_ai_request(
        &self,
        provider: &AIProvider,
        url: &str,
        body: &serde_json::Value,
        auth_headers: Vec<(&str, String)>,
    ) -> Result<serde_json::Value, BullShiftError> {
        let mut request = self.client
            .post(url)
            .header("Content-Type", "application/json");

        for (key, value) in auth_headers {
            request = request.header(key, value);
        }

        let response = request
            .json(body)
            .send()
            .await
            .map_err(|e| BullShiftError::AiBridge(format!("Request failed: {}", e)))?;

        if response.status().is_success() {
            response.json::<serde_json::Value>().await
                .map_err(|e| BullShiftError::AiBridge(format!("Failed to parse response: {}", e)))
        } else {
            Err(BullShiftError::Api(format!("AI API error: {}", response.status())))
        }
    }

    fn build_ai_response(&self, provider: &AIProvider, content: &str, tokens_used: u32, cost_per_token: f64) -> AIResponse {
        AIResponse {
            id: Uuid::new_v4(),
            provider_id: provider.id,
            prompt_id: Uuid::new_v4(),
            response: content.to_string(),
            tokens_used,
            cost: tokens_used as f64 * cost_per_token,
            response_time_ms: 0,
            success: true,
            error_message: None,
            timestamp: Utc::now(),
        }
    }

    // Provider-specific connection tests
    async fn test_openai_connection(&self, provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, BullShiftError> {
        self.test_endpoint_connection(&format!("{}/models", provider.api_endpoint), "OpenAI").await
    }

    async fn test_anthropic_connection(&self, provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, BullShiftError> {
        self.test_endpoint_connection(&format!("{}/v1/messages", provider.api_endpoint), "Anthropic").await
    }

    async fn test_ollama_connection(&self, provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, BullShiftError> {
        self.test_endpoint_connection(&format!("{}/api/tags", provider.api_endpoint), "Ollama").await
    }

    async fn test_local_llm_connection(&self, provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, BullShiftError> {
        self.test_endpoint_connection(&format!("{}/health", provider.api_endpoint), "Local LLM").await
    }

    async fn test_custom_connection(&self, provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, BullShiftError> {
        self.test_endpoint_connection(&format!("{}/test", provider.api_endpoint), "Custom").await
    }

    async fn test_secureyeoman_connection(&self, provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, BullShiftError> {
        self.test_endpoint_connection(&format!("{}/api/v1/health", provider.api_endpoint), "SecureYeoman").await
    }

    // Provider-specific request implementations
    async fn send_openai_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, BullShiftError> {
        let body = serde_json::json!({
            "model": provider.model_name,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": provider.max_tokens,
            "temperature": provider.temperature
        });
        let url = format!("{}/chat/completions", provider.api_endpoint);
        let result = self.post_ai_request(provider, &url, &body, vec![
            ("Authorization", format!("Bearer {}", &provider.api_key)),
        ]).await?;

        let content = result["choices"][0]["message"]["content"].as_str().ok_or_else(|| BullShiftError::AiBridge("Invalid OpenAI response format".to_string()))?;
        let tokens_used = result["usage"]["total_tokens"].as_u64().ok_or_else(|| BullShiftError::AiBridge("Invalid token usage data".to_string()))? as u32;
        Ok(self.build_ai_response(provider, content, tokens_used, 0.002))
    }

    async fn send_anthropic_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, BullShiftError> {
        let body = serde_json::json!({
            "model": provider.model_name,
            "max_tokens": provider.max_tokens,
            "temperature": provider.temperature,
            "messages": [{"role": "user", "content": prompt}]
        });
        let url = format!("{}/v1/messages", provider.api_endpoint);
        let result = self.post_ai_request(provider, &url, &body, vec![
            ("x-api-key", provider.api_key.clone()),
            ("anthropic-version", "2023-06-01".to_string()),
        ]).await?;

        let content = result["content"][0]["text"].as_str().ok_or_else(|| BullShiftError::AiBridge("Invalid Anthropic response format".to_string()))?;
        let tokens_used = result["usage"]["input_tokens"].as_u64().unwrap_or(0) as u32
            + result["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;
        Ok(self.build_ai_response(provider, content, tokens_used, 0.015))
    }

    async fn send_ollama_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, BullShiftError> {
        let body = serde_json::json!({
            "model": provider.model_name,
            "prompt": prompt,
            "stream": false,
            "options": {"temperature": provider.temperature, "num_predict": provider.max_tokens}
        });
        let url = format!("{}/api/generate", provider.api_endpoint);
        let result = self.post_ai_request(provider, &url, &body, vec![]).await?;

        let content = result["response"].as_str().ok_or_else(|| BullShiftError::AiBridge("Invalid Ollama response format".to_string()))?;
        let tokens_used = result["prompt_eval_count"].as_u64().unwrap_or(0) as u32
            + result["eval_count"].as_u64().unwrap_or(0) as u32;
        Ok(self.build_ai_response(provider, content, tokens_used, 0.0))
    }

    async fn send_local_llm_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, BullShiftError> {
        let body = serde_json::json!({
            "inputs": prompt,
            "parameters": {"temperature": provider.temperature, "max_new_tokens": provider.max_tokens}
        });
        let url = format!("{}/generate", provider.api_endpoint);
        let result = self.post_ai_request(provider, &url, &body, vec![]).await?;

        let content = result["generated_text"].as_str()
            .or_else(|| result["0"]["generated_text"].as_str())
            .ok_or_else(|| BullShiftError::AiBridge("Invalid Local LLM response format".to_string()))?;
        Ok(self.build_ai_response(provider, content, prompt.len() as u32, 0.0))
    }

    async fn send_secureyeoman_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, BullShiftError> {
        let body = serde_json::json!({
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": provider.max_tokens,
            "temperature": provider.temperature
        });
        let url = format!("{}/api/v1/chat", provider.api_endpoint);

        let mut auth_headers = Vec::new();
        if !provider.api_key.is_empty() {
            auth_headers.push(("Authorization", format!("Bearer {}", &provider.api_key)));
        }

        let result = self.post_ai_request(provider, &url, &body, auth_headers).await?;

        let content = result["choices"][0]["message"]["content"].as_str()
            .or_else(|| result["response"].as_str())
            .or_else(|| result["content"].as_str())
            .ok_or_else(|| BullShiftError::AiBridge("Invalid SecureYeoman response format".to_string()))?;
        let tokens_used = result["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32;
        Ok(self.build_ai_response(provider, content, tokens_used, 0.0))
    }

    async fn send_custom_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, BullShiftError> {
        let body = serde_json::json!({
            "prompt": prompt,
            "model": provider.model_name,
            "max_tokens": provider.max_tokens,
            "temperature": provider.temperature
        });
        let url = format!("{}/completions", provider.api_endpoint);
        let result = self.post_ai_request(provider, &url, &body, vec![]).await?;

        let content = result["text"].as_str()
            .or_else(|| result["response"].as_str())
            .or_else(|| result["completion"].as_str())
            .ok_or_else(|| BullShiftError::AiBridge("Invalid Custom provider response format".to_string()))?;
        let tokens_used = result["usage"].as_u64().unwrap_or(prompt.len() as u64) as u32;
        Ok(self.build_ai_response(provider, content, tokens_used, 0.001))
    }

    // Helper methods
    fn validate_provider(&self, provider: &AIProvider) -> Result<(), BullShiftError> {
        if provider.name.is_empty() {
            return Err(BullShiftError::Validation("Provider name cannot be empty".to_string()));
        }
        
        if provider.api_endpoint.is_empty() {
            return Err(BullShiftError::Validation("API endpoint cannot be empty".to_string()));
        }
        
        if provider.model_name.is_empty() {
            return Err(BullShiftError::Validation("Model name cannot be empty".to_string()));
        }
        
        Ok(())
    }

    fn validate_strategy(&self, strategy: &TradingStrategy) -> Result<(), BullShiftError> {
        if strategy.name.is_empty() {
            return Err(BullShiftError::Validation("Strategy name cannot be empty".to_string()));
        }
        
        if strategy.parameters.symbols.is_empty() {
            return Err(BullShiftError::Validation("Strategy must have at least one symbol".to_string()));
        }
        
        Ok(())
    }

    fn validate_prompt(&self, prompt: &AIPrompt) -> Result<(), BullShiftError> {
        if prompt.name.is_empty() {
            return Err(BullShiftError::Validation("Prompt name cannot be empty".to_string()));
        }
        
        if prompt.template.is_empty() {
            return Err(BullShiftError::Validation("Prompt template cannot be empty".to_string()));
        }
        
        Ok(())
    }

    fn encrypt_configuration(&self, config: &AIConfiguration) -> Result<AIConfiguration, BullShiftError> {
        // Encrypt API key
        let encrypted_api_key = self.security_manager.encrypt_sensitive_data(&config.api_key)?;
        
        // Encrypt organization ID if present
        let encrypted_org_id = if let Some(ref org_id) = config.organization_id {
            Some(self.security_manager.encrypt_sensitive_data(org_id)?)
        } else {
            None
        };
        
        // Encrypt custom headers
        let mut encrypted_headers = HashMap::new();
        for (key, value) in &config.custom_headers {
            let encrypted_value = self.security_manager.encrypt_sensitive_data(value)?;
            encrypted_headers.insert(key.clone(), encrypted_value);
        }
        
        Ok(AIConfiguration {
            provider_id: config.provider_id,
            api_key: encrypted_api_key,
            organization_id: encrypted_org_id,
            custom_headers: encrypted_headers,
            rate_limit: config.rate_limit.clone(),
            cost_tracking: config.cost_tracking.clone(),
        })
    }

    fn build_strategy_generation_prompt(&self, strategy: &TradingStrategy) -> String {
        format!(
            "Generate a detailed trading strategy for the following parameters:
            
            Strategy Type: {:?}
            Symbols: {:?}
            Timeframe: {}
            Risk Level: {:?}
            Position Sizing: {:?}
            Stop Loss: {}%
            Take Profit: {}%
            Max Positions: {}
            
            Please provide a comprehensive strategy description including:
            1. Entry conditions
            2. Exit conditions
            3. Risk management rules
            4. Market conditions to avoid
            5. Performance expectations",
            strategy.strategy_type,
            strategy.parameters.symbols,
            strategy.parameters.timeframe,
            strategy.parameters.risk_level,
            strategy.parameters.position_sizing,
            strategy.parameters.stop_loss * 100.0,
            strategy.parameters.take_profit * 100.0,
            strategy.parameters.max_positions
        )
    }

    fn substitute_prompt_variables(&self, template: &str, variables: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        
        for (key, value) in variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        
        result
    }

    // Public interface methods
    pub fn get_providers(&self) -> Vec<&AIProvider> {
        self.providers.values().collect()
    }

    pub fn get_strategies(&self) -> Vec<&TradingStrategy> {
        self.strategies.values().collect()
    }

    pub fn get_prompts(&self) -> Vec<&AIPrompt> {
        self.prompts.values().collect()
    }

    pub fn get_provider(&self, provider_id: &Uuid) -> Option<&AIProvider> {
        self.providers.get(provider_id)
    }

    pub fn get_strategy(&self, strategy_id: &Uuid) -> Option<&TradingStrategy> {
        self.strategies.get(strategy_id)
    }

    /// Update the encrypted API key for an existing provider.
    pub fn update_provider_api_key(&mut self, provider_id: &Uuid, new_api_key: &str) -> Result<(), BullShiftError> {
        let provider = self.providers.get(provider_id)
            .ok_or_else(|| BullShiftError::AiBridge("Provider not found".to_string()))?;
        self.security_manager.store_api_key(&provider.name, new_api_key)?;
        log::info!("Updated encrypted API key for provider: {:?}", provider_id);
        Ok(())
    }

    /// Check if a provider has an encrypted API key stored.
    pub fn has_encrypted_api_key(&self, provider_id: &Uuid) -> bool {
        self.providers.get(provider_id)
            .map(|p| self.security_manager.has_api_key(&p.name))
            .unwrap_or(false)
    }

    pub fn is_provider_configured(&self, provider_id: &Uuid) -> bool {
        self.configurations.contains_key(provider_id)
    }

    pub async fn get_provider_usage_stats(&self, provider_id: &Uuid) -> Result<UsageStats, BullShiftError> {
        let provider_responses: Vec<_> = self.responses
            .iter()
            .filter(|r| r.provider_id == *provider_id)
            .collect();
        
        let total_requests = provider_responses.len();
        let total_tokens = provider_responses.iter().map(|r| r.tokens_used).sum::<u32>();
        let total_cost = provider_responses.iter().map(|r| r.cost).sum::<f64>();
        let success_rate = if total_requests > 0 {
            provider_responses.iter().filter(|r| r.success).count() as f64 / total_requests as f64
        } else {
            0.0
        };
        
        Ok(UsageStats {
            total_requests,
            total_tokens,
            total_cost,
            success_rate,
            avg_response_time_ms: if total_requests > 0 {
                provider_responses.iter().map(|r| r.response_time_ms).sum::<u64>() / total_requests as u64
            } else {
                0
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub total_requests: usize,
    pub total_tokens: u32,
    pub total_cost: f64,
    pub success_rate: f64,
    pub avg_response_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_security_manager() -> crate::security::SecurityManager {
        crate::security::SecurityManager::new().expect("SecurityManager should initialize")
    }

    fn test_provider(provider_type: AIProviderType, name: &str, api_key: &str) -> AIProvider {
        AIProvider {
            id: Uuid::new_v4(),
            name: name.to_string(),
            provider_type,
            api_endpoint: "http://localhost:18789".to_string(),
            api_key: api_key.to_string(),
            model_name: "test-model".to_string(),
            is_configured: false,
            is_active: false,
            max_tokens: 4096,
            temperature: 0.7,
            created_at: Utc::now(),
            last_used: None,
        }
    }

    #[tokio::test]
    async fn test_api_key_encrypted_on_add() {
        let sm = test_security_manager();
        let mut bearly = BearlyManaged::new(sm);
        let provider = test_provider(AIProviderType::OpenAI, "test_openai", "sk-secret-key-12345");
        let provider_id = provider.id;

        bearly.add_provider(provider).await.unwrap();

        // In-memory provider should have its key cleared
        let stored = bearly.get_provider(&provider_id).unwrap();
        assert!(stored.api_key.is_empty(), "plaintext key should be cleared from provider struct");

        // Key should be retrievable via SecurityManager
        assert!(bearly.has_encrypted_api_key(&provider_id));
    }

    #[tokio::test]
    async fn test_api_key_update() {
        let sm = test_security_manager();
        let mut bearly = BearlyManaged::new(sm);
        let provider = test_provider(AIProviderType::Anthropic, "test_anthropic", "sk-old-key");
        let provider_id = provider.id;

        bearly.add_provider(provider).await.unwrap();

        // Update the key
        bearly.update_provider_api_key(&provider_id, "sk-new-key-67890").unwrap();

        // Resolve should return the new key
        let stored_provider = bearly.get_provider(&provider_id).unwrap();
        let resolved = bearly.resolve_api_key(stored_provider).unwrap();
        assert_eq!(resolved, "sk-new-key-67890");
    }

    #[tokio::test]
    async fn test_api_key_resolve_decrypts() {
        let sm = test_security_manager();
        let mut bearly = BearlyManaged::new(sm);
        let provider = test_provider(AIProviderType::Ollama, "test_ollama", "my-secret-token");
        let provider_id = provider.id;

        bearly.add_provider(provider).await.unwrap();

        let stored = bearly.get_provider(&provider_id).unwrap();
        let decrypted = bearly.resolve_api_key(stored).unwrap();
        assert_eq!(decrypted, "my-secret-token");
    }

    #[test]
    fn test_secureyeoman_provider_type_exists() {
        let provider = test_provider(AIProviderType::SecureYeoman, "SecureYeoman Agent", "");
        assert!(matches!(provider.provider_type, AIProviderType::SecureYeoman));
    }

    #[test]
    fn test_secureyeoman_provider_defaults() {
        let provider = test_provider(AIProviderType::SecureYeoman, "SecureYeoman Agent", "");
        assert_eq!(provider.api_endpoint, "http://localhost:18789");
        assert_eq!(provider.model_name, "test-model");
        assert_eq!(provider.max_tokens, 4096);
    }

    #[tokio::test]
    async fn test_provider_without_key_still_works() {
        let sm = test_security_manager();
        let mut bearly = BearlyManaged::new(sm);
        // SecureYeoman might not need a key (local service)
        let provider = test_provider(AIProviderType::SecureYeoman, "sy_local", "");
        let provider_id = provider.id;

        bearly.add_provider(provider).await.unwrap();

        let stored = bearly.get_provider(&provider_id).unwrap();
        let resolved = bearly.resolve_api_key(stored).unwrap();
        assert!(resolved.is_empty());
        assert!(!bearly.has_encrypted_api_key(&provider_id));
    }

    #[test]
    fn test_security_manager_api_key_roundtrip() {
        let mut sm = test_security_manager();
        sm.store_api_key("test_provider", "super-secret-key-123").unwrap();
        assert!(sm.has_api_key("test_provider"));

        let decrypted = sm.get_api_key("test_provider").unwrap();
        assert_eq!(decrypted, "super-secret-key-123");
    }

    #[test]
    fn test_security_manager_api_key_remove() {
        let mut sm = test_security_manager();
        sm.store_api_key("removable", "key-to-remove").unwrap();
        assert!(sm.has_api_key("removable"));

        sm.remove_api_key("removable").unwrap();
        assert!(!sm.has_api_key("removable"));
    }

    #[test]
    fn test_security_manager_api_key_not_found() {
        let sm = test_security_manager();
        let result = sm.get_api_key("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_provider_empty_name() {
        let sm = test_security_manager();
        let bearly = BearlyManaged::new(sm);
        let provider = test_provider(AIProviderType::OpenAI, "", "key");
        assert!(bearly.validate_provider(&provider).is_err());
    }
}
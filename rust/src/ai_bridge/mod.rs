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
    responses: Vec<AIResponse>,
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
            responses: Vec::new(),
            security_manager,
        }
    }

    // Provider Management
    pub async fn add_provider(&mut self, provider: AIProvider) -> Result<Uuid, String> {
        let provider_id = provider.id;
        
        // Validate provider configuration
        self.validate_provider(&provider)?;
        
        // Store provider
        self.providers.insert(provider_id, provider);
        
        log::info!("Added AI provider: {:?}", provider_id);
        Ok(provider_id)
    }

    pub async fn configure_provider(&mut self, provider_id: Uuid, config: AIConfiguration) -> Result<(), String> {
        // Validate provider exists
        if !self.providers.contains_key(&provider_id) {
            return Err("Provider not found".to_string());
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

    pub async fn test_provider_connection(&self, provider_id: Uuid) -> Result<bool, String> {
        let provider = self.providers.get(&provider_id)
            .ok_or("Provider not found")?;
        
        let config = self.configurations.get(&provider_id)
            .ok_or("Provider not configured")?;

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
            AIProviderType::Custom => {
                self.test_custom_connection(provider, config).await
            }
        }
    }

    // Strategy Management
    pub async fn create_strategy(&mut self, strategy: TradingStrategy) -> Result<Uuid, String> {
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

    pub async fn generate_ai_strategy(&self, strategy: &TradingStrategy) -> Result<TradingStrategy, String> {
        let provider = self.providers.get(&strategy.provider_id)
            .ok_or("Provider not found")?;
        
        let prompt = self.build_strategy_generation_prompt(strategy);
        
        let response = self.send_ai_request(provider, &prompt).await?;
        
        if !response.success {
            return Err(response.error_message.unwrap_or("AI request failed".to_string()));
        }
        
        // Parse AI response and update strategy
        let mut enhanced_strategy = strategy.clone();
        enhanced_strategy.description = response.response;
        enhanced_strategy.last_updated = Utc::now();
        
        Ok(enhanced_strategy)
    }

    // Prompt Management
    pub async fn add_prompt(&mut self, prompt: AIPrompt) -> Result<Uuid, String> {
        let prompt_id = prompt.id;
        
        // Validate prompt
        self.validate_prompt(&prompt)?;
        
        // Store prompt
        self.prompts.insert(prompt_id, prompt);
        
        log::info!("Added AI prompt: {:?}", prompt_id);
        Ok(prompt_id)
    }

    pub async fn execute_prompt(&mut self, prompt_id: Uuid, variables: HashMap<String, String>) -> Result<AIResponse, String> {
        let prompt = self.prompts.get(&prompt_id)
            .ok_or("Prompt not found")?;
        
        let provider = self.providers.get(&prompt.provider_id)
            .ok_or("Provider not found")?;
        
        // Build final prompt with variables
        let final_prompt = self.substitute_prompt_variables(&prompt.template, &variables);
        
        // Send AI request
        let response = self.send_ai_request(provider, &final_prompt).await?;
        
        // Store response
        self.responses.push(response.clone());
        
        Ok(response)
    }

    // AI Request Execution
    async fn send_ai_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, String> {
        let start_time = std::time::Instant::now();
        
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
                    error_message: Some(e),
                    timestamp: Utc::now(),
                })
            }
        }
    }

    // Provider-specific implementations
    async fn test_openai_connection(&self, provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, String> {
        let test_url = format!("{}/models", provider.api_endpoint);
        
        match self.client.get(&test_url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => Err(format!("OpenAI connection test failed: {}", e)),
        }
    }

    async fn send_openai_request(&self, provider: &AIProvider, prompt: &str) -> Result<AIResponse, String> {
        let request_body = serde_json::json!({
            "model": provider.model_name,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": provider.max_tokens,
            "temperature": provider.temperature
        });

        let url = format!("{}/chat/completions", provider.api_endpoint);
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer YOUR_API_KEY") // Would use encrypted key
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("OpenAI request failed: {}", e))?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await
                .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;
            
            let content = result["choices"][0]["message"]["content"]
                .as_str()
                .ok_or("Invalid OpenAI response format")?;
            
            let tokens_used = result["usage"]["total_tokens"]
                .as_u64()
                .ok_or("Invalid token usage data")? as u32;
            
            Ok(AIResponse {
                id: Uuid::new_v4(),
                provider_id: provider.id,
                prompt_id: Uuid::new_v4(),
                response: content.to_string(),
                tokens_used,
                cost: tokens_used as f64 * 0.002, // OpenAI pricing
                response_time_ms: 0,
                success: true,
                error_message: None,
                timestamp: Utc::now(),
            })
        } else {
            Err(format!("OpenAI API error: {}", response.status()))
        }
    }

    // Placeholder implementations for other providers
    async fn test_anthropic_connection(&self, _provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, String> {
        // TODO: Implement Anthropic connection test
        Ok(true)
    }

    async fn send_anthropic_request(&self, _provider: &AIProvider, _prompt: &str) -> Result<AIResponse, String> {
        // TODO: Implement Anthropic request
        Err("Anthropic provider not implemented yet".to_string())
    }

    async fn test_ollama_connection(&self, _provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, String> {
        // TODO: Implement Ollama connection test
        Ok(true)
    }

    async fn send_ollama_request(&self, _provider: &AIProvider, _prompt: &str) -> Result<AIResponse, String> {
        // TODO: Implement Ollama request
        Err("Ollama provider not implemented yet".to_string())
    }

    async fn test_local_llm_connection(&self, _provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, String> {
        // TODO: Implement Local LLM connection test
        Ok(true)
    }

    async fn send_local_llm_request(&self, _provider: &AIProvider, _prompt: &str) -> Result<AIResponse, String> {
        // TODO: Implement Local LLM request
        Err("Local LLM provider not implemented yet".to_string())
    }

    async fn test_custom_connection(&self, _provider: &AIProvider, _config: &AIConfiguration) -> Result<bool, String> {
        // TODO: Implement Custom connection test
        Ok(true)
    }

    async fn send_custom_request(&self, _provider: &AIProvider, _prompt: &str) -> Result<AIResponse, String> {
        // TODO: Implement Custom request
        Err("Custom provider not implemented yet".to_string())
    }

    // Helper methods
    fn validate_provider(&self, provider: &AIProvider) -> Result<(), String> {
        if provider.name.is_empty() {
            return Err("Provider name cannot be empty".to_string());
        }
        
        if provider.api_endpoint.is_empty() {
            return Err("API endpoint cannot be empty".to_string());
        }
        
        if provider.model_name.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }
        
        Ok(())
    }

    fn validate_strategy(&self, strategy: &TradingStrategy) -> Result<(), String> {
        if strategy.name.is_empty() {
            return Err("Strategy name cannot be empty".to_string());
        }
        
        if strategy.parameters.symbols.is_empty() {
            return Err("Strategy must have at least one symbol".to_string());
        }
        
        Ok(())
    }

    fn validate_prompt(&self, prompt: &AIPrompt) -> Result<(), String> {
        if prompt.name.is_empty() {
            return Err("Prompt name cannot be empty".to_string());
        }
        
        if prompt.template.is_empty() {
            return Err("Prompt template cannot be empty".to_string());
        }
        
        Ok(())
    }

    fn encrypt_configuration(&self, config: &AIConfiguration) -> Result<AIConfiguration, String> {
        // TODO: Implement encryption using security manager
        Ok(config.clone())
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

    pub fn is_provider_configured(&self, provider_id: &Uuid) -> bool {
        self.configurations.contains_key(provider_id)
    }

    pub async fn get_provider_usage_stats(&self, provider_id: &Uuid) -> Result<UsageStats, String> {
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
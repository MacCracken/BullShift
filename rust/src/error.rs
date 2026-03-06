use thiserror::Error;

#[derive(Error, Debug)]
pub enum BullShiftError {
    #[error("Security error: {0}")]
    Security(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Keychain error: {0}")]
    Keychain(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Trading error: {0}")]
    Trading(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Database error: {0}")]
    DatabaseSql(#[from] rusqlite::Error),

    #[error("AI Bridge error: {0}")]
    AiBridge(String),

    #[error("Data stream error: {0}")]
    DataStream(String),

    #[error("Portfolio error: {0}")]
    Portfolio(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, BullShiftError>;

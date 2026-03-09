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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let security_err = BullShiftError::Security("access denied".to_string());
        assert_eq!(format!("{}", security_err), "Security error: access denied");

        let api_err = BullShiftError::Api("rate limited".to_string());
        assert_eq!(format!("{}", api_err), "API error: rate limited");

        let trading_err = BullShiftError::Trading("insufficient funds".to_string());
        assert_eq!(
            format!("{}", trading_err),
            "Trading error: insufficient funds"
        );

        let unknown_err = BullShiftError::Unknown("something went wrong".to_string());
        assert_eq!(
            format!("{}", unknown_err),
            "Unknown error: something went wrong"
        );
    }

    #[test]
    fn test_error_from_rusqlite() {
        let rusqlite_err = rusqlite::Error::QueryReturnedNoRows;
        let err: BullShiftError = BullShiftError::from(rusqlite_err);
        match err {
            BullShiftError::DatabaseSql(_) => {} // expected
            other => panic!("Expected DatabaseSql variant, got: {:?}", other),
        }
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: BullShiftError = BullShiftError::from(io_err);
        match err {
            BullShiftError::Io(_) => {} // expected
            other => panic!("Expected Io variant, got: {:?}", other),
        }
        assert!(format!("{}", err).contains("file not found"));
    }

    #[test]
    fn test_error_from_serde() {
        // Create a serde_json error by attempting to parse invalid JSON
        let serde_err = serde_json::from_str::<serde_json::Value>("not valid json").unwrap_err();
        let err: BullShiftError = BullShiftError::from(serde_err);
        match err {
            BullShiftError::Serialization(_) => {} // expected
            other => panic!("Expected Serialization variant, got: {:?}", other),
        }
    }

    #[test]
    fn test_result_type_alias() {
        // Verify that Result<i32> works as std::result::Result<i32, BullShiftError>
        let ok_result: Result<i32> = Ok(42);
        assert_eq!(ok_result.unwrap(), 42);

        let err_result: Result<i32> = Err(BullShiftError::Validation("invalid".to_string()));
        assert!(err_result.is_err());
        let err = err_result.unwrap_err();
        assert_eq!(format!("{}", err), "Validation error: invalid");
    }
}

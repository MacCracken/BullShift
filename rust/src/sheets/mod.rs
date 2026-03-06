use crate::error::BullShiftError;
use crate::trading::api::{ApiAccount, ApiPosition};
use crate::trading::trade_history::Trade;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Supported export formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Csv,
    Tsv,
    Json,
    GoogleSheets,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Csv => write!(f, "CSV"),
            Self::Tsv => write!(f, "TSV"),
            Self::Json => write!(f, "JSON"),
            Self::GoogleSheets => write!(f, "Google Sheets"),
        }
    }
}

/// Data types that can be exported.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportDataType {
    Trades,
    Positions,
    Account,
    Sentiment,
    AuditLog,
    Portfolio,
}

/// Configuration for a Google Sheets connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleSheetsConfig {
    pub spreadsheet_id: String,
    pub api_key: Option<String>,
    pub service_account_json: Option<String>,
    pub sheet_name: String,
}

/// Configuration for a scheduled export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSchedule {
    pub id: Uuid,
    pub name: String,
    pub data_type: ExportDataType,
    pub format: ExportFormat,
    pub destination: ExportDestination,
    pub interval_secs: u64,
    pub enabled: bool,
    pub last_export: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Where exported data goes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportDestination {
    File(String),
    GoogleSheets(GoogleSheetsConfig),
    Webhook(String),
}

/// A row of tabular data for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRow {
    pub values: Vec<String>,
}

/// Result of an export operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub rows_exported: usize,
    pub format: ExportFormat,
    pub timestamp: DateTime<Utc>,
    pub destination: String,
}

/// Manages data exports to Excel (CSV/TSV) and Google Sheets.
pub struct SheetsManager {
    schedules: HashMap<Uuid, ExportSchedule>,
    client: Client,
}

impl Default for SheetsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SheetsManager {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
            client: Client::new(),
        }
    }

    /// Add a scheduled export.
    pub fn add_schedule(&mut self, schedule: ExportSchedule) -> Uuid {
        let id = schedule.id;
        self.schedules.insert(id, schedule);
        id
    }

    /// Remove a scheduled export.
    pub fn remove_schedule(&mut self, id: &Uuid) -> bool {
        self.schedules.remove(id).is_some()
    }

    /// List all export schedules.
    pub fn list_schedules(&self) -> Vec<&ExportSchedule> {
        self.schedules.values().collect()
    }

    // --- CSV/TSV generation ---

    /// Export trades to CSV format.
    pub fn trades_to_csv(trades: &[Trade]) -> String {
        let mut output = String::from(
            "Order ID,Symbol,Side,Quantity,Price,Commission,Executed At\n",
        );
        for t in trades {
            output.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                csv_escape(&t.order_id),
                csv_escape(&t.symbol),
                csv_escape(&t.side),
                t.quantity,
                t.price,
                t.commission,
                csv_escape(&t.executed_at),
            ));
        }
        output
    }

    /// Export trades to TSV format.
    pub fn trades_to_tsv(trades: &[Trade]) -> String {
        let mut output = String::from(
            "Order ID\tSymbol\tSide\tQuantity\tPrice\tCommission\tExecuted At\n",
        );
        for t in trades {
            output.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                t.order_id, t.symbol, t.side, t.quantity, t.price, t.commission, t.executed_at,
            ));
        }
        output
    }

    /// Export positions to CSV format.
    pub fn positions_to_csv(positions: &[ApiPosition]) -> String {
        let mut output = String::from(
            "Symbol,Quantity,Entry Price,Current Price,Unrealized P&L\n",
        );
        for p in positions {
            output.push_str(&format!(
                "{},{},{},{},{}\n",
                csv_escape(&p.symbol),
                p.quantity,
                p.entry_price,
                p.current_price,
                p.unrealized_pnl,
            ));
        }
        output
    }

    /// Export account summary to CSV.
    pub fn account_to_csv(account: &ApiAccount) -> String {
        format!(
            "Balance,Available,Margin Used\n{},{},{}\n",
            account.balance, account.available, account.margin_used,
        )
    }

    /// Generic export: takes headers and rows, outputs CSV or TSV.
    pub fn tabular_export(
        headers: &[&str],
        rows: &[DataRow],
        format: &ExportFormat,
    ) -> String {
        let sep = match format {
            ExportFormat::Tsv => "\t",
            _ => ",",
        };

        let mut output = headers.join(sep);
        output.push('\n');

        for row in rows {
            let line: Vec<String> = row.values.iter().map(|v| {
                if matches!(format, ExportFormat::Csv) {
                    csv_escape(v)
                } else {
                    v.clone()
                }
            }).collect();
            output.push_str(&line.join(sep));
            output.push('\n');
        }

        output
    }

    // --- Google Sheets integration ---

    /// Append rows to a Google Sheets spreadsheet via the Sheets API v4.
    pub async fn append_to_google_sheets(
        &self,
        config: &GoogleSheetsConfig,
        headers: &[&str],
        rows: &[DataRow],
    ) -> Result<ExportResult, BullShiftError> {
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| BullShiftError::Configuration("Google Sheets API key required".to_string()))?;

        let range = format!("{}!A1", config.sheet_name);
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption=USER_ENTERED&key={}",
            config.spreadsheet_id, range, api_key,
        );

        let mut values: Vec<Vec<String>> = Vec::with_capacity(rows.len() + 1);
        values.push(headers.iter().map(|h| h.to_string()).collect());
        for row in rows {
            values.push(row.values.clone());
        }

        let body = serde_json::json!({
            "range": range,
            "majorDimension": "ROWS",
            "values": values,
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| BullShiftError::Network(format!("Google Sheets API failed: {}", e)))?;

        if resp.status().is_success() {
            Ok(ExportResult {
                rows_exported: rows.len(),
                format: ExportFormat::GoogleSheets,
                timestamp: Utc::now(),
                destination: format!("sheets:{}", config.spreadsheet_id),
            })
        } else {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            Err(BullShiftError::Api(format!(
                "Google Sheets append failed ({}): {}",
                status, body_text
            )))
        }
    }

    /// Read data from a Google Sheets spreadsheet.
    pub async fn read_from_google_sheets(
        &self,
        config: &GoogleSheetsConfig,
        range: &str,
    ) -> Result<Vec<DataRow>, BullShiftError> {
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| BullShiftError::Configuration("Google Sheets API key required".to_string()))?;

        let full_range = format!("{}!{}", config.sheet_name, range);
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
            config.spreadsheet_id, full_range, api_key,
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| BullShiftError::Network(format!("Google Sheets read failed: {}", e)))?;

        if resp.status().is_success() {
            let body: serde_json::Value = resp.json().await.map_err(|e| {
                BullShiftError::Api(format!("Failed to parse Sheets response: {}", e))
            })?;

            let rows = body["values"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|row| DataRow {
                            values: row
                                .as_array()
                                .map(|cells| {
                                    cells
                                        .iter()
                                        .map(|c| c.as_str().unwrap_or("").to_string())
                                        .collect()
                                })
                                .unwrap_or_default(),
                        })
                        .collect()
                })
                .unwrap_or_default();

            Ok(rows)
        } else {
            Err(BullShiftError::Api(format!(
                "Google Sheets read failed: {}",
                resp.status()
            )))
        }
    }

    /// Export trades to the configured destination.
    pub async fn export_trades(
        &self,
        trades: &[Trade],
        format: &ExportFormat,
        destination: &ExportDestination,
    ) -> Result<ExportResult, BullShiftError> {
        match destination {
            ExportDestination::File(path) => {
                let content = match format {
                    ExportFormat::Csv => Self::trades_to_csv(trades),
                    ExportFormat::Tsv => Self::trades_to_tsv(trades),
                    ExportFormat::Json => serde_json::to_string_pretty(trades)
                        .map_err(|e| BullShiftError::Api(format!("JSON serialize failed: {}", e)))?,
                    ExportFormat::GoogleSheets => {
                        return Err(BullShiftError::Configuration(
                            "Use GoogleSheets destination for Sheets export".to_string(),
                        ));
                    }
                };
                std::fs::write(path, &content)
                    .map_err(BullShiftError::Io)?;
                Ok(ExportResult {
                    rows_exported: trades.len(),
                    format: format.clone(),
                    timestamp: Utc::now(),
                    destination: format!("file:{}", path),
                })
            }
            ExportDestination::GoogleSheets(config) => {
                let headers = &["Order ID", "Symbol", "Side", "Quantity", "Price", "Commission", "Executed At"];
                let rows: Vec<DataRow> = trades
                    .iter()
                    .map(|t| DataRow {
                        values: vec![
                            t.order_id.clone(),
                            t.symbol.clone(),
                            t.side.clone(),
                            t.quantity.to_string(),
                            t.price.to_string(),
                            t.commission.to_string(),
                            t.executed_at.clone(),
                        ],
                    })
                    .collect();
                self.append_to_google_sheets(config, headers, &rows).await
            }
            ExportDestination::Webhook(url) => {
                let body = serde_json::to_value(trades)
                    .map_err(|e| BullShiftError::Api(format!("Serialize failed: {}", e)))?;
                let resp = self
                    .client
                    .post(url)
                    .json(&body)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .map_err(|e| BullShiftError::Network(format!("Export webhook failed: {}", e)))?;
                if resp.status().is_success() {
                    Ok(ExportResult {
                        rows_exported: trades.len(),
                        format: ExportFormat::Json,
                        timestamp: Utc::now(),
                        destination: format!("webhook:{}", url),
                    })
                } else {
                    Err(BullShiftError::Api(format!(
                        "Webhook export failed: {}",
                        resp.status()
                    )))
                }
            }
        }
    }
}

/// Escape a value for CSV (RFC 4180).
fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_trades() -> Vec<Trade> {
        vec![
            Trade {
                order_id: "ord-001".to_string(),
                symbol: "AAPL".to_string(),
                side: "BUY".to_string(),
                quantity: 100.0,
                price: 150.25,
                commission: 1.50,
                executed_at: "2026-03-05T10:30:00Z".to_string(),
            },
            Trade {
                order_id: "ord-002".to_string(),
                symbol: "TSLA".to_string(),
                side: "SELL".to_string(),
                quantity: 50.0,
                price: 200.00,
                commission: 1.00,
                executed_at: "2026-03-05T11:00:00Z".to_string(),
            },
        ]
    }

    #[test]
    fn test_trades_to_csv() {
        let csv = SheetsManager::trades_to_csv(&sample_trades());
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines[0], "Order ID,Symbol,Side,Quantity,Price,Commission,Executed At");
        assert!(lines[1].contains("AAPL"));
        assert!(lines[2].contains("TSLA"));
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_trades_to_tsv() {
        let tsv = SheetsManager::trades_to_tsv(&sample_trades());
        let lines: Vec<&str> = tsv.lines().collect();
        assert!(lines[0].contains('\t'));
        assert!(lines[1].contains("AAPL"));
    }

    #[test]
    fn test_positions_to_csv() {
        let positions = vec![ApiPosition {
            symbol: "AAPL".to_string(),
            quantity: 100.0,
            entry_price: 150.0,
            current_price: 155.0,
            unrealized_pnl: 500.0,
        }];
        let csv = SheetsManager::positions_to_csv(&positions);
        assert!(csv.contains("AAPL"));
        assert!(csv.contains("500"));
    }

    #[test]
    fn test_account_to_csv() {
        let account = ApiAccount {
            balance: 50000.0,
            available: 45000.0,
            margin_used: 5000.0,
        };
        let csv = SheetsManager::account_to_csv(&account);
        assert!(csv.contains("50000"));
        assert!(csv.contains("45000"));
    }

    #[test]
    fn test_csv_escape() {
        assert_eq!(csv_escape("hello"), "hello");
        assert_eq!(csv_escape("hello,world"), "\"hello,world\"");
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(csv_escape("line\nbreak"), "\"line\nbreak\"");
    }

    #[test]
    fn test_tabular_export_csv() {
        let headers = &["Name", "Value"];
        let rows = vec![
            DataRow { values: vec!["Alpha".to_string(), "100".to_string()] },
            DataRow { values: vec!["Beta".to_string(), "200".to_string()] },
        ];
        let output = SheetsManager::tabular_export(headers, &rows, &ExportFormat::Csv);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[0], "Name,Value");
        assert_eq!(lines[1], "Alpha,100");
        assert_eq!(lines[2], "Beta,200");
    }

    #[test]
    fn test_tabular_export_tsv() {
        let headers = &["A", "B"];
        let rows = vec![DataRow {
            values: vec!["x".to_string(), "y".to_string()],
        }];
        let output = SheetsManager::tabular_export(headers, &rows, &ExportFormat::Tsv);
        assert!(output.contains("A\tB"));
        assert!(output.contains("x\ty"));
    }

    #[test]
    fn test_schedule_management() {
        let mut mgr = SheetsManager::new();
        let schedule = ExportSchedule {
            id: Uuid::new_v4(),
            name: "Daily trades".to_string(),
            data_type: ExportDataType::Trades,
            format: ExportFormat::Csv,
            destination: ExportDestination::File("/tmp/trades.csv".to_string()),
            interval_secs: 86400,
            enabled: true,
            last_export: None,
            created_at: Utc::now(),
        };
        let id = mgr.add_schedule(schedule);
        assert_eq!(mgr.list_schedules().len(), 1);
        assert!(mgr.remove_schedule(&id));
        assert_eq!(mgr.list_schedules().len(), 0);
    }

    #[test]
    fn test_export_format_display() {
        assert_eq!(ExportFormat::Csv.to_string(), "CSV");
        assert_eq!(ExportFormat::GoogleSheets.to_string(), "Google Sheets");
    }

    #[test]
    fn test_json_export_format() {
        let trades = sample_trades();
        let json_str = serde_json::to_string_pretty(&trades).unwrap();
        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed.is_array());
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["symbol"], "AAPL");
        assert_eq!(arr[1]["symbol"], "TSLA");
        assert_eq!(arr[0]["quantity"], 100.0);
        assert_eq!(arr[1]["side"], "SELL");
    }

    #[test]
    fn test_empty_trades_export() {
        let empty: Vec<Trade> = vec![];
        let csv = SheetsManager::trades_to_csv(&empty);
        let lines: Vec<&str> = csv.lines().collect();
        // Should have only the header line
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Order ID,Symbol,Side,Quantity,Price,Commission,Executed At");

        let tsv = SheetsManager::trades_to_tsv(&empty);
        let tsv_lines: Vec<&str> = tsv.lines().collect();
        assert_eq!(tsv_lines.len(), 1);
        assert!(tsv_lines[0].contains("Order ID"));
    }

    #[test]
    fn test_csv_escape_special_chars() {
        // Comma in field
        assert_eq!(csv_escape("hello,world"), "\"hello,world\"");
        // Double quotes in field
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
        // Newline in field
        assert_eq!(csv_escape("line\nbreak"), "\"line\nbreak\"");
        // Multiple special chars
        assert_eq!(csv_escape("a,b\"c\nd"), "\"a,b\"\"c\nd\"");
        // No special chars — no quoting
        assert_eq!(csv_escape("plain text"), "plain text");
        // Empty string — no quoting
        assert_eq!(csv_escape(""), "");

        // Verify trades_to_csv applies escaping for a symbol with a comma
        let trades = vec![Trade {
            order_id: "ord,special".to_string(),
            symbol: "BRK.B".to_string(),
            side: "BUY".to_string(),
            quantity: 10.0,
            price: 400.0,
            commission: 0.0,
            executed_at: "2026-03-05T12:00:00Z".to_string(),
        }];
        let csv = SheetsManager::trades_to_csv(&trades);
        // The order_id with a comma should be quoted
        assert!(csv.contains("\"ord,special\""));
    }

    #[test]
    fn test_positions_to_json() {
        let positions = vec![
            ApiPosition {
                symbol: "AAPL".to_string(),
                quantity: 100.0,
                entry_price: 150.0,
                current_price: 160.0,
                unrealized_pnl: 1000.0,
            },
            ApiPosition {
                symbol: "GOOG".to_string(),
                quantity: 50.0,
                entry_price: 2800.0,
                current_price: 2750.0,
                unrealized_pnl: -2500.0,
            },
        ];
        let json_str = serde_json::to_string_pretty(&positions).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed.is_array());
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["symbol"], "AAPL");
        assert_eq!(arr[0]["unrealized_pnl"], 1000.0);
        assert_eq!(arr[1]["symbol"], "GOOG");
        assert_eq!(arr[1]["unrealized_pnl"], -2500.0);
    }

    #[test]
    fn test_account_to_tsv() {
        let account = ApiAccount {
            balance: 100000.0,
            available: 85000.0,
            margin_used: 15000.0,
        };
        // Use tabular_export with TSV format for account data
        let headers = &["Balance", "Available", "Margin Used"];
        let rows = vec![DataRow {
            values: vec![
                account.balance.to_string(),
                account.available.to_string(),
                account.margin_used.to_string(),
            ],
        }];
        let tsv = SheetsManager::tabular_export(headers, &rows, &ExportFormat::Tsv);
        let lines: Vec<&str> = tsv.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "Balance\tAvailable\tMargin Used");
        assert!(lines[1].contains("100000"));
        assert!(lines[1].contains("85000"));
        assert!(lines[1].contains("15000"));
        // Verify tab separation
        let fields: Vec<&str> = lines[1].split('\t').collect();
        assert_eq!(fields.len(), 3);
    }
}

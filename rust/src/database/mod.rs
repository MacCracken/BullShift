use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&data_dir).ok();
        let db_path = data_dir.join("bullshift.db");
        let conn = Connection::open(db_path)?;

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());

        conn.execute(
            "CREATE TABLE IF NOT EXISTS portfolios (
                id INTEGER PRIMARY KEY,
                cash_balance REAL NOT NULL,
                total_value REAL NOT NULL,
                available_margin REAL NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS positions (
                id INTEGER PRIMARY KEY,
                portfolio_id INTEGER NOT NULL,
                symbol TEXT NOT NULL,
                quantity REAL NOT NULL,
                entry_price REAL NOT NULL,
                current_price REAL NOT NULL,
                unrealized_pnl REAL NOT NULL,
                realized_pnl REAL NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (portfolio_id) REFERENCES portfolios(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS trades (
                id INTEGER PRIMARY KEY,
                order_id TEXT NOT NULL,
                symbol TEXT NOT NULL,
                side TEXT NOT NULL,
                quantity REAL NOT NULL,
                price REAL NOT NULL,
                commission REAL NOT NULL,
                executed_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_positions_symbol ON positions(symbol)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_trades_symbol ON trades(symbol)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_trades_executed_at ON trades(executed_at)",
            [],
        )?;

        Ok(())
    }

    pub fn save_portfolio(
        &self,
        cash_balance: f64,
        total_value: f64,
        available_margin: f64,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO portfolios (cash_balance, total_value, available_margin, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![cash_balance, total_value, available_margin, now, now],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn update_portfolio(
        &self,
        id: i64,
        cash_balance: f64,
        total_value: f64,
        available_margin: f64,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE portfolios SET cash_balance = ?1, total_value = ?2, available_margin = ?3, updated_at = ?4 WHERE id = ?5",
            params![cash_balance, total_value, available_margin, now, id],
        )?;

        Ok(())
    }

    pub fn get_portfolio(&self) -> Result<Option<(i64, f64, f64, f64)>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());

        let mut stmt = conn.prepare(
            "SELECT id, cash_balance, total_value, available_margin FROM portfolios ORDER BY id DESC LIMIT 1"
        )?;

        let result = stmt.query_row([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        });

        match result {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn save_position(
        &self,
        portfolio_id: i64,
        symbol: &str,
        quantity: f64,
        entry_price: f64,
        current_price: f64,
        unrealized_pnl: f64,
        realized_pnl: f64,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO positions (portfolio_id, symbol, quantity, entry_price, current_price, unrealized_pnl, realized_pnl, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![portfolio_id, symbol, quantity, entry_price, current_price, unrealized_pnl, realized_pnl, now, now],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn update_position(
        &self,
        id: i64,
        quantity: f64,
        entry_price: f64,
        current_price: f64,
        unrealized_pnl: f64,
        realized_pnl: f64,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE positions SET quantity = ?1, entry_price = ?2, current_price = ?3, unrealized_pnl = ?4, realized_pnl = ?5, updated_at = ?6 WHERE id = ?7",
            params![quantity, entry_price, current_price, unrealized_pnl, realized_pnl, now, id],
        )?;

        Ok(())
    }

    pub fn get_positions(&self, portfolio_id: i64) -> Result<Vec<Position>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());

        let mut stmt = conn.prepare(
            "SELECT id, symbol, quantity, entry_price, current_price, unrealized_pnl, realized_pnl FROM positions WHERE portfolio_id = ?1"
        )?;

        let positions = stmt.query_map([portfolio_id], |row| {
            Ok(Position {
                id: row.get(0)?,
                symbol: row.get(1)?,
                quantity: row.get(2)?,
                entry_price: row.get(3)?,
                current_price: row.get(4)?,
                unrealized_pnl: row.get(5)?,
                realized_pnl: row.get(6)?,
            })
        })?;

        positions.collect()
    }

    pub fn delete_position(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        conn.execute("DELETE FROM positions WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn save_trade(
        &self,
        order_id: &str,
        symbol: &str,
        side: &str,
        quantity: f64,
        price: f64,
        commission: f64,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO trades (order_id, symbol, side, quantity, price, commission, executed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![order_id, symbol, side, quantity, price, commission, now],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn get_trades(&self, symbol: Option<&str>, limit: Option<i64>) -> Result<Vec<Trade>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());

        let (query, params_vec): (&str, Vec<Box<dyn rusqlite::ToSql>>) = match symbol {
            Some(s) => (
                "SELECT id, order_id, symbol, side, quantity, price, commission, executed_at FROM trades WHERE symbol = ?1 ORDER BY executed_at DESC LIMIT ?2",
                vec![Box::new(s.to_string()), Box::new(limit.unwrap_or(100))],
            ),
            None => (
                "SELECT id, order_id, symbol, side, quantity, price, commission, executed_at FROM trades ORDER BY executed_at DESC LIMIT ?1",
                vec![Box::new(limit.unwrap_or(100))],
            ),
        };

        let mut stmt = conn.prepare(query)?;

        let trades = stmt.query_map(
            rusqlite::params_from_iter(params_vec.iter().map(|p| p.as_ref())),
            |row| {
                Ok(Trade {
                    id: row.get(0)?,
                    order_id: row.get(1)?,
                    symbol: row.get(2)?,
                    side: row.get(3)?,
                    quantity: row.get(4)?,
                    price: row.get(5)?,
                    commission: row.get(6)?,
                    executed_at: row.get(7)?,
                })
            },
        )?;

        trades.collect()
    }

    pub fn get_trades_by_date_range(&self, start_date: &str, end_date: &str) -> Result<Vec<Trade>> {
        let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());

        let mut stmt = conn.prepare(
            "SELECT id, order_id, symbol, side, quantity, price, commission, executed_at 
             FROM trades 
             WHERE executed_at BETWEEN ?1 AND ?2 
             ORDER BY executed_at DESC",
        )?;

        let trades = stmt.query_map([start_date, end_date], |row| {
            Ok(Trade {
                id: row.get(0)?,
                order_id: row.get(1)?,
                symbol: row.get(2)?,
                side: row.get(3)?,
                quantity: row.get(4)?,
                price: row.get(5)?,
                commission: row.get(6)?,
                executed_at: row.get(7)?,
            })
        })?;

        trades.collect()
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub id: i64,
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: i64,
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub price: f64,
    pub commission: f64,
    pub executed_at: String,
}

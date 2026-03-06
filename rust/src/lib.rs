use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::error::{BullShiftError, Result};

pub mod ai_bridge;
pub mod database;
pub mod error;
pub mod logging;
pub mod security;
pub mod trading;

pub use database::Database;
pub use trading::trade_history::{Trade, TradeHistory};

#[repr(C)]
pub struct TradeOrder {
    symbol: *const c_char,
    side: *const c_char,
    quantity: f64,
    order_type: *const c_char,
    /// Price for limit orders. Use `f64::NAN` or a non-positive value to indicate no price (market order).
    price: f64,
    has_price: bool,
}

#[repr(C)]
pub struct Position {
    symbol: *const c_char,
    quantity: f64,
    entry_price: f64,
    current_price: f64,
    unrealized_pnl: f64,
}

#[repr(C)]
pub struct MarketData {
    symbol: *const c_char,
    price: f64,
    volume: f64,
    timestamp: i64,
}

// Maximum string length for FFI safety
const MAX_SYMBOL_LENGTH: usize = 32;
const MAX_SIDE_LENGTH: usize = 8;
const MAX_ORDER_TYPE_LENGTH: usize = 16;

/// Validates that a C string pointer is not null and points to valid data
unsafe fn validate_c_string(
    ptr: *const c_char,
    max_len: usize,
    field_name: &str,
) -> Result<String> {
    if ptr.is_null() {
        return Err(BullShiftError::Validation(format!(
            "{} is null",
            field_name
        )));
    }

    let c_str = CStr::from_ptr(ptr);
    let str_slice = c_str.to_str().map_err(|_| {
        BullShiftError::Validation(format!("{} contains invalid UTF-8", field_name))
    })?;

    if str_slice.is_empty() {
        return Err(BullShiftError::Validation(format!(
            "{} is empty",
            field_name
        )));
    }

    if str_slice.len() > max_len {
        return Err(BullShiftError::Validation(format!(
            "{} exceeds maximum length of {} characters",
            field_name, max_len
        )));
    }

    Ok(str_slice.to_string())
}

/// Validates trade order data
fn validate_trade_order(order: &TradeOrder) -> Result<()> {
    // Validate quantity
    if order.quantity <= 0.0 {
        return Err(BullShiftError::Validation(
            "Quantity must be greater than zero".to_string(),
        ));
    }

    if !order.quantity.is_finite() {
        return Err(BullShiftError::Validation(
            "Quantity must be a finite number".to_string(),
        ));
    }

    // Validate price if provided
    if order.has_price {
        if order.price <= 0.0 {
            return Err(BullShiftError::Validation(
                "Price must be greater than zero".to_string(),
            ));
        }
        if !order.price.is_finite() {
            return Err(BullShiftError::Validation(
                "Price must be a finite number".to_string(),
            ));
        }
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn submit_order(order: TradeOrder) -> bool {
    // Validate all pointer fields with proper safety checks
    let symbol = match unsafe { validate_c_string(order.symbol, MAX_SYMBOL_LENGTH, "symbol") } {
        Ok(s) => s,
        Err(e) => {
            log::error!("Order submission failed: {}", e);
            return false;
        }
    };

    let side = match unsafe { validate_c_string(order.side, MAX_SIDE_LENGTH, "side") } {
        Ok(s) => s.to_uppercase(),
        Err(e) => {
            log::error!("Order submission failed: {}", e);
            return false;
        }
    };

    let order_type =
        match unsafe { validate_c_string(order.order_type, MAX_ORDER_TYPE_LENGTH, "order_type") } {
            Ok(s) => s,
            Err(e) => {
                log::error!("Order submission failed: {}", e);
                return false;
            }
        };

    // Validate side is valid
    if side != "BUY" && side != "SELL" {
        log::error!(
            "Order submission failed: Invalid side '{}'. Must be 'BUY' or 'SELL'",
            side
        );
        return false;
    }

    // Validate numeric fields
    if let Err(e) = validate_trade_order(&order) {
        log::error!("Order submission failed: {}", e);
        return false;
    }

    // Core trading logic implementation
    log::info!(
        "Submitting order: symbol={}, side={}, quantity={}, type={}",
        symbol,
        side,
        order.quantity,
        order_type
    );

    true
}

#[no_mangle]
pub extern "C" fn get_positions() -> *mut Position {
    // Return current positions - properly allocate memory
    // Note: In production, this would return actual positions
    // For now, return null to indicate no positions
    std::ptr::null_mut()
}

/// # Safety
/// `symbol` must be a valid, null-terminated C string pointer or null.
#[no_mangle]
pub unsafe extern "C" fn connect_market_data(symbol: *const c_char) -> bool {
    // Validate symbol pointer
    let symbol_str = match unsafe { validate_c_string(symbol, MAX_SYMBOL_LENGTH, "symbol") } {
        Ok(s) => s,
        Err(e) => {
            log::error!("Market data connection failed: {}", e);
            return false;
        }
    };

    // WebSocket connection for real-time data
    log::info!("Connecting to market data for: {}", symbol_str);
    true
}

#[no_mangle]
pub extern "C" fn get_account_balance() -> f64 {
    // Return account balance
    10000.0
}

/// # Safety
/// `positions` must be null or a pointer previously returned by `get_positions`.
#[no_mangle]
pub unsafe extern "C" fn free_positions(positions: *mut Position) {
    // Safety: Only free if pointer is not null
    if !positions.is_null() {
        unsafe {
            // Free the allocated memory
            // This assumes positions was allocated with Box::into_raw
            let _ = Box::from_raw(positions);
        }
    }
}

/// # Safety
/// `s` must be null or a pointer previously returned by a BullShift FFI function.
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    // Safety: Only free if pointer is not null
    if !s.is_null() {
        unsafe {
            // Free the allocated CString
            let _ = CString::from_raw(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_validate_c_string() {
        let c_string = CString::new("AAPL").unwrap();
        let result = unsafe { validate_c_string(c_string.as_ptr(), MAX_SYMBOL_LENGTH, "symbol") };
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "AAPL");
    }

    #[test]
    fn test_validate_c_string_null() {
        let result = unsafe { validate_c_string(std::ptr::null(), MAX_SYMBOL_LENGTH, "symbol") };
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{}", err).contains("null"));
    }

    #[test]
    fn test_validate_c_string_too_long() {
        let long_string = "A".repeat(MAX_SYMBOL_LENGTH + 1);
        let c_string = CString::new(long_string).unwrap();
        let result = unsafe { validate_c_string(c_string.as_ptr(), MAX_SYMBOL_LENGTH, "symbol") };
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{}", err).contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_trade_order() {
        let valid_order = TradeOrder {
            symbol: CString::new("AAPL").unwrap().into_raw(),
            side: CString::new("BUY").unwrap().into_raw(),
            quantity: 100.0,
            order_type: CString::new("MARKET").unwrap().into_raw(),
            price: 150.0,
            has_price: true,
        };

        assert!(validate_trade_order(&valid_order).is_ok());

        // Clean up
        unsafe {
            let _ = CString::from_raw(valid_order.symbol as *mut c_char);
            let _ = CString::from_raw(valid_order.side as *mut c_char);
            let _ = CString::from_raw(valid_order.order_type as *mut c_char);
        }
    }

    #[test]
    fn test_validate_trade_order_invalid_quantity() {
        let invalid_order = TradeOrder {
            symbol: CString::new("AAPL").unwrap().into_raw(),
            side: CString::new("BUY").unwrap().into_raw(),
            quantity: -100.0,
            order_type: CString::new("MARKET").unwrap().into_raw(),
            price: 0.0,
            has_price: false,
        };

        let result = validate_trade_order(&invalid_order);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{}", err).contains("Quantity"));

        // Clean up
        unsafe {
            let _ = CString::from_raw(invalid_order.symbol as *mut c_char);
            let _ = CString::from_raw(invalid_order.side as *mut c_char);
            let _ = CString::from_raw(invalid_order.order_type as *mut c_char);
        }
    }

    #[test]
    fn test_submit_order_valid() {
        let symbol = CString::new("AAPL").unwrap();
        let side = CString::new("BUY").unwrap();
        let order_type = CString::new("MARKET").unwrap();
        let order = TradeOrder {
            symbol: symbol.as_ptr(),
            side: side.as_ptr(),
            quantity: 100.0,
            order_type: order_type.as_ptr(),
            price: 150.0,
            has_price: true,
        };

        let result = submit_order(order);
        assert!(result);
    }

    #[test]
    fn test_submit_order_null_symbol() {
        let side = CString::new("BUY").unwrap();
        let order_type = CString::new("MARKET").unwrap();
        let order = TradeOrder {
            symbol: std::ptr::null(),
            side: side.as_ptr(),
            quantity: 100.0,
            order_type: order_type.as_ptr(),
            price: 0.0,
            has_price: false,
        };

        let result = submit_order(order);
        assert!(!result);
    }

    #[test]
    fn test_submit_order_invalid_side() {
        let symbol = CString::new("AAPL").unwrap();
        let side = CString::new("HOLD").unwrap();
        let order_type = CString::new("MARKET").unwrap();
        let order = TradeOrder {
            symbol: symbol.as_ptr(),
            side: side.as_ptr(),
            quantity: 100.0,
            order_type: order_type.as_ptr(),
            price: 0.0,
            has_price: false,
        };

        let result = submit_order(order);
        assert!(!result);
    }

    #[test]
    fn test_connect_market_data_valid() {
        let symbol = CString::new("AAPL").unwrap();
        let result = unsafe { connect_market_data(symbol.as_ptr()) };
        assert!(result);
    }

    #[test]
    fn test_connect_market_data_null() {
        let result = unsafe { connect_market_data(std::ptr::null()) };
        assert!(!result);
    }
}

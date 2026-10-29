use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub mod asset;
pub mod liability;
pub mod portfolio;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearContext {
    pub calendar_year: usize,
    pub user_age: usize,
    pub market_return: f64,
    pub inflation_rate: f64,
    pub statslaneranta: f64,
    pub prisbasbelopp: Decimal,
    pub inkomstbasbelopp: Decimal,
}

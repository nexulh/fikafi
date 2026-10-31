use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketContext {
    pub calendar_year: usize,
    pub calendar_month: usize,
    pub user_age: usize,
    pub monthly_market_return: f64,
    pub monthly_inflation_rate: f64,
    pub statslaneranta: f64,
    pub prisbasbelopp: Decimal,
    pub inkomstbasbelopp: Decimal,
}

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{Inkomstbasbelopp, Prisbasbelopp, Statslaneranta, YearMonth};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyMarketContext {
    pub current_year_month: YearMonth,
    pub monthly_market_return: Decimal,
    pub monthly_inflation_rate: Decimal,
    pub statslaneranta: Statslaneranta,
    pub prisbasbelopp: Prisbasbelopp,
    pub inkomstbasbelopp: Inkomstbasbelopp,
}

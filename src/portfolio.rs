use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::asset::AccountOperations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub assets: Vec<crate::asset::Account>,
    pub liabilities: Vec<crate::liability::Liability>,
    pub liquid_cash: Decimal,
}

impl Portfolio {
    pub fn total_nominal_value(&self) -> Decimal {
        self.assets
            .iter()
            .map(|account| account.value())
            .sum::<Decimal>()
            + self.liquid_cash
    }
}

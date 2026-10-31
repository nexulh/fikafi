use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::asset::AssetOperations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub assets: Vec<crate::asset::Asset>,
    pub liabilities: Vec<crate::liability::Liability>,
    pub liquid_cash: Decimal,
}

impl Portfolio {
    pub fn total_nominal_value(&self) -> Decimal {
        self.assets
            .iter()
            .map(|account| account.value())
            .sum::<Decimal>()
            - self
                .liabilities
                .iter()
                .map(|liability| match liability {
                    crate::liability::Liability::Mortgage { principal, .. } => *principal,
                    crate::liability::Liability::Loan { principal, .. } => *principal,
                })
                .sum::<Decimal>()
            + self.liquid_cash
    }
}

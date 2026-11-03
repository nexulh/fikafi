use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::asset::AssetOperations;
use crate::liability::Liability;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub assets: Vec<crate::asset::Asset>,
    pub liabilities: Vec<crate::liability::Liability>,
    pub liquid_cash: Decimal,
}

impl Portfolio {
    #[must_use]
    pub fn total_nominal_value(&self) -> Decimal {
        self.assets
            .iter()
            .map(AssetOperations::value)
            .sum::<Decimal>()
            - self
                .liabilities
                .iter()
                .map(|liability| match liability {
                    Liability::Mortgage { principal, .. } | Liability::Loan { principal, .. } => {
                        *principal
                    }
                })
                .sum::<Decimal>()
            + self.liquid_cash
    }
}

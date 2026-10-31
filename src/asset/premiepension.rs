use rust_decimal::Decimal;
use rust_decimal::dec;
use serde::{Deserialize, Serialize};

use crate::asset::AssetOperations;
use crate::context::MarketContext;
use crate::taxes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiepensionAccount {
    pub balance: Decimal,
}

impl AssetOperations for PremiepensionAccount {
    fn value(&self) -> Decimal {
        self.balance
    }

    fn deposit(&mut self, amount: Decimal) {
        self.balance += amount;
    }

    fn withdraw(&mut self, _amount: Decimal) -> Decimal {
        dec!(0)
    }

    fn tax_base(&self) -> Option<taxes::TaxBase> {
        None
    }

    fn step_month(&mut self, _market_context: &MarketContext) {
        // No-op for PremiepensionAccount
    }
}

use rust_decimal::Decimal;
use rust_decimal::dec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiepensionAccount {
    pub balance: Decimal,
}

impl crate::asset::AccountOperations for PremiepensionAccount {
    fn value(&self) -> Decimal {
        self.balance
    }

    fn deposit(&mut self, amount: Decimal) {
        self.balance += amount;
    }

    fn withdraw(&mut self, _amount: Decimal) -> Decimal {
        dec!(0)
    }

    fn step_month(&mut self) {
        // No-op for PremiepensionAccount
    }
}

use crate::asset::AccountOperations;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IskAccount {
    pub balance: Decimal,
}

impl IskAccount {
    pub fn new(balance: Decimal) -> Self {
        if balance < Decimal::ZERO {
            panic!("Balance cannot be negative");
        }
        Self { balance }
    }
}

impl AccountOperations for IskAccount {
    fn value(&self) -> Decimal {
        self.balance
    }

    fn deposit(&mut self, amount: Decimal) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: Decimal) -> Decimal {
        let withdrawn = std::cmp::min(self.balance, amount);
        self.balance -= withdrawn;
        withdrawn
    }

    fn step_month(&mut self) {
        // No-op for IskAccount
    }
}

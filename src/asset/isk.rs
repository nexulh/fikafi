use crate::asset::AssetOperations;
use rust_decimal::Decimal;
use rust_decimal::dec;
use serde::{Deserialize, Serialize};

use crate::context::MonthlyMarketContext;
use crate::taxes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IskAccount {
    pub balance: Decimal,
    annual_deposits: Decimal,
    quarter_values: [Decimal; 4],
}

impl IskAccount {
    /// Creates a new ISK account with the specified initial balance.
    /// # Panics
    /// Panics if the balance is strictly less than zero.
    #[must_use]
    pub fn new(balance: Decimal) -> Self {
        assert!(balance >= Decimal::ZERO, "Balance cannot be negative");
        Self {
            balance,
            annual_deposits: Decimal::ZERO,
            quarter_values: [balance; 4],
        }
    }
}

impl AssetOperations for IskAccount {
    fn value(&self) -> Decimal {
        self.balance
    }

    fn deposit(&mut self, amount: Decimal) {
        self.balance += amount;
        self.annual_deposits += amount;
    }

    fn withdraw(&mut self, amount: Decimal) -> Decimal {
        let withdrawn = if self.balance < amount {
            self.balance
        } else {
            amount
        };
        self.balance -= withdrawn;
        withdrawn
    }

    fn tax_base(&self) -> Option<taxes::TaxBase> {
        let isk_capital_base =
            self.quarter_values.iter().sum::<Decimal>() / dec!(4) + self.annual_deposits;
        Some(taxes::TaxBase {
            kind: taxes::TaxBaseType::IskCapitalBase,
            amount: isk_capital_base,
        })
    }

    fn step_month(&mut self, market_context: &MonthlyMarketContext) {
        self.balance *= dec!(1) + market_context.monthly_market_return;
        match market_context.current_year_month.month() {
            3 => self.quarter_values[0] = self.balance,
            6 => self.quarter_values[1] = self.balance,
            9 => self.quarter_values[2] = self.balance,
            12 => self.quarter_values[3] = self.balance,
            _ => (),
        }
    }
}

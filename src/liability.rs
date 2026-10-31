use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Liability {
    Mortgage {
        principal: Decimal,
        interest_rate: Decimal,
        term_months: u32,
    },
    Loan {
        principal: Decimal,
        interest_rate: Decimal,
        term_months: u32,
    },
}

pub trait LiabilityOperations {
    fn balance(&self) -> Decimal;
    fn monthly_obligation(&mut self) -> Decimal;
    fn pay(&mut self, amount: Decimal) -> Decimal;
    fn step_month(&mut self);
}

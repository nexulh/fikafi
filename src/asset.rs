use crate::taxes::TaxBase;
use enum_dispatch::enum_dispatch;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub mod inkomstpension;
pub mod isk;
pub mod premiepension;
pub mod tjanstepension;

use crate::context::MarketContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[enum_dispatch(AssetOperations)]
pub enum Asset {
    Isk(isk::IskAccount),
    //Depa(depa::DepaAccount),
    Tjanstepension(tjanstepension::TjanstepensionAccount),
    Inkomstpension(inkomstpension::InkomstpensionAccount),
    Premiepension(premiepension::PremiepensionAccount),
}

#[enum_dispatch]
pub trait AssetOperations {
    fn value(&self) -> Decimal;
    fn deposit(&mut self, amount: Decimal);
    fn withdraw(&mut self, amount: Decimal) -> Decimal;
    fn tax_base(&self) -> Option<TaxBase>;
    fn step_month(&mut self, market_context: &MarketContext);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PensionState {
    Accumulating,
    Annuitizing,
    Depleted,
}

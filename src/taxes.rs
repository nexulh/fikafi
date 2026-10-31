use rust_decimal::Decimal;

pub enum TaxBaseType {
    IskCapitalBase,
    RealizedCapitalGains,
    DeductibleInterest,
    TaxableIncome,
}

pub struct TaxBase {
    pub kind: TaxBaseType,
    pub amount: Decimal,
}

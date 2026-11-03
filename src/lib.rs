pub mod asset;
pub mod context;
pub mod household;
pub mod liability;
pub mod portfolio;
pub mod taxes;
pub mod utils;

pub use household::Household;
pub use household::Person;
pub use portfolio::Portfolio;
pub use utils::YearMonth;

pub type Prisbasbelopp = rust_decimal::Decimal;
pub type Inkomstbasbelopp = rust_decimal::Decimal;
pub type TaxRate = rust_decimal::Decimal;
pub type Statslaneranta = rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[must_use]
pub struct YearMonth {
    total_months: u32,
}

pub enum YearMonthError {
    InvalidYear(i32, u32),
    InvalidMonth(i32, u32),
}

impl std::fmt::Display for YearMonthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YearMonthError::InvalidMonth(year, month) => {
                write!(f, "Invalid month {month} for year {year}")
            }
            YearMonthError::InvalidYear(year, month) => {
                write!(f, "Invalid year {year} for month {month}")
            }
        }
    }
}

impl std::fmt::Display for YearMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}", self.year(), self.month())
    }
}

impl YearMonth {
    pub fn new(year: i32, month: u32) -> Result<Self, YearMonthError> {
        let valid_years = 1850..=9999;
        if !valid_years.contains(&year) {
            return Err(YearMonthError::InvalidYear(year, month));
        }

        let valid_months = 1..=12;
        if !valid_months.contains(&month) {
            return Err(YearMonthError::InvalidMonth(year, month));
        }

        let total_months = (year * 12).cast_unsigned() + (month - 1);
        Ok(Self { total_months })
    }

    #[must_use]
    pub fn year(&self) -> i32 {
        (self.total_months / 12).cast_signed()
    }

    #[must_use]
    pub fn month(&self) -> u32 {
        self.total_months % 12 + 1
    }

    #[must_use]
    pub fn months_since_epoch(&self) -> u32 {
        self.total_months
    }

    pub fn add_months(self, months: u32) -> Self {
        Self {
            total_months: self.total_months + months,
        }
    }

    pub fn inc_month(self) -> Self {
        self.add_months(1)
    }
}

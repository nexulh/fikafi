use crate::utils::YearMonth;

#[derive(Debug, Clone)]
pub enum Household {
    Single(Person),
    Couple(Person, Person),
    Family {
        parents: Vec<Person>,
        children: Vec<Person>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub born: YearMonth,
}

impl Person {
    pub fn new(name: impl Into<String>, born: YearMonth) -> Self {
        Self {
            name: name.into(),
            born,
        }
    }

    #[must_use]
    pub fn age_on(&self, yearmonth: YearMonth) -> u32 {
        (yearmonth.months_since_epoch() - self.born.months_since_epoch()) / 12
    }

    #[must_use]
    pub fn has_reached_age(&self, yearmonth: YearMonth, target_age: u32) -> bool {
        self.age_on(yearmonth) >= target_age
    }
}

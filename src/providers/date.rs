#[cfg(test)]
use unimock::unimock;

use chrono::Datelike;
use log::trace;

/// The months of the year, each one is one variant.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// A trait for providing the current date components such as year and month.
#[cfg_attr(test, unimock(api=DateInfoProviderMock))]
pub trait DateInfoProvider {
    /// Returns the current year.
    fn current_year(&self) -> u32;
    /// Returns the current month.
    fn current_month(&self) -> Month;
    /// Returns the current day
    fn current_day(&self) -> u32;
}

/// Implementation of the `DateInfoProvider` trait
pub struct DateAdapter {}

impl DateInfoProvider for DateAdapter {
    fn current_year(&self) -> u32 {
        chrono::Local::now().year() as u32
    }

    fn current_month(&self) -> Month {
        match chrono::Local::now().month() {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => unreachable!("Month should never get here"),
        }
    }

    fn current_day(&self) -> u32 {
        chrono::Local::now().day()
    }
}

impl Default for DateAdapter {
    fn default() -> Self {
        trace!("Instantiating new DateAdapter...");
        DateAdapter{}
    }
}

/// Returns the default implementation for the `DateInfoProvider`
pub fn default_date_provider() -> impl DateInfoProvider {
    DateAdapter::default()
}
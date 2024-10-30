//! This module provides the `DateService`, which offers utilities for calculating
//! default dates

use crate::providers::date::{CurrentDateProvider, DateAdapter, Month};
use log::trace;

/// `DateService` is a struct that provides functionalities to calculate default dates
/// such as default year and default day based on the current date.
///
/// The `DateService` is generic over a type `D` which must implement the
/// `CurrentDateProvider` trait.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct DateService<D>
where
    D: CurrentDateProvider,
{
    provider: D,
}

impl<D> DateService<D>
where
    D: CurrentDateProvider,
{
    /// Constructs a new `DateService` with the given date provider.
    pub fn new(provider: D) -> Self {
        DateService { provider }
    }

    /// It calculates the default year.
    /// It should be the current year on December; and the previous year otherwise.
    pub fn default_year(&self) -> u32 {
        trace!("Calculating default year...");

        let current_month = self.provider.current_month();
        let current_year = self.provider.current_year();

        match current_month {
            Month::December => current_year,
            _ => current_year - 1,
        }
    }

    /// It calculates the default day.
    /// It should be the current day if:
    /// - We are on December AND
    /// - We are between 1 and 25
    ///
    /// Otherwise, it will be 1
    pub fn default_day(&self) -> u32 {
        trace!("Calculating default day...");

        let current_month = self.provider.current_month();
        let current_day = self.provider.current_day();

        if current_month == Month::December && (1..=25).contains(&current_day) {
            current_day
        } else {
            1
        }
    }
}

impl Default for DateService<DateAdapter> {
    fn default() -> Self {
        Self::new(DateAdapter::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    struct DateInfoProviderMock {
        month: Month,
        year: u32,
        day: u32,
    }

    impl DateInfoProviderMock {
        fn new(year: u32, month: Month, day: u32) -> Self {
            Self { year, month, day }
        }
    }

    impl CurrentDateProvider for DateInfoProviderMock {
        fn current_year(&self) -> u32 {
            self.year
        }

        fn current_month(&self) -> Month {
            self.month
        }

        fn current_day(&self) -> u32 {
            self.day
        }
    }

    #[rstest]
    #[case(Month::January)]
    #[case(Month::February)]
    #[case(Month::March)]
    #[case(Month::April)]
    #[case(Month::May)]
    #[case(Month::June)]
    #[case(Month::July)]
    #[case(Month::August)]
    #[case(Month::September)]
    #[case(Month::October)]
    #[case(Month::November)]
    fn default_year_non_december(#[case] month: Month) {
        let provider_mock = DateInfoProviderMock::new(2024, month, 15);
        let service = DateService::new(provider_mock);
        let result = service.default_year();

        assert_eq!(result, 2023)
    }

    #[rstest]
    fn default_year_on_december() {
        let provider_mock =
            DateInfoProviderMock::new(2024, Month::December, 15);
        let service = DateService::new(provider_mock);
        let result = service.default_year();

        assert_eq!(result, 2024);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(6)]
    #[case(7)]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    #[case(11)]
    #[case(12)]
    #[case(13)]
    #[case(14)]
    #[case(15)]
    #[case(16)]
    #[case(17)]
    #[case(18)]
    #[case(19)]
    #[case(20)]
    #[case(21)]
    #[case(22)]
    #[case(23)]
    #[case(24)]
    #[case(25)]
    fn default_day_on_december_in_range(#[case] day: u32) {
        let provider_mock =
            DateInfoProviderMock::new(2024, Month::December, day);
        let service = DateService::new(provider_mock);
        let result = service.default_day();

        assert_eq!(result, day);
    }

    #[rstest]
    #[case(26)]
    #[case(27)]
    #[case(28)]
    #[case(29)]
    #[case(30)]
    #[case(31)]
    fn default_day_december_outside_range(#[case] day: u32) {
        let provider_mock =
            DateInfoProviderMock::new(2024, Month::December, day);
        let service = DateService::new(provider_mock);
        let result = service.default_day();

        assert_eq!(result, 1u32);
    }

    #[rstest]
    #[case(Month::January)]
    #[case(Month::February)]
    #[case(Month::March)]
    #[case(Month::April)]
    #[case(Month::May)]
    #[case(Month::June)]
    #[case(Month::July)]
    #[case(Month::August)]
    #[case(Month::September)]
    #[case(Month::October)]
    #[case(Month::November)]
    fn default_day_outside_december(#[case] month: Month) {
        let provider_mock = DateInfoProviderMock::new(2024, month, 10);
        let service = DateService::new(provider_mock);
        let result = service.default_day();

        assert_eq!(result, 1u32);
    }
}

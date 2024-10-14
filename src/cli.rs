//! This contains helper functions related to the CLI

use chrono;
use std::cmp::PartialEq;

#[cfg(test)]
use mockall::automock;
#[cfg(test)]
use rstest::*;

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
#[cfg_attr(test, automock)]
pub trait DateInfoProvider {
    /// Returns the current year.
    fn current_year(&self) -> u32;
    /// Returns the current month.
    fn current_month(&self) -> Month;
    /// Returns the current day
    fn current_day(&self) -> u32;
}

/// It calculates the default year.
/// It should be the current year on December; and the previous year otherwise.
pub fn default_year<T>(date_provider: &T) -> u32
where
    T: DateInfoProvider,
{
    let current_month = date_provider.current_month();
    let current_year = date_provider.current_year();

    match current_month {
        Month::December => current_year,
        _ => current_year - 1,
    }
}

/// It calculates the default day.
/// It should be the current day if:
/// - We are on December AND
/// - We are between 1 and 25
/// Otherwise, it will be 1
pub fn default_day<T>(provider: &T) -> u32
where
    T: DateInfoProvider,
{
    let current_month = provider.current_month();
    let current_day = provider.current_day();

    if current_month == Month::December && (1..=25).contains(&current_day) {
        current_day
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut mock = MockDateInfoProvider::new();

        mock.expect_current_month()
            .times(1)
            .returning(move || month);

        mock.expect_current_year().times(1).returning(|| 2024);

        let result = default_year(&mock);

        assert_eq!(result, 2023);
    }

    #[rstest]
    fn default_year_on_december() {
        let mut mock = MockDateInfoProvider::new();

        mock.expect_current_month()
            .times(1)
            .returning(move || Month::December);

        mock.expect_current_year().times(1).returning(|| 2024);

        let result = default_year(&mock);
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
        let mut mock = MockDateInfoProvider::new();

        mock.expect_current_day().times(1).returning(move || day);

        mock.expect_current_month()
            .times(1)
            .returning(move || Month::December);

        let result = default_day(&mock);
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
        let mut mock = MockDateInfoProvider::new();

        mock.expect_current_day().times(1).returning(move || day);

        mock.expect_current_month()
            .times(1)
            .returning(move || Month::December);

        let result = default_day(&mock);
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
        let mut mock = MockDateInfoProvider::new();

        mock.expect_current_day().times(1).returning(|| 10u32);

        mock.expect_current_month()
            .times(1)
            .returning(move || month);

        let result = default_day(&mock);
        assert_eq!(result, 1u32);
    }
}

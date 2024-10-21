//! This contains helper functions related to the CLI

use crate::providers::date::{CurrentDateProvider, Month};
use log::trace;

/// It calculates the default year.
/// It should be the current year on December; and the previous year otherwise.
pub fn default_year<D>(deps: &D) -> u32
where
    D: CurrentDateProvider,
{
    trace!("Calculating default year...");

    let current_month = deps.current_month();
    let current_year = deps.current_year();

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
pub fn default_day<D>(deps: &D) -> u32
where
    D: CurrentDateProvider,
{
    trace!("Calculating default day...");

    let current_month = deps.current_month();
    let current_day = deps.current_day();

    if current_month == Month::December && (1..=25).contains(&current_day) {
        current_day
    } else {
        1
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
        let mocked_deps = DateInfoProviderMock::new(2024, month, 15);
        let result = default_year(&mocked_deps);

        assert_eq!(result, 2023)
    }
    #[rstest]
    fn default_year_on_december() {
        let mocked_deps = DateInfoProviderMock::new(2024, Month::December, 15);
        let result = default_year(&mocked_deps);

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
        let mocked_deps = DateInfoProviderMock::new(2024, Month::December, day);
        let result = default_day(&mocked_deps);

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
        let mocked_deps = DateInfoProviderMock::new(2024, Month::December, day);
        let result = default_day(&mocked_deps);

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
        let mocked_deps = DateInfoProviderMock::new(2024, month, 10);
        let result = default_day(&mocked_deps);

        assert_eq!(result, 1u32);
    }
}

//! This contains helper functions related to the CLI

use super::providers::date::{DateInfoProvider, Month};
use log::trace;

/// It calculates the default year.
/// It should be the current year on December; and the previous year otherwise.
pub fn default_year<D>(deps: &D) -> u32
where
    D: DateInfoProvider,
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
    D: DateInfoProvider,
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
    use super::super::providers::date::DateInfoProviderMock;
    use super::*;
    use rstest::rstest;
    use unimock::{matching, MockFn, Unimock};

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
        let current_month_clause = DateInfoProviderMock::current_month
            .each_call(matching!())
            .returns(month)
            .once();

        let current_year_clause = DateInfoProviderMock::current_year
            .each_call(matching!())
            .returns(2024u32)
            .once();

        let mocked_deps =
            Unimock::new((current_month_clause, current_year_clause));
        let result = default_year(&mocked_deps);

        assert_eq!(result, 2023)
    }
    #[rstest]
    fn default_year_on_december() {
        let current_month_clause = DateInfoProviderMock::current_month
            .each_call(matching!())
            .returns(Month::December)
            .once();

        let current_year_clause = DateInfoProviderMock::current_year
            .each_call(matching!())
            .returns(2024u32)
            .once();

        let deps_mock =
            Unimock::new((current_month_clause, current_year_clause));
        let result = default_year(&deps_mock);

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
        let current_month_clause = DateInfoProviderMock::current_month
            .each_call(matching!())
            .returns(Month::December)
            .once();

        let current_day_clause = DateInfoProviderMock::current_day
            .each_call(matching!())
            .returns(day)
            .once();

        let deps_mock =
            Unimock::new((current_month_clause, current_day_clause));
        let result = default_day(&deps_mock);

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
        let current_month_clause = DateInfoProviderMock::current_month
            .each_call(matching!())
            .returns(Month::December)
            .once();

        let current_day_clause = DateInfoProviderMock::current_day
            .each_call(matching!())
            .returns(day)
            .once();

        let deps_mock =
            Unimock::new((current_month_clause, current_day_clause));
        let result = default_day(&deps_mock);

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
        let current_month_clause = DateInfoProviderMock::current_month
            .each_call(matching!())
            .returns(month)
            .once();

        let current_day_clause = DateInfoProviderMock::current_day
            .each_call(matching!())
            .returns(10u32)
            .once();

        let deps_mock =
            Unimock::new((current_month_clause, current_day_clause));
        let result = default_day(&deps_mock);

        assert_eq!(result, 1u32);
    }
}

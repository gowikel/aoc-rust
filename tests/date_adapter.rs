use aoc::providers::date;
use aoc::providers::date::{DateInfoProvider, Month};
use chrono::Datelike;

#[test]
fn test_current_year() {
    let current = chrono::Local::now().year() as u32;
    let adapter = date::default_date_provider();

    let result = adapter.current_year();

    assert_eq!(result, current);
}

#[test]
fn test_current_month() {
    let current = chrono::Local::now().month();
    let adapter = date::default_date_provider();

    let current = match current {
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
        _ => unreachable!("This should not be reachable"),
    };

    let result = adapter.current_month();

    assert_eq!(result, current);
}

#[test]
fn test_current_day() {
    let current = chrono::Local::now().day();
    let adapter = date::default_date_provider();
    let result = adapter.current_day();

    assert_eq!(result, current);
}

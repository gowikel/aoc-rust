use crate::{solvers::Solution, Puzzle};
use clap::ValueEnum;
use num_format::{Buffer, Locale};
use owo_colors::OwoColorize;
use tabled::{
    builder::Builder,
    settings::{
        object::Rows, style::BorderSpanCorrection, Alignment, Settings, Span,
        Style,
    },
};

/// Controls whatever to use a table format or a simple text format
#[derive(PartialEq, Eq, Clone, Copy, Debug, ValueEnum, Default)]
pub enum StyleFormat {
    #[default]
    Simple,
    Tabulated,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub struct OutputFormat {
    style: StyleFormat,
    color: bool,
}

/// Creates a new formatter builder
pub fn new() -> OutputFormat {
    OutputFormat::default()
}

impl OutputFormat {
    pub fn set_color(&mut self, color: bool) -> &mut Self {
        self.color = color;

        self
    }

    pub fn set_style(&mut self, style: StyleFormat) -> &mut Self {
        self.style = style;

        self
    }

    /// Formats the given solutions for the given puzzle input
    pub fn format(&self, puzzle: Puzzle, solutions: &[Solution; 2]) -> String {
        match self.style {
            StyleFormat::Simple => {
                print_simple_results(puzzle, solutions, self.color)
            }
            StyleFormat::Tabulated => {
                print_tabulated_results(puzzle, solutions, self.color)
            }
        }
    }
}

/// Helper that gets the puzzle input and the solutions and prints them
/// one line after the other. The with_color paramter controls if certains
/// parts of the string will use ASCI.
fn print_simple_results(
    puzzle: Puzzle,
    solutions: &[Solution; 2],
    with_color: bool,
) -> String {
    let mut error_string = String::from("Error:");
    if with_color {
        error_string = error_string.red().bold().to_string();
    }
    let error_string = error_string;

    let p1_solution = match &solutions[0] {
        Solution::NotExecuted => "Not executed".to_string(),
        Solution::NotImplemented => "Not implemented".to_string(),
        Solution::Err(err) => format!("{} {}", error_string, err).to_string(),
        Solution::Value(x) => match with_color {
            true => x.to_string().green().to_string(),
            false => x.to_string(),
        },
    };
    let p2_solution = match &solutions[1] {
        Solution::NotExecuted => "Not executed".to_string(),
        Solution::NotImplemented => "Not implemented".to_string(),
        Solution::Err(err) => format!("{} {}", error_string, err).to_string(),
        Solution::Value(x) => match with_color {
            true => x.to_string().green().to_string(),
            false => x.to_string(),
        },
    };

    [
        format!("Solutions {}/{:02}", puzzle.year(), puzzle.day()),
        format!("{}\n{}", p1_solution, p2_solution),
    ]
    .join("\n")
}

/// Helper function that prints the given solutions in a table. The with_color property
/// controls if colors will be added to the table or not.
fn print_tabulated_results(
    puzzle: Puzzle,
    solutions: &[Solution; 2],
    with_color: bool,
) -> String {
    let mut builder: Builder = Builder::default();
    let mut error_string = String::from("Error:");
    if with_color {
        error_string = error_string.red().bold().to_string();
    }
    let error_string = error_string;

    let mut table_header =
        format!("Solutions {}/{:02}", puzzle.year(), puzzle.day());
    if with_color {
        table_header = table_header.bold().to_string();
    }
    let table_header = table_header;

    builder.push_record(vec![table_header]);

    for (index, solution) in solutions.iter().enumerate() {
        let mut header = format!("P{}", index + 1);
        if with_color {
            header = header.bold().to_string();
        }
        let header = header;

        let solution = match solution {
            Solution::NotExecuted => "Not executed".to_string(),
            Solution::NotImplemented => "Not implemented".to_string(),
            Solution::Err(err) => {
                format!("{} {}", error_string, err).to_string()
            }
            Solution::Value(x) => {
                let mut buf = Buffer::default();
                buf.write_formatted(x, &Locale::en);

                match with_color {
                    true => buf.as_str().green().to_string(),
                    false => buf.to_string(),
                }
            }
        };

        builder.push_record(vec![header, solution]);
    }

    let mut table = builder.build();
    table
        .with(Style::rounded())
        .modify(Rows::first(), Span::column(2))
        .modify(
            Rows::first(),
            Settings::new(Alignment::center(), Alignment::center()),
        )
        .with(BorderSpanCorrection);

    table.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Puzzle;
    use std::sync::LazyLock;

    static PUZZLE: LazyLock<Puzzle> =
        LazyLock::new(|| Puzzle::new(2024, 1).unwrap());

    static NOT_IMPLEMENTED_SOLUTIONS: LazyLock<[Solution; 2]> =
        LazyLock::new(|| [Solution::NotImplemented, Solution::NotImplemented]);

    static NOT_EXECUTED_SOLUTIONS: LazyLock<[Solution; 2]> =
        LazyLock::new(|| [Solution::NotExecuted, Solution::NotExecuted]);

    static ERROR_SOLUTIONS: LazyLock<[Solution; 2]> = LazyLock::new(|| {
        [
            Solution::Err(String::from("ERR 1")),
            Solution::Err(String::from("ERR 2")),
        ]
    });

    static VALUE_SOLUTIONS: LazyLock<[Solution; 2]> =
        LazyLock::new(|| [Solution::Value(12), Solution::Value(24)]);

    mod print_simple_results {
        use super::*;

        #[test]
        fn test_no_implemented_solutions() {
            let expected =
                ["Solutions 2024/01", "Not implemented", "Not implemented"]
                    .join("\n");
            let result = print_simple_results(
                *PUZZLE,
                &NOT_IMPLEMENTED_SOLUTIONS,
                false,
            );

            assert_eq!(expected, result);
        }

        #[test]
        fn test_no_executed_solutions() {
            let expected =
                ["Solutions 2024/01", "Not executed", "Not executed"]
                    .join("\n");
            let result =
                print_simple_results(*PUZZLE, &NOT_EXECUTED_SOLUTIONS, false);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_err_solutions() {
            let expected =
                ["Solutions 2024/01", "Error: ERR 1", "Error: ERR 2"]
                    .join("\n");
            let result = print_simple_results(*PUZZLE, &ERROR_SOLUTIONS, false);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_err_solutions_with_color_variant() {
            let expected = [
                "Solutions 2024/01",
                // ANSI bold and red, wrapping the error string
                // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
                "\u{1b}[1m\u{1b}[31mError:\u{1b}[39m\u{1b}[0m ERR 1",
                "\u{1b}[1m\u{1b}[31mError:\u{1b}[39m\u{1b}[0m ERR 2",
            ]
            .join("\n");
            let result = print_simple_results(*PUZZLE, &ERROR_SOLUTIONS, true);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_value_solutions() {
            let expected = ["Solutions 2024/01", "12", "24"].join("\n");
            let result = print_simple_results(*PUZZLE, &VALUE_SOLUTIONS, false);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_value_solutions_with_color_variant() {
            let expected = [
                "Solutions 2024/01",
                // ANSI bold and green
                // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
                "\u{1b}[32m12\u{1b}[39m",
                "\u{1b}[32m24\u{1b}[39m",
            ]
            .join("\n");
            let result = print_simple_results(*PUZZLE, &VALUE_SOLUTIONS, true);

            assert_eq!(expected, result);
        }
    }

    mod print_tabulated_results {
        use super::*;

        #[test]
        fn test_no_implemented_solutions() {
            let expected = [
                "╭──────────────────────╮",
                "│  Solutions 2024/01   │",
                "├────┬─────────────────┤",
                "│ P1 │ Not implemented │",
                "│ P2 │ Not implemented │",
                "╰────┴─────────────────╯",
            ]
            .join("\n");
            let result = print_tabulated_results(
                *PUZZLE,
                &NOT_IMPLEMENTED_SOLUTIONS,
                false,
            );

            assert_eq!(expected, result);
        }

        #[test]
        fn test_no_implemented_solutions_with_color_variant() {
            // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
            let expected = [
                "╭──────────────────────╮",
                "│  \u{1b}[1mSolutions 2024/01\u{1b}[0m   │",
                "├────┬─────────────────┤",
                "│ \u{1b}[1mP1\u{1b}[0m │ Not implemented │",
                "│ \u{1b}[1mP2\u{1b}[0m │ Not implemented │",
                "╰────┴─────────────────╯",
            ]
            .join("\n");
            let result = print_tabulated_results(
                *PUZZLE,
                &NOT_IMPLEMENTED_SOLUTIONS,
                true,
            );

            assert_eq!(expected, result);
        }

        #[test]
        fn test_no_executed_solutions() {
            let expected = [
                "╭───────────────────╮",
                "│ Solutions 2024/01 │",
                "├────┬──────────────┤",
                "│ P1 │ Not executed │",
                "│ P2 │ Not executed │",
                "╰────┴──────────────╯",
            ]
            .join("\n");
            let result = print_tabulated_results(
                *PUZZLE,
                &NOT_EXECUTED_SOLUTIONS,
                false,
            );

            assert_eq!(expected, result);
        }

        #[test]
        fn test_no_executed_solutions_with_color_variant() {
            // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
            let expected = [
                "╭───────────────────╮",
                "│ \u{1b}[1mSolutions 2024/01\u{1b}[0m │",
                "├────┬──────────────┤",
                "│ \u{1b}[1mP1\u{1b}[0m │ Not executed │",
                "│ \u{1b}[1mP2\u{1b}[0m │ Not executed │",
                "╰────┴──────────────╯",
            ]
            .join("\n");
            let result =
                print_tabulated_results(*PUZZLE, &NOT_EXECUTED_SOLUTIONS, true);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_errors_solutions() {
            let expected = [
                "╭───────────────────╮",
                "│ Solutions 2024/01 │",
                "├────┬──────────────┤",
                "│ P1 │ Error: ERR 1 │",
                "│ P2 │ Error: ERR 2 │",
                "╰────┴──────────────╯",
            ]
            .join("\n");
            let result =
                print_tabulated_results(*PUZZLE, &ERROR_SOLUTIONS, false);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_errors_solutions_with_color_variant() {
            let expected = [
                "╭───────────────────╮",
                "│ \u{1b}[1mSolutions 2024/01\u{1b}[0m │",
                "├────┬──────────────┤",
                "│ \u{1b}[1mP1\u{1b}[0m │ \u{1b}[1m\u{1b}[31mError:\u{1b}[39m\u{1b}[0m ERR 1 │",
                "│ \u{1b}[1mP2\u{1b}[0m │ \u{1b}[1m\u{1b}[31mError:\u{1b}[39m\u{1b}[0m ERR 2 │",
                "╰────┴──────────────╯",
            ]
            .join("\n");
            let result =
                print_tabulated_results(*PUZZLE, &ERROR_SOLUTIONS, true);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_value_solutions() {
            let expected = [
                "╭───────────────────╮",
                "│ Solutions 2024/01 │",
                "├─────────┬─────────┤",
                "│ P1      │ 12      │",
                "│ P2      │ 24      │",
                "╰─────────┴─────────╯",
            ]
            .join("\n");
            let result =
                print_tabulated_results(*PUZZLE, &VALUE_SOLUTIONS, false);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_value_solutions_with_color_variant() {
            let expected = [
                "╭───────────────────╮",
                "│ \u{1b}[1mSolutions 2024/01\u{1b}[0m │",
                "├─────────┬─────────┤",
                "│ \u{1b}[1mP1\u{1b}[0m      │ \u{1b}[32m12\u{1b}[39m      │",
                "│ \u{1b}[1mP2\u{1b}[0m      │ \u{1b}[32m24\u{1b}[39m      │",
                "╰─────────┴─────────╯",
            ]
            .join("\n");
            let result =
                print_tabulated_results(*PUZZLE, &VALUE_SOLUTIONS, true);

            assert_eq!(expected, result);
        }
    }

    mod output_format {
        use crate::formatter::{
            new, print_simple_results, print_tabulated_results, OutputFormat,
            StyleFormat,
        };

        use super::{PUZZLE, VALUE_SOLUTIONS};

        #[test]
        fn test_new_equals_default() {
            let new_value = new();
            let default_value = OutputFormat::default();

            assert_eq!(new_value, default_value);
        }

        #[test]
        fn test_set_color_builder() {
            let mut builder1 = OutputFormat::default();

            assert_eq!(builder1.color, false);

            let &mut builder2 = builder1.set_color(true);
            assert_eq!(builder1.color, true);
            assert_eq!(builder1, builder2);
        }

        #[test]
        fn test_set_style_builder() {
            let mut builder1 = OutputFormat::default();

            assert_eq!(builder1.style, StyleFormat::Simple);

            let &mut builder2 = builder1.set_style(StyleFormat::Tabulated);
            assert_eq!(builder1.style, StyleFormat::Tabulated);
            assert_eq!(builder1, builder2);
        }

        #[test]
        fn test_simple_format() {
            let expected =
                print_simple_results(*PUZZLE, &VALUE_SOLUTIONS, false);
            let result = OutputFormat::default()
                .set_color(false)
                .set_style(StyleFormat::Simple)
                .format(*PUZZLE, &VALUE_SOLUTIONS);

            assert_eq!(expected, result);
        }

        #[test]
        fn test_tabulated_format() {
            let expected =
                print_tabulated_results(*PUZZLE, &VALUE_SOLUTIONS, false);
            let result = OutputFormat::default()
                .set_color(false)
                .set_style(StyleFormat::Tabulated)
                .format(*PUZZLE, &VALUE_SOLUTIONS);

            assert_eq!(expected, result);
        }
    }
}

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

#[derive(Default)]
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
/// parts of the string will use ASCI
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

    let mut table_header = format!("Solutions {}/{:02}", puzzle.year(), puzzle.day());
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
                  false => buf.to_string()
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

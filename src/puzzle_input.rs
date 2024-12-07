use std::fmt::Display;

use crate::report::Report;

#[derive(Debug)]
pub struct PuzzleInput(Vec<Report>);

impl Display for PuzzleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for item in &self.0 {
            writeln!(f, "  {}", item)?;
        }
        writeln!(f, "]")?;

        Ok(())
    }
}

impl From::<&str> for PuzzleInput {
    fn from(value: &str) -> Self {
        let reports: Result<Vec<Report>, _> = value
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|token| {
                        token.parse::<i32>()
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .and_then(|reports| {
                Ok(reports
                    .into_iter()
                    .map(|levels| {
                        Report(levels)
                    })
                    .collect::<Vec<_>>()
                )
            });

    
        // TODO
        let reports = reports.unwrap();
        Self(reports)
    }
}

impl PuzzleInput {
    pub fn reports(&self) -> &Vec<Report> {
        &self.0
    }
}

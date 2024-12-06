use crate::report::Report;

pub struct PuzzleInput(Vec<Report>);

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

use std::fmt::Display;

#[derive(Debug)]
pub struct Report(pub Vec<i32>);

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..&self.0.len() - 1 {
            write!(f, "{}, ", self.0[i])?;
        }
        write!(f, "{}]", self.0[self.0.len() - 1])?;

        Ok(())
    }
}

impl Report {
    pub fn levels(&self) -> &Vec<i32> {
        &self.0
    }
}

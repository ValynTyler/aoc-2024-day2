use std::{cmp::Ordering, fmt::Display};

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

    pub fn is_safe(&self) -> bool {
        let levels = self.levels();

        let mut last = levels[0];
        let mut this = levels[1];

        let order = this.cmp(&last);
        
        if order == Ordering::Equal { return false };
        if (this - last).abs() > 3 { return false };

        for i in 2..levels.len() {
            last = this;
            this = levels[i];

            if (this - last).abs() > 3 { return false };
            if this.cmp(&last) != order { return false };
        }

        true
    }
}

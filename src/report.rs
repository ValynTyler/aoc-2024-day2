pub struct Report(Vec<i32>);

impl Report {
    pub fn levels(&self) -> &Vec<i32> {
        &self.0
    }
}

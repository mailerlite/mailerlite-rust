#[derive(Debug, Clone)]
pub struct Parameter {
    pub data: Vec<(&'static str, &'static str)>,
}

impl Parameter {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(mut self, key: &'static str, value: &'static str) -> Self {
        self.data.push((key, value));

        self
    }
}

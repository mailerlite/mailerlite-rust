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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_new() {
        let parameter: Parameter = Parameter::new();

        assert_eq!(parameter.data, Vec::new());
    }

    #[test]
    fn parameter_add() {
        let parameter: Parameter = Parameter::new().add("key", "value");

        assert_eq!(parameter.data, vec![("key", "value")]);
    }
}

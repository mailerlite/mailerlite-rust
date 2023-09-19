#[derive(Debug, Clone)]
pub struct Data {
    pub data: Vec<(&'static str, &'static str)>,
}

impl Data {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(mut self, key: &'static str, value: &'static str) -> Self {
        self.data.push((key, value));

        self
    }

    pub fn get(&self, key: &'static str) -> Option<&'static str> {
        for (k, v) in &self.data {
            if k == &key {
                return Some(v);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_new() {
        let data: Data = Data::new();

        assert_eq!(data.data, Vec::new());
    }

    #[test]
    fn data_add() {
        let data: Data = Data::new().add("key", "value");

        assert_eq!(data.data, vec![("key", "value")]);
    }

    #[test]
    fn data_get() {
        let data: Data = Data::new().add("key", "value");

        assert_eq!(data.get("key"), Some("value"));
        assert_eq!(data.get("key2"), None);
    }
}

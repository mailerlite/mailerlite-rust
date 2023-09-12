#[derive(Debug, Clone)]
pub struct Form {
    pub data: Vec<(&'static str, &'static str)>,
}

impl Form {
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
    fn form_new() {
        let form: Form = Form::new();

        assert_eq!(form.data, Vec::new());
    }

    #[test]
    fn form_add() {
        let form: Form = Form::new().add("key", "value");

        assert_eq!(form.data, vec![("key", "value")]);
    }

    #[test]
    fn form_get() {
        let form: Form = Form::new().add("key", "value");

        assert_eq!(form.get("key"), Some("value"));
        assert_eq!(form.get("key2"), None);
    }
}

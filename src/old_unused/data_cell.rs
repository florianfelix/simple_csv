#[derive(Default, Debug, Clone)]
pub struct DataCell {
    column_name: String,
    value: String,
}

impl std::fmt::Display for DataCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl DataCell {
    pub fn from_tuple(input: (String, String)) -> Self {
        DataCell {
            column_name: input.0,
            value: input.1,
        }
    }
}

impl DataCell {
    pub fn set(&mut self, value: &str) {
        self.value = value.to_string();
    }
    pub fn as_item(&self) -> (String, String) {
        (self.column_name.clone(), self.value.clone())
    }
}

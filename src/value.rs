pub struct ValueArray {
    pub values: Vec<f64>
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray {
            values: Vec::new()
        }
    }

    pub fn write_value_array(&mut self, value: f64) {
        self.values.push(value);
    }
}
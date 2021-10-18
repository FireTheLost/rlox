pub enum Value {
    Number(f64)
}

pub struct ValueArray {
    pub values: Vec<Value>
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray {
            values: Vec::new()
        }
    }

    pub fn write_value_array(&mut self, value: f64) {
        self.values.push(Value::Number(value));
    }
}
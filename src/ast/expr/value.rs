
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
}

impl Value {
    pub fn negate(self) -> Self {
        use Value::*;
        match self {
            Int(val) => Int(-val),
            Float(val) => Float(-val),
        }
    }
} 

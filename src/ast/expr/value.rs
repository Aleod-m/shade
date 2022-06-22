#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Unit(()),
    List(Vec<Value>),
}

impl Value {
    pub fn new(val: impl Into<Self>) -> Self {
        val.into()
    }

    pub fn negate(&self) -> Self {
        use Value::*;
        match self {
            Int(val) => Int(-val),
            Float(val) => Float(-val),
            _ => unreachable!(),
        }
    }

    pub fn modulo(&self, other: Value) -> Value {
        use Value::*;
        match (self, other) {
            (Int(val), Int(other)) => Int(val % other),
            _ => unreachable!(),
        }
    }

    pub fn add(&self, other: Self) -> Self {
        use Value::*;
        match (self, other) {
            (Int(val), Int(other)) => Int(val + other),
            (Float(val), Float(other)) => Float(val + other),
            _ => unreachable!(),
        }
    }

    pub fn sub(&self, other: Self) -> Self {
        use Value::*;
        match (self, other) {
            (Int(val), Int(other)) => Int(val - other),
            (Float(val), Float(other)) => Float(val - other),
            _ => unreachable!(),
        }
    }

    pub fn mul(&self, other: Value) -> Value {
        use Value::*;
        match (self, other) {
            (Int(val), Int(other)) => Int(val * other),
            (Float(val), Float(other)) => Float(val * other),
            _ => unreachable!(),
        }
    }

    pub fn div(&self, other: Value) -> Value {
        use Value::*;
        match (self, other) {
            (Int(val), Int(other)) => Int(val / other),
            (Float(val), Float(other)) => Float(val / other),
            _ => unreachable!(),
        }
    }

    pub(crate) fn eval(&self) -> Value {
        use Value::*;
        match self {
            Int(v) => Int(*v),
            Float(v) => Float(*v),
            Unit(_) => Unit(()),
            List(v) => List(v.to_vec()),
        }
    }
}

macro_rules! into_value {
    ($t:ty, $var: ident) => {
        impl Into<Value> for $t {
            fn into(self) -> Value {
                Value::$var(self)
            }
        }
    };
}
into_value!(i32, Int);
into_value!(f32, Float);
into_value!((), Unit);
into_value!(Vec<Value>, List);

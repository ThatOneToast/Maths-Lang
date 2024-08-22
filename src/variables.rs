use std::collections::HashMap;

use crate::expressions::{Expression, ExpressionContainer};


pub struct Variables {
    pub numbers: HashMap<String, <Expression as ExpressionContainer>::Number>
}

pub trait VariableContainer {
    type Numbers;
}

impl VariableContainer for Variables {
    type Numbers = HashMap<String, <Expression as ExpressionContainer>::Number>;
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            numbers: HashMap::new()
        }
    }
}
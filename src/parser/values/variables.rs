use std::collections::HashMap;

use super::expressions::Expression;



pub struct Variables {
    pub string_vars: HashMap<String, String>, // Store strings directly
    pub expr_vars: HashMap<String, Expression>,
}

impl Variables {
    pub fn find(&self, name: &str) -> Option<&Expression> {
        if let Some(expr) = self.expr_vars.get(name) {Some(expr)} else {None}
    }
}
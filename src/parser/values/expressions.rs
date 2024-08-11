

#[derive(Debug, Clone)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    
    Power(Box<Expression>, Box<Expression>),
    
    Number(f64),
    Boolean(bool),
    Variable(String), // Store variable name as a String
    
    LessThan(Box<Expression>, Box<Expression>),
    MoreThan(Box<Expression>, Box<Expression>),
    
}



impl Expression {
    pub fn is_number(&self) -> bool {
        matches!(self, Expression::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Expression::Variable(var) if var.starts_with("\"") && var.ends_with("\""))
    }

    pub fn as_number(&self) -> Option<f64> {
        if let Expression::Number(num) = self {
            Some(*num)
        } else {
            None
        }
    }
    
    pub fn as_boolean(&self) -> Option<bool> {
        if let Expression::Boolean(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let Expression::Variable(var) = self {
            if var.starts_with("\"") && var.ends_with("\"") {
                Some(var[1..var.len() - 1].to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_variable(&self) -> bool {
        matches!(self, Expression::Variable(_))
    }

    pub fn variable_name(&self) -> Option<&String> {
        if let Expression::Variable(var) = self {
            Some(var)
        } else {
            None
        }
    }

    pub fn is_math(&self) -> bool {
        matches!(
            self,
            Expression::Add(_, _)
                | Expression::Subtract(_, _)
                | Expression::Multiply(_, _)
                | Expression::Divide(_, _)
                | Expression::Power(_, _)
        )
    }
    
    pub fn is_power(&self) -> bool {
        matches!(self, Expression::Power(_, _))
    }
    
    

    pub fn is_less_than(&self) -> bool {
        matches!(self, Expression::LessThan(_, _))
    }
    
    pub fn is_more_than(&self) -> bool {
        matches!(self, Expression::MoreThan(_, _))
    }
    

}
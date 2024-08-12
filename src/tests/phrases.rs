
#[cfg(test)]
mod phrases {
    use crate::parser::{self, math_expr::evaluate_expression, values::expressions::Expression};


    fn mini_interpreter(input: &str) -> Result<(Box<Expression>), String> {
        let mut variables = parser::parser::parse_expression_file(input);
        let calculated_eval = parser::parser::calculate_sequence(&mut variables);
        
        Ok(calculated_eval)
    }
    
    #[test]
    fn if_phrase() {
        let maths = r"
            let result = 50
            
            
            ??? (result > 30) {
                let result = 1
            } 
            
            
            
            ;result
            ";

        let result = mini_interpreter(maths).unwrap().as_number()
            .expect("Failed converting final result to a number");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn if_else_phrase() {
        let maths = r"
            
            ??? (5 < 2) { 
                let result = 10 
            } !!! { 
                let result = 20
            }
            
            ;result
            ";

        let result = mini_interpreter(maths).unwrap().as_number()
            .expect("Failed converting final result to a number");
        assert_eq!(result, 20.0)
    }
    
    #[test]
    fn equals_condition() {
        let maths = format!("
            let result = 0
            
            let num = 5
            ??? (num == 5) {{
                let result = 5
            }} !!! {{
                let result = 10
            }}
            
            ;result
        ");
        
        let result = mini_interpreter(&maths).unwrap().as_number()
            .expect("Failed converting final result to a number");
        assert_eq!(result, 5.0)
    }
}

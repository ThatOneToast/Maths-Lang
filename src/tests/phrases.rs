
#[cfg(test)]
mod phrases {

    #[test]
    fn if_phrase() {
        let maths = r"
            let result = 50
            
            
            ??? (result > 30) {
                let result = 1
            } 
            
            
            
            ;result
            ";
        
        let mut sequence = crate::parser::parser::parse_expression_file(maths);
        let calculation = crate::parser::parser::calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 1.0);
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
        
        let mut sequence = crate::parser::parser::parse_expression_file(maths);
        let calculation = crate::parser::parser::calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 20.0);
    }
    


}
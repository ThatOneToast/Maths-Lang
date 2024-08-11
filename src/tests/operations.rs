


#[cfg(test)]
mod operations {
    use crate::parser::parser::{calculate_sequence, parse_expression_file};

    
    #[test]
    fn add() {
        let maths = r"
            let addition = 50 + 50
            
            ;addition
            ";
        
        let mut sequence = parse_expression_file(maths);
        let calculation = calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 100.0);
    }
    
    #[test]
    fn subtract() {
        let maths = r"
            let subtraction = 50 - 50
            
            ;subtraction
            ";
        
        let mut sequence = parse_expression_file(maths);
        let calculation = calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 0.0);
    }
    
    #[test]
    fn multiply() {
        let maths = r"
            let multiplication = 50 * 50
            
            ;multiplication
            ";
        
        let mut sequence = parse_expression_file(maths);
        let calculation = calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 2500.0);
    }
    
    #[test]
    fn divide() {
        let maths = r"
            let division = 50 / 50
            
            ;division
            ";
        
        let mut sequence = parse_expression_file(maths);
        let calculation = calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 1.0);
    }
    
    #[test]
    fn power() {
        let maths = r"
            let power = 5 ^ 2
            
            ;power
            ";
        
        let mut sequence = parse_expression_file(maths);
        let calculation = calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 25.0);
    }
    
    #[test]
    fn less_than() {
        let maths = r"
            let less_than = 5 < 2
            
            ;less_than
            ";
        
        let mut sequence = parse_expression_file(maths);
        let calculation = calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 0.0);
    }
    
    #[test]
    fn more_than() {
        let maths = r"
            let more_than = 5 > 2
            
            ;more_than
            ";
        
        let mut sequence = parse_expression_file(maths);
        let calculation = calculate_sequence(&mut sequence);
        assert_eq!(calculation.as_number().unwrap(), 1.0);
    }

}


#[cfg(test)]
pub mod tests {
    
    #[test]
    fn full_test() {
        let test_file = r#"
            let addition1 = 50 + 3
            let addition2 = 50 + 3
            let subtraction1 = 50 - 3
            let subtraction2 = 50 - 3
            let multiplication1 = 50 * 3
            let multiplication2 = 50 * 3
            let division1 = 50 / 3
            let division2 = 50 / 3
            
            let some_final_result = addition1 + addition2 + subtraction1 + subtraction2 + multiplication1 + multiplication2 + division1 + division2
            
            let powered = 5 ^ 2
            let some_final_result = some_final_result + powered
            ;some_final_result
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file);
        test_file_parser.parse();
        
        assert_eq!(test_file_parser.var_container.get_number("some_final_result").unwrap().round(), 558.0);
    }
    
    #[test]
    fn addition() {
        let test_file = r#"
            let result = 50 + 3
            
            ;result
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file);
        test_file_parser.parse();
        
        assert_eq!(test_file_parser.var_container.get_number("result").unwrap(), &53.0);
    }
    
    #[test]
    fn subtraction() {
        let test_file = r#"
            let result = 50 - 3
            
            ;result
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file);
        test_file_parser.parse();
        
        assert_eq!(test_file_parser.var_container.get_number("result").unwrap(), &47.0);
    }
    
    #[test]
    fn multiplication() {
        let test_file = r#"
            let result = 50 * 3
            
            ;result
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file);
        test_file_parser.parse();
        
        assert_eq!(test_file_parser.var_container.get_number("result").unwrap(), &150.0);
    }
    
    #[test]
    fn division() {
        let test_file = r#"
            let result = 60 / 3
            
            ;result
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file);
        test_file_parser.parse();
        
        assert_eq!(test_file_parser.var_container.get_number("result").unwrap(), &20.0);
    }
    
    #[test]
    fn power() {
        let test_file = r#"
            let result = 5 ^ 2
            
            ;result
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file);
        test_file_parser.parse();
        
        assert_eq!(test_file_parser.var_container.get_number("result").unwrap(), &25.0);
    }
    

    
}
#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;


    #[test]
    fn addition() {
        let test_file = r#"
            let result = 50 + 3

            ;result
            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file, None);
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &53.0
        );
    }

    #[test]
    fn subtraction() {
        let test_file = r#"
            let result = 50 - 3

            ;result
            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file, None);
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &47.0
        );
    }

    #[test]
    fn multiplication() {
        let test_file = r#"
            let result = 50 * 3

            ;result
            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file, None);
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &150.0
        );
    }

    #[test]
    fn division() {
        let test_file = r#"
            let result = 60 / 3

            ;result
            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file, None);
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &20.0
        );
    }

    #[test]
    fn power() {
        let test_file = r#"
            let result = 5 ^ 2

            ;result
            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file, None);
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &25.0
        );
    }

    #[test]
    fn if_statement() {
        let test_file = r#"

            if 50 > 30 {
                let result = 1
            } else {
                let result = 0
            }

            ;result
            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file.trim(), None);
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &1.0
        );
    }

    #[test]
    fn ifelse_statement() {
        let test_file = r#"

            let result = 60 + 3

            if result < 60 {
                let result = 1
            } else {
                let result = 0
            }

            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file.trim(), None);
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &0.0
        );
    }
    
    #[test]
    fn context_parameter() {
        let test_file = r#"
#[height, width, length]
let result = height * width * length

;result
"#;

        let mut test_file_parser = crate::parser::Parser::new(test_file.trim(), None);
        
        let params = test_file_parser.get_paramater_names().unwrap();
        
        for param in params {
            test_file_parser.var_container.numbers.insert(param.to_string(), 10.0);
        }
        
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &1000.0
        );
    }
    
    #[test]
    fn function() {
        let test_file = r#"
let height = 10
let width = 10
let length = 10
            
let result = @Volume(height,width,length)
            
;result
            "#;
        
        let patterns = HashMap::from([
            ("Volume".to_string(), r#"#[height, width, length]
                let result = height * width * length
                "#.to_string())
        ]);

        let mut test_file_parser = crate::parser::Parser::new(test_file, Option::from(patterns));
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &1000.0
        );
    }
    

    #[test]
    fn standard_lib_sqrt() {
        let test_file = r#"
let result = @SqrRt(16)
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file, Option::from(HashMap::new()));
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &4.0
        );
    }
    
    #[test]
    fn standard_lib_quadratic() {
        let test_file = r#"
let result = @Quadratic(1,2,3,4)

;result
            "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file, Option::from(HashMap::new()));
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &27.0
        );
    }
    
    
    #[test]
    fn loop_statement() {
        let test_file = r#"
        
let num = 0

loop 10 
    let num = num + 1
    
    ;num
loop_end
    "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file, Option::from(HashMap::new()));
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("num").unwrap(),
            &10.0
        );
    }
    
    #[test]
    fn loop_break_statement() {
        let test_file = r#"
        
let num = 0

loop 10 

if num == 5 {
    break
}

let num = num + 1

;num

loop_end
        "#;
        
        let mut test_file_parser = crate::parser::Parser::new(test_file, Option::from(HashMap::new()));
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("num").unwrap(),
            &5.0
        );
    }
        
}

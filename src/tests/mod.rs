#[cfg(test)]
pub mod tests {

    #[test]
    fn addition() {
        let test_file = r#"
            let result = 50 + 3

            ;result
            "#;

        let mut test_file_parser = crate::parser::Parser::new(test_file);
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

        let mut test_file_parser = crate::parser::Parser::new(test_file);
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

        let mut test_file_parser = crate::parser::Parser::new(test_file);
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

        let mut test_file_parser = crate::parser::Parser::new(test_file);
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

        let mut test_file_parser = crate::parser::Parser::new(test_file);
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

        let mut test_file_parser = crate::parser::Parser::new(test_file.trim());
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

        let mut test_file_parser = crate::parser::Parser::new(test_file.trim());
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

        let mut test_file_parser = crate::parser::Parser::new(test_file.trim());
        
        let params = test_file_parser.get_paramater_names();
        
        for param in params {
            test_file_parser.var_container.numbers.insert(param.to_string(), 10.0);
        }
        
        test_file_parser.parse();

        assert_eq!(
            test_file_parser.var_container.get_number("result").unwrap(),
            &1000.0
        );
    }
}

use std::{collections::HashMap, sync::LazyLock};



pub static PATTERNS: LazyLock<HashMap<String, Pattern>> = LazyLock::new(|| {
    HashMap::from([
        ("Quadratic".to_string(), Pattern::new(
            "Quadratic", 
            Vec::from(["a", "b", "c", "x"]), 
            r#"let result = a * x ^ 2 + b * x + c"#
        )),
        ("SqrRt".to_string(), Pattern::new(
            "SqrRt", 
            Vec::from(["x"]), 
            r#"let result = x ^ (1/2)"#
        )),
        ("CubeRt".to_string(), Pattern::new(
            "CubeRt", 
            Vec::from(["x"]), 
            r#"let result = x ^ (1/3)"#
        )),
    ])
});



#[derive(Debug, Clone)]
pub struct Pattern {
    name: String,
    context_params: Vec<String>,
    content: String
}

impl Pattern {
    pub fn new(name: &str, context_params: Vec<&str>, content: &str) -> Self {
        let name = name.to_string();
        let content = content.to_string();
        let context_params = context_params.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        Self {
            name,
            context_params,
            content
        }
    }
    
    pub fn construct(&self) -> (String, String) {
        let mut builder = String::new();
        builder.push_str(&format!("#[{}]\n", self.context_params.join(", ")));
        builder.push_str(&self.content);
        (self.name.to_owned(), builder)
    }
}
    


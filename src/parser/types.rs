use std::collections::HashMap;




pub struct VariableContainer {
    pub numbers: HashMap<String, f64>,
    
}


impl VariableContainer {
    pub fn new() -> Self {
        let mut nums = HashMap::new();
        nums.insert("restult".to_string(), 0.0);
    
        Self {
            numbers: nums,
        }
        
        
    }
    
    pub fn get_number(&self, key: &str) -> Option<&f64> {
        self.numbers.get(key)
    }
}
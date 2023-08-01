use std::collections::HashMap;

const VARIABLE_PREFIX: &str = "let";
const CONST_PREFIX: &str = "const";
const FUNCTION_PREFIX: &str = "fn";
const STRUCT_PREFIX: &str = "struct";

#[derive(Clone, Debug)]
pub struct IdentGenerator{
    prefix_map: HashMap<String, i32>
}

impl IdentGenerator {
    pub fn new() -> Self {
        IdentGenerator {
            prefix_map: HashMap::new(),
        }
    }

    fn generate_variable(&mut self) -> String {
        self.generate(VARIABLE_PREFIX)
    }

    fn generate_const(&mut self) -> String {
        self.generate(CONST_PREFIX)
    }

    fn generate_function_name(&mut self) -> String {
        self.generate(FUNCTION_PREFIX)
    }

    fn generate_struct_name(&mut self) -> String {
        self.generate(STRUCT_PREFIX)
    }

    fn generate(&mut self, prefix: &str) -> String {
        let count = self.prefix_map.entry(prefix.to_string()).or_insert(0);
        *count += 1;
        format!("{}{}", prefix, count)
    }
}






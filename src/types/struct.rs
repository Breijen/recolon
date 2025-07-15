use std::collections::HashMap;
use std::fmt;
use crate::expr::Expr;
use crate::literal_value::LiteralValue;

#[derive(Clone, Debug)]
pub struct StructDefinition {
    pub name: String,
    pub fields: HashMap<String, Expr>, // Fields as expressions during parsing
}

#[derive(Clone, Debug)]
pub struct StructInstance {
    pub name: String,
    pub fields: HashMap<String, LiteralValue>, // Fields as evaluated values during runtime
}

// Implement Display for StructInstance to format the output as desired
impl fmt::Display for StructInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fields_string = String::new();

        // Convert each field to a string in the format "name: value"
        for (key, value) in &self.fields {
            fields_string.push_str(&format!("\"{}\": {:?}", key, value));
            fields_string.push_str(", ");
        }

        // Remove the trailing comma and space
        if fields_string.len() > 2 {
            fields_string.truncate(fields_string.len() - 2);
        }

        // Write the final formatted string to the formatter
        write!(f, "{{ name: \"{}\", fields: {{{}}} }}", self.name, fields_string)
    }
}

impl StructInstance {
    // Method to retrieve a value by field name
    pub fn get_field(&self, field_name: &str) -> Option<&LiteralValue> {
        self.fields.get(field_name)
    }
}
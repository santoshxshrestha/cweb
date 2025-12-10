use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct CompilationResult {
    success: bool,
    output: String,
    error: Option<String>,
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Main function to compile and execute C code
/// Returns a JSON string with compilation result
#[wasm_bindgen]
pub fn compile_and_run_c(c_code: &str) -> String {
    let result = match compile_c_code(c_code) {
        Ok(output) => CompilationResult {
            success: true,
            output,
            error: None,
        },
        Err(error) => CompilationResult {
            success: false,
            output: String::new(),
            error: Some(error),
        },
    };

    serde_json::to_string(&result).unwrap_or_else(|_| {
        r#"{"success":false,"output":"","error":"Failed to serialize result"}"#.to_string()
    })
}

/// Simple C interpreter for basic C programs
fn compile_c_code(code: &str) -> Result<String, String> {
    // Parse and execute the C code
    let mut interpreter = CInterpreter::new();
    interpreter.execute(code)
}

struct CInterpreter {
    variables: HashMap<String, i64>,
    output: String,
}

impl CInterpreter {
    fn new() -> Self {
        CInterpreter {
            variables: HashMap::new(),
            output: String::new(),
        }
    }

    fn execute(&mut self, code: &str) -> Result<String, String> {
        // Basic C code parsing and execution
        // This is a simplified interpreter for demonstration
        
        // Find main function
        let main_start = code.find("int main").or(code.find("void main"))
            .ok_or_else(|| "Error: No main function found".to_string())?;
        
        let code = &code[main_start..];
        
        // Find the body of main function
        let body_start = code.find('{')
            .ok_or_else(|| "Error: Invalid main function syntax".to_string())?;
        
        let body_end = self.find_matching_brace(code, body_start)
            .ok_or_else(|| "Error: Unmatched braces in main function".to_string())?;
        
        let body = &code[body_start + 1..body_end];
        
        // Execute statements in the body
        self.execute_statements(body)?;
        
        Ok(self.output.clone())
    }

    fn find_matching_brace(&self, code: &str, start: usize) -> Option<usize> {
        let chars: Vec<char> = code.chars().collect();
        let mut depth = 0;
        
        for i in start..chars.len() {
            match chars[i] {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn execute_statements(&mut self, body: &str) -> Result<(), String> {
        let statements: Vec<&str> = body.split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for statement in statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    fn execute_statement(&mut self, statement: &str) -> Result<(), String> {
        let statement = statement.trim();
        
        // Handle printf statements
        if statement.contains("printf") {
            return self.handle_printf(statement);
        }
        
        // Handle variable declarations and assignments
        if statement.contains("int ") {
            return self.handle_int_declaration(statement);
        }
        
        // Handle assignments
        if statement.contains('=') && !statement.starts_with("int ") {
            return self.handle_assignment(statement);
        }
        
        // Handle return statement
        if statement.starts_with("return") {
            return Ok(()); // Just ignore return for now
        }

        Ok(())
    }

    fn handle_printf(&mut self, statement: &str) -> Result<(), String> {
        // Extract format string and arguments
        let start = statement.find('(')
            .ok_or_else(|| "Error: Invalid printf syntax".to_string())?;
        let end = statement.rfind(')')
            .ok_or_else(|| "Error: Invalid printf syntax".to_string())?;
        
        let args = &statement[start + 1..end];
        
        // Parse format string
        let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        
        if parts.is_empty() {
            return Ok(());
        }

        let format_str = parts[0].trim_matches('"');
        
        // Simple format string processing
        let mut result = format_str.to_string();
        
        // Handle escape sequences
        result = result.replace("\\n", "\n");
        result = result.replace("\\t", "\t");
        result = result.replace("\\\\", "\\");
        
        // Handle format specifiers
        for arg in parts.iter().skip(1) {
            let value = self.evaluate_expression(arg)?;
            
            // Replace format specifiers
            if let Some(pos) = result.find("%d") {
                result.replace_range(pos..pos + 2, &value.to_string());
            } else if let Some(pos) = result.find("%i") {
                result.replace_range(pos..pos + 2, &value.to_string());
            }
        }
        
        self.output.push_str(&result);
        Ok(())
    }

    fn handle_int_declaration(&mut self, statement: &str) -> Result<(), String> {
        let statement = statement.strip_prefix("int ").unwrap_or(statement).trim();
        
        if let Some(eq_pos) = statement.find('=') {
            let var_name = statement[..eq_pos].trim().to_string();
            let expr = statement[eq_pos + 1..].trim();
            let value = self.evaluate_expression(expr)?;
            self.variables.insert(var_name, value);
        } else {
            // Just declaration without initialization
            let var_name = statement.trim().to_string();
            self.variables.insert(var_name, 0);
        }
        
        Ok(())
    }

    fn handle_assignment(&mut self, statement: &str) -> Result<(), String> {
        let parts: Vec<&str> = statement.split('=').collect();
        if parts.len() != 2 {
            return Err("Error: Invalid assignment syntax".to_string());
        }
        
        let var_name = parts[0].trim().to_string();
        let expr = parts[1].trim();
        let value = self.evaluate_expression(expr)?;
        
        self.variables.insert(var_name, value);
        Ok(())
    }

    fn evaluate_expression(&self, expr: &str) -> Result<i64, String> {
        let expr = expr.trim();
        
        // Check if it's a number
        if let Ok(num) = expr.parse::<i64>() {
            return Ok(num);
        }
        
        // Check if it's a variable
        if let Some(&value) = self.variables.get(expr) {
            return Ok(value);
        }
        
        // Handle simple arithmetic expressions
        if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').collect();
            let left = self.evaluate_expression(parts[0].trim())?;
            let right = self.evaluate_expression(parts[1].trim())?;
            return Ok(left + right);
        }
        
        if expr.contains('-') && !expr.starts_with('-') {
            let parts: Vec<&str> = expr.split('-').collect();
            let left = self.evaluate_expression(parts[0].trim())?;
            let right = self.evaluate_expression(parts[1].trim())?;
            return Ok(left - right);
        }
        
        if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').collect();
            let left = self.evaluate_expression(parts[0].trim())?;
            let right = self.evaluate_expression(parts[1].trim())?;
            return Ok(left * right);
        }
        
        if expr.contains('/') {
            let parts: Vec<&str> = expr.split('/').collect();
            let left = self.evaluate_expression(parts[0].trim())?;
            let right = self.evaluate_expression(parts[1].trim())?;
            if right == 0 {
                return Err("Error: Division by zero".to_string());
            }
            return Ok(left / right);
        }
        
        Err(format!("Error: Cannot evaluate expression: {}", expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hello_world() {
        let code = r#"
            #include <stdio.h>
            int main() {
                printf("Hello, World!\n");
                return 0;
            }
        "#;
        
        let result = compile_and_run_c(code);
        assert!(result.contains("Hello, World!"));
    }

    #[test]
    fn test_variable_declaration() {
        let code = r#"
            int main() {
                int x = 5;
                printf("x = %d\n", x);
                return 0;
            }
        "#;
        
        let result = compile_and_run_c(code);
        assert!(result.contains("x = 5"));
    }

    #[test]
    fn test_arithmetic() {
        let code = r#"
            int main() {
                int a = 10;
                int b = 20;
                int c = a + b;
                printf("Result: %d\n", c);
                return 0;
            }
        "#;
        
        let result = compile_and_run_c(code);
        assert!(result.contains("Result: 30"));
    }
}

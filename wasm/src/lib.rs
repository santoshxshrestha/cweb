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

#[derive(Clone, Debug)]
enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

struct CInterpreter {
    variables: HashMap<String, Value>,
    output: String,
    input_buffer: Vec<String>,
}

impl CInterpreter {
    fn new() -> Self {
        CInterpreter {
            variables: HashMap::new(),
            output: String::new(),
            input_buffer: Vec::new(),
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
        // Handle for loops
        if body.contains("for") {
            return self.handle_for_loop(body);
        }

        // Handle while loops
        if body.contains("while") {
            return self.handle_while_loop(body);
        }

        // Handle if statements
        if body.contains("if") {
            return self.handle_if_statement(body);
        }

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

        // Handle scanf statements
        if statement.contains("scanf") {
            return self.handle_scanf(statement);
        }

        // Handle puts statement
        if statement.contains("puts") {
            return self.handle_puts(statement);
        }

        // Handle strlen
        if statement.contains("strlen") {
            return self.handle_strlen(statement);
        }

        // Handle math functions
        if statement.contains("sqrt") || statement.contains("pow") || 
           statement.contains("abs") || statement.contains("sin") ||
           statement.contains("cos") || statement.contains("tan") {
            return self.handle_math_function(statement);
        }
        
        // Handle variable declarations
        if statement.starts_with("int ") || statement.starts_with("float ") || 
           statement.starts_with("double ") || statement.starts_with("char ") {
            return self.handle_declaration(statement);
        }
        
        // Handle assignments
        if statement.contains('=') && !self.is_declaration(statement) {
            return self.handle_assignment(statement);
        }

        // Handle increment/decrement
        if statement.contains("++") || statement.contains("--") {
            return self.handle_increment_decrement(statement);
        }
        
        // Handle return statement
        if statement.starts_with("return") {
            return Ok(()); // Just ignore return for now
        }

        Ok(())
    }

    fn is_declaration(&self, statement: &str) -> bool {
        statement.starts_with("int ") || statement.starts_with("float ") ||
        statement.starts_with("double ") || statement.starts_with("char ")
    }

    fn handle_for_loop(&mut self, body: &str) -> Result<(), String> {
        // Simple for loop parsing: for(int i = 0; i < n; i++)
        let for_start = body.find("for").ok_or("Invalid for loop")?;
        let paren_start = body[for_start..].find('(').ok_or("Invalid for loop syntax")? + for_start;
        let paren_end = body[paren_start..].find(')').ok_or("Invalid for loop syntax")? + paren_start;
        
        let for_header = &body[paren_start + 1..paren_end];
        let parts: Vec<&str> = for_header.split(';').collect();
        
        if parts.len() != 3 {
            return Err("Invalid for loop syntax".to_string());
        }

        // Initialize
        self.execute_statement(parts[0].trim())?;

        // Find loop body
        let body_start = body[paren_end..].find('{').ok_or("Invalid for loop body")? + paren_end;
        let body_end = self.find_matching_brace(body, body_start).ok_or("Unmatched braces")?;
        let loop_body = &body[body_start + 1..body_end];

        // Execute loop
        let condition = parts[1].trim();
        let increment = parts[2].trim();

        while self.evaluate_condition(condition)? {
            self.execute_statements(loop_body)?;
            self.execute_statement(increment)?;
        }

        Ok(())
    }

    fn handle_while_loop(&mut self, body: &str) -> Result<(), String> {
        let while_start = body.find("while").ok_or("Invalid while loop")?;
        let paren_start = body[while_start..].find('(').ok_or("Invalid while loop syntax")? + while_start;
        let paren_end = body[paren_start..].find(')').ok_or("Invalid while loop syntax")? + paren_start;
        
        let condition = &body[paren_start + 1..paren_end];

        // Find loop body
        let body_start = body[paren_end..].find('{').ok_or("Invalid while loop body")? + paren_end;
        let body_end = self.find_matching_brace(body, body_start).ok_or("Unmatched braces")?;
        let loop_body = &body[body_start + 1..body_end];

        // Execute loop
        while self.evaluate_condition(condition)? {
            self.execute_statements(loop_body)?;
        }

        Ok(())
    }

    fn handle_if_statement(&mut self, body: &str) -> Result<(), String> {
        let if_start = body.find("if").ok_or("Invalid if statement")?;
        let paren_start = body[if_start..].find('(').ok_or("Invalid if syntax")? + if_start;
        let paren_end = body[paren_start..].find(')').ok_or("Invalid if syntax")? + paren_start;
        
        let condition = &body[paren_start + 1..paren_end];

        // Find if body
        let body_start = body[paren_end..].find('{').ok_or("Invalid if body")? + paren_end;
        let body_end = self.find_matching_brace(body, body_start).ok_or("Unmatched braces")?;
        let if_body = &body[body_start + 1..body_end];

        // Execute if condition is true
        if self.evaluate_condition(condition)? {
            self.execute_statements(if_body)?;
        }

        Ok(())
    }

    fn evaluate_condition(&self, condition: &str) -> Result<bool, String> {
        let condition = condition.trim();

        // Handle comparison operators
        if condition.contains("<=") {
            let parts: Vec<&str> = condition.split("<=").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left <= right);
        }

        if condition.contains(">=") {
            let parts: Vec<&str> = condition.split(">=").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left >= right);
        }

        if condition.contains("==") {
            let parts: Vec<&str> = condition.split("==").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left == right);
        }

        if condition.contains("!=") {
            let parts: Vec<&str> = condition.split("!=").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left != right);
        }

        if condition.contains('<') && !condition.contains("<<") {
            let parts: Vec<&str> = condition.split('<').collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left < right);
        }

        if condition.contains('>') && !condition.contains(">>") {
            let parts: Vec<&str> = condition.split('>').collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left > right);
        }

        Err(format!("Cannot evaluate condition: {}", condition))
    }

    fn handle_increment_decrement(&mut self, statement: &str) -> Result<(), String> {
        if statement.contains("++") {
            let var_name = statement.replace("++", "").trim().to_string();
            if let Some(Value::Int(val)) = self.variables.get(&var_name) {
                self.variables.insert(var_name, Value::Int(val + 1));
            }
        } else if statement.contains("--") {
            let var_name = statement.replace("--", "").trim().to_string();
            if let Some(Value::Int(val)) = self.variables.get(&var_name) {
                self.variables.insert(var_name, Value::Int(val - 1));
            }
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
        let parts = self.split_args(args);
        
        if parts.is_empty() {
            return Ok(());
        }

        let format_str = parts[0].trim_matches('"').trim_matches('\'');
        
        // Simple format string processing
        let mut result = format_str.to_string();
        
        // Handle escape sequences
        result = result.replace("\\n", "\n");
        result = result.replace("\\t", "\t");
        result = result.replace("\\\\", "\\");
        result = result.replace("\\r", "\r");
        
        // Handle format specifiers
        let mut arg_index = 1;
        let format_specs = vec!["%d", "%i", "%f", "%lf", "%c", "%s", "%ld", "%u"];
        
        for spec in format_specs {
            while result.contains(spec) && arg_index < parts.len() {
                let value = self.evaluate_value_expression(&parts[arg_index])?;
                let replacement = match value {
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Char(c) => c.to_string(),
                    Value::String(s) => s,
                    Value::Bool(b) => (b as i32).to_string(),
                };
                
                if let Some(pos) = result.find(spec) {
                    result.replace_range(pos..pos + spec.len(), &replacement);
                }
                arg_index += 1;
            }
        }
        
        self.output.push_str(&result);
        Ok(())
    }

    fn handle_scanf(&mut self, _statement: &str) -> Result<(), String> {
        // Simplified scanf - just acknowledge it for now
        // In a real implementation, you'd need input handling
        Ok(())
    }

    fn handle_puts(&mut self, statement: &str) -> Result<(), String> {
        let start = statement.find('(').ok_or("Invalid puts syntax")?;
        let end = statement.rfind(')').ok_or("Invalid puts syntax")?;
        let content = &statement[start + 1..end].trim_matches('"');
        
        self.output.push_str(content);
        self.output.push('\n');
        Ok(())
    }

    fn handle_strlen(&mut self, statement: &str) -> Result<(), String> {
        // Handle strlen in assignment: int len = strlen("hello");
        if statement.contains('=') {
            let parts: Vec<&str> = statement.split('=').collect();
            let var_name = parts[0].replace("int ", "").trim().to_string();
            
            let strlen_part = parts[1].trim();
            let start = strlen_part.find('(').ok_or("Invalid strlen syntax")?;
            let end = strlen_part.rfind(')').ok_or("Invalid strlen syntax")?;
            let string_content = &strlen_part[start + 1..end].trim_matches('"');
            
            let length = string_content.len() as i64;
            self.variables.insert(var_name, Value::Int(length));
        }
        Ok(())
    }

    fn handle_math_function(&mut self, statement: &str) -> Result<(), String> {
        if statement.contains('=') {
            let parts: Vec<&str> = statement.split('=').collect();
            let var_type_name = parts[0].trim();
            let var_name = if var_type_name.contains(' ') {
                var_type_name.split_whitespace().last().unwrap()
            } else {
                var_type_name
            };
            
            let expr = parts[1].trim();
            
            // Handle sqrt
            if expr.contains("sqrt") {
                let start = expr.find('(').ok_or("Invalid sqrt syntax")?;
                let end = expr.rfind(')').ok_or("Invalid sqrt syntax")?;
                let arg = &expr[start + 1..end];
                let value = self.evaluate_numeric_expression(arg)?;
                let result = (value as f64).sqrt();
                self.variables.insert(var_name.to_string(), Value::Float(result));
                return Ok(());
            }

            // Handle pow
            if expr.contains("pow") {
                let start = expr.find('(').ok_or("Invalid pow syntax")?;
                let end = expr.rfind(')').ok_or("Invalid pow syntax")?;
                let args_str = &expr[start + 1..end];
                let args: Vec<&str> = args_str.split(',').collect();
                if args.len() == 2 {
                    let base = self.evaluate_numeric_expression(args[0].trim())?;
                    let exp = self.evaluate_numeric_expression(args[1].trim())?;
                    let result = (base as f64).powf(exp as f64);
                    self.variables.insert(var_name.to_string(), Value::Float(result));
                }
                return Ok(());
            }

            // Handle abs
            if expr.contains("abs") {
                let start = expr.find('(').ok_or("Invalid abs syntax")?;
                let end = expr.rfind(')').ok_or("Invalid abs syntax")?;
                let arg = &expr[start + 1..end];
                let value = self.evaluate_numeric_expression(arg)?;
                self.variables.insert(var_name.to_string(), Value::Int(value.abs()));
                return Ok(());
            }
        }
        Ok(())
    }

    fn split_args(&self, args: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_string = false;
        let mut paren_depth = 0;

        for ch in args.chars() {
            match ch {
                '"' => in_string = !in_string,
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                ',' if !in_string && paren_depth == 0 => {
                    if !current.is_empty() {
                        result.push(current.trim().to_string());
                        current = String::new();
                    }
                    continue;
                }
                _ => {}
            }
            current.push(ch);
        }

        if !current.is_empty() {
            result.push(current.trim().to_string());
        }

        result
    }

    fn handle_declaration(&mut self, statement: &str) -> Result<(), String> {
        let statement = statement.trim();
        
        let (var_type, rest) = if statement.starts_with("int ") {
            ("int", statement.strip_prefix("int ").unwrap())
        } else if statement.starts_with("float ") {
            ("float", statement.strip_prefix("float ").unwrap())
        } else if statement.starts_with("double ") {
            ("double", statement.strip_prefix("double ").unwrap())
        } else if statement.starts_with("char ") {
            ("char", statement.strip_prefix("char ").unwrap())
        } else {
            return Err("Unknown type".to_string());
        };

        if let Some(eq_pos) = rest.find('=') {
            let var_name = rest[..eq_pos].trim().to_string();
            let expr = rest[eq_pos + 1..].trim();
            
            let value = match var_type {
                "float" | "double" => {
                    let num = self.evaluate_numeric_expression(expr)? as f64;
                    Value::Float(num)
                },
                "char" => {
                    let ch = expr.trim_matches('\'').chars().next().unwrap_or('\0');
                    Value::Char(ch)
                },
                _ => {
                    let num = self.evaluate_numeric_expression(expr)?;
                    Value::Int(num)
                }
            };
            
            self.variables.insert(var_name, value);
        } else {
            // Just declaration without initialization
            let var_name = rest.trim().to_string();
            let value = match var_type {
                "float" | "double" => Value::Float(0.0),
                "char" => Value::Char('\0'),
                _ => Value::Int(0),
            };
            self.variables.insert(var_name, value);
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
        
        // Check if variable exists to determine type
        if let Some(existing_value) = self.variables.get(&var_name) {
            let value = match existing_value {
                Value::Float(_) => {
                    let num = self.evaluate_numeric_expression(expr)? as f64;
                    Value::Float(num)
                },
                Value::Char(_) => {
                    let ch = expr.trim_matches('\'').chars().next().unwrap_or('\0');
                    Value::Char(ch)
                },
                _ => {
                    let num = self.evaluate_numeric_expression(expr)?;
                    Value::Int(num)
                }
            };
            self.variables.insert(var_name, value);
        } else {
            let num = self.evaluate_numeric_expression(expr)?;
            self.variables.insert(var_name, Value::Int(num));
        }
        
        Ok(())
    }

    fn evaluate_value_expression(&self, expr: &str) -> Result<Value, String> {
        let expr = expr.trim();
        
        // Check if it's a string literal
        if expr.starts_with('"') && expr.ends_with('"') {
            return Ok(Value::String(expr.trim_matches('"').to_string()));
        }

        // Check if it's a char literal
        if expr.starts_with('\'') && expr.ends_with('\'') {
            let ch = expr.trim_matches('\'').chars().next().unwrap_or('\0');
            return Ok(Value::Char(ch));
        }

        // Check if it's a variable
        if let Some(value) = self.variables.get(expr) {
            return Ok(value.clone());
        }

        // Otherwise treat as numeric
        let num = self.evaluate_numeric_expression(expr)?;
        Ok(Value::Int(num))
    }

    fn evaluate_numeric_expression(&self, expr: &str) -> Result<i64, String> {
        let expr = expr.trim();
        
        // Check if it's a number
        if let Ok(num) = expr.parse::<i64>() {
            return Ok(num);
        }

        // Check if it's a float
        if let Ok(num) = expr.parse::<f64>() {
            return Ok(num as i64);
        }
        
        // Check if it's a variable
        if let Some(value) = self.variables.get(expr) {
            return match value {
                Value::Int(i) => Ok(*i),
                Value::Float(f) => Ok(*f as i64),
                Value::Char(c) => Ok(*c as i64),
                Value::Bool(b) => Ok(*b as i64),
                Value::String(_) => Err("Cannot convert string to number".to_string()),
            };
        }

        // Handle parentheses
        if expr.starts_with('(') && expr.ends_with(')') {
            return self.evaluate_numeric_expression(&expr[1..expr.len() - 1]);
        }
        
        // Handle simple arithmetic expressions with operator precedence
        // First handle + and -
        let chars: Vec<char> = expr.chars().collect();
        let mut depth = 0;
        for i in (0..chars.len()).rev() {
            let ch = chars[i];
            match ch {
                ')' => depth += 1,
                '(' => depth -= 1,
                '+' if depth == 0 && i > 0 => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left + right);
                }
                '-' if depth == 0 && i > 0 => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left - right);
                }
                _ => {}
            }
        }

        // Then handle * and /
        depth = 0;
        for i in (0..chars.len()).rev() {
            let ch = chars[i];
            match ch {
                ')' => depth += 1,
                '(' => depth -= 1,
                '*' if depth == 0 => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left * right);
                }
                '/' if depth == 0 => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    if right == 0 {
                        return Err("Error: Division by zero".to_string());
                    }
                    return Ok(left / right);
                }
                '%' if depth == 0 => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left % right);
                }
                _ => {}
            }
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

    #[test]
    fn test_for_loop() {
        let code = r#"
            int main() {
                int i;
                for(i = 0; i < 3; i++) {
                    printf("%d ", i);
                }
                return 0;
            }
        "#;
        
        let result = compile_and_run_c(code);
        assert!(result.contains("0 1 2"));
    }
}

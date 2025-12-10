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
    Array(Vec<Value>),
    Pointer(i64), // Simulated memory address
}

#[derive(Clone, Debug)]
struct Function {
    params: Vec<(String, String)>, // (type, name)
    body: String,
    return_type: String,
}

// Simulated memory system for pointers
struct Memory {
    heap: HashMap<i64, Value>,
    next_address: i64,
    // Map variable names to their simulated addresses
    address_map: HashMap<String, i64>,
}

impl Memory {
    fn new() -> Self {
        Memory {
            heap: HashMap::new(),
            next_address: 0x1000, // Start at a "realistic" address
            address_map: HashMap::new(),
        }
    }

    fn allocate(&mut self, value: Value) -> i64 {
        let addr = self.next_address;
        self.heap.insert(addr, value);
        self.next_address += 8; // Simulate 8-byte alignment
        addr
    }

    fn get_address_of(&mut self, var_name: &str, value: &Value) -> i64 {
        if let Some(&addr) = self.address_map.get(var_name) {
            addr
        } else {
            let addr = self.next_address;
            self.heap.insert(addr, value.clone());
            self.address_map.insert(var_name.to_string(), addr);
            self.next_address += 8;
            addr
        }
    }

    fn read(&self, addr: i64) -> Result<Value, String> {
        self.heap.get(&addr)
            .cloned()
            .ok_or_else(|| format!("Segmentation fault: invalid memory address 0x{:x}", addr))
    }

    fn write(&mut self, addr: i64, value: Value) -> Result<(), String> {
        if self.heap.contains_key(&addr) {
            self.heap.insert(addr, value);
            Ok(())
        } else {
            Err(format!("Segmentation fault: invalid memory address 0x{:x}", addr))
        }
    }

    fn update_variable_address(&mut self, var_name: &str, value: &Value) {
        if let Some(&addr) = self.address_map.get(var_name) {
            self.heap.insert(addr, value.clone());
        }
    }
}

struct CInterpreter {
    variables: HashMap<String, Value>,
    global_variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    output: String,
    input_buffer: Vec<String>,
    loop_break: bool,
    loop_continue: bool,
    memory: Memory,
}

impl CInterpreter {
    fn new() -> Self {
        CInterpreter {
            variables: HashMap::new(),
            global_variables: HashMap::new(),
            functions: HashMap::new(),
            output: String::new(),
            input_buffer: Vec::new(),
            loop_break: false,
            loop_continue: false,
            memory: Memory::new(),
        }
    }

    fn execute(&mut self, code: &str) -> Result<String, String> {
        // Parse global variables and functions
        self.parse_globals_and_functions(code)?;
        
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

    fn parse_globals_and_functions(&mut self, _code: &str) -> Result<(), String> {
        // This is a simplified parser - just acknowledges functions exist
        // In a real implementation, you would parse function definitions here
        Ok(())
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

    fn find_matching_paren(&self, code: &str, start: usize) -> Option<usize> {
        let chars: Vec<char> = code.chars().collect();
        let mut depth = 0;
        
        for i in start..chars.len() {
            match chars[i] {
                '(' => depth += 1,
                ')' => {
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
        let body = body.trim();
        if body.is_empty() {
            return Ok(());
        }

        // Handle complex structures first
        if body.contains("for") {
            return self.handle_for_loop(body);
        }

        if body.contains("while") {
            return self.handle_while_loop(body);
        }

        if body.contains("do") && body.contains("while") {
            return self.handle_do_while_loop(body);
        }

        if body.contains("if") {
            return self.handle_if_else_statement(body);
        }

        if body.contains("switch") {
            return self.handle_switch_statement(body);
        }

        let statements: Vec<String> = self.split_statements(body).iter().map(|s| s.to_string()).collect();

        for statement in statements {
            if self.loop_break || self.loop_continue {
                break;
            }
            self.execute_statement(&statement)?;
        }

        Ok(())
    }

    fn split_statements<'a>(&self, body: &'a str) -> Vec<&'a str> {
        let mut statements = Vec::new();
        let mut current_start = 0;
        let mut brace_depth = 0;
        let mut paren_depth = 0;
        let mut in_string = false;
        let chars: Vec<char> = body.chars().collect();

        for i in 0..chars.len() {
            match chars[i] {
                '"' if i == 0 || chars[i - 1] != '\\' => in_string = !in_string,
                '{' if !in_string => brace_depth += 1,
                '}' if !in_string => brace_depth -= 1,
                '(' if !in_string => paren_depth += 1,
                ')' if !in_string => paren_depth -= 1,
                ';' if !in_string && brace_depth == 0 && paren_depth == 0 => {
                    let stmt = body[current_start..=i].trim();
                    if !stmt.is_empty() {
                        statements.push(stmt);
                    }
                    current_start = i + 1;
                }
                _ => {}
            }
        }

        let remaining = body[current_start..].trim();
        if !remaining.is_empty() {
            statements.push(remaining);
        }

        statements
    }

    fn execute_statement(&mut self, statement: &str) -> Result<(), String> {
        let statement = statement.trim().trim_end_matches(';');
        
        if statement.is_empty() {
            return Ok(());
        }

        // Handle break and continue
        if statement == "break" {
            self.loop_break = true;
            return Ok(());
        }

        if statement == "continue" {
            self.loop_continue = true;
            return Ok(());
        }

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

        // Handle gets statement
        if statement.contains("gets") {
            return self.handle_gets(statement);
        }

        // Handle string functions
        if statement.contains("strlen") {
            return self.handle_strlen(statement);
        }

        if statement.contains("strcpy") {
            return self.handle_strcpy(statement);
        }

        if statement.contains("strcmp") {
            return self.handle_strcmp(statement);
        }

        if statement.contains("strcat") {
            return self.handle_strcat(statement);
        }

        // Handle math functions
        if statement.contains("sqrt") || statement.contains("pow") || 
           statement.contains("abs") || statement.contains("sin") ||
           statement.contains("cos") || statement.contains("tan") ||
           statement.contains("ceil") || statement.contains("floor") ||
           statement.contains("exp") || statement.contains("log") ||
           statement.contains("fabs") {
            return self.handle_math_function(statement);
        }

        // Handle rand/srand
        if statement.contains("rand") {
            return self.handle_rand(statement);
        }

        if statement.contains("srand") {
            return self.handle_srand(statement);
        }
        
        // Handle variable declarations
        if statement.starts_with("int ") || statement.starts_with("float ") || 
           statement.starts_with("double ") || statement.starts_with("char ") ||
           statement.starts_with("long ") || statement.starts_with("short ") {
            return self.handle_declaration(statement);
        }
        
        // Handle assignments with compound operators
        if statement.contains("+=") || statement.contains("-=") || 
           statement.contains("*=") || statement.contains("/=") ||
           statement.contains("%=") {
            return self.handle_compound_assignment(statement);
        }

        // Handle assignments
        if statement.contains('=') && !self.is_declaration(statement) && 
           !statement.contains("==") && !statement.contains("!=") &&
           !statement.contains("<=") && !statement.contains(">=") {
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
        statement.starts_with("double ") || statement.starts_with("char ") ||
        statement.starts_with("long ") || statement.starts_with("short ")
    }

    fn handle_for_loop(&mut self, body: &str) -> Result<(), String> {
        let for_start = body.find("for").ok_or("Invalid for loop")?;
        let paren_start = body[for_start..].find('(').ok_or("Invalid for loop syntax")? + for_start;
        let paren_end = self.find_matching_paren(body, paren_start).ok_or("Invalid for loop syntax")?;
        
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

        let max_iterations = 100000; // Safety limit
        let mut iterations = 0;

        while self.evaluate_condition(condition)? {
            if iterations >= max_iterations {
                return Err("Loop exceeded maximum iterations (possible infinite loop)".to_string());
            }
            iterations += 1;

            self.loop_break = false;
            self.loop_continue = false;

            self.execute_statements(loop_body)?;

            if self.loop_break {
                self.loop_break = false;
                break;
            }

            if !self.loop_continue {
                self.execute_statement(increment)?;
            } else {
                self.execute_statement(increment)?;
                self.loop_continue = false;
            }
        }

        Ok(())
    }

    fn handle_while_loop(&mut self, body: &str) -> Result<(), String> {
        let while_start = body.find("while").ok_or("Invalid while loop")?;
        let paren_start = body[while_start..].find('(').ok_or("Invalid while loop syntax")? + while_start;
        let paren_end = self.find_matching_paren(body, paren_start).ok_or("Invalid while loop syntax")?;
        
        let condition = &body[paren_start + 1..paren_end];

        // Find loop body
        let body_start = body[paren_end..].find('{').ok_or("Invalid while loop body")? + paren_end;
        let body_end = self.find_matching_brace(body, body_start).ok_or("Unmatched braces")?;
        let loop_body = &body[body_start + 1..body_end];

        let max_iterations = 100000;
        let mut iterations = 0;

        // Execute loop
        while self.evaluate_condition(condition)? {
            if iterations >= max_iterations {
                return Err("Loop exceeded maximum iterations (possible infinite loop)".to_string());
            }
            iterations += 1;

            self.loop_break = false;
            self.loop_continue = false;

            self.execute_statements(loop_body)?;

            if self.loop_break {
                self.loop_break = false;
                break;
            }

            self.loop_continue = false;
        }

        Ok(())
    }

    fn handle_do_while_loop(&mut self, body: &str) -> Result<(), String> {
        let do_start = body.find("do").ok_or("Invalid do-while loop")?;
        let body_start = body[do_start..].find('{').ok_or("Invalid do-while body")? + do_start;
        let body_end = self.find_matching_brace(body, body_start).ok_or("Unmatched braces")?;
        let loop_body = &body[body_start + 1..body_end];

        let while_start = body[body_end..].find("while").ok_or("Invalid do-while loop")? + body_end;
        let paren_start = body[while_start..].find('(').ok_or("Invalid do-while syntax")? + while_start;
        let paren_end = self.find_matching_paren(body, paren_start).ok_or("Invalid do-while syntax")?;
        let condition = &body[paren_start + 1..paren_end];

        let max_iterations = 100000;
        let mut iterations = 0;

        loop {
            if iterations >= max_iterations {
                return Err("Loop exceeded maximum iterations (possible infinite loop)".to_string());
            }
            iterations += 1;

            self.loop_break = false;
            self.loop_continue = false;

            self.execute_statements(loop_body)?;

            if self.loop_break {
                self.loop_break = false;
                break;
            }

            self.loop_continue = false;

            if !self.evaluate_condition(condition)? {
                break;
            }
        }

        Ok(())
    }

    fn handle_if_else_statement(&mut self, body: &str) -> Result<(), String> {
        let if_start = body.find("if").ok_or("Invalid if statement")?;
        let paren_start = body[if_start..].find('(').ok_or("Invalid if syntax")? + if_start;
        let paren_end = self.find_matching_paren(body, paren_start).ok_or("Invalid if syntax")?;
        
        let condition = &body[paren_start + 1..paren_end];

        // Find if body
        let body_start = body[paren_end..].find('{').ok_or("Invalid if body")? + paren_end;
        let body_end = self.find_matching_brace(body, body_start).ok_or("Unmatched braces")?;
        let if_body = &body[body_start + 1..body_end];

        // Check for else
        let remaining = body[body_end + 1..].trim();
        
        if self.evaluate_condition(condition)? {
            self.execute_statements(if_body)?;
        } else if remaining.starts_with("else") {
            let else_part = &remaining[4..].trim();
            
            // Check for else if
            if else_part.starts_with("if") {
                self.handle_if_else_statement(else_part)?;
            } else {
                // Simple else
                let else_body_start = else_part.find('{').ok_or("Invalid else body")?;
                let else_body_end = self.find_matching_brace(else_part, else_body_start).ok_or("Unmatched braces")?;
                let else_body = &else_part[else_body_start + 1..else_body_end];
                self.execute_statements(else_body)?;
            }
        }

        Ok(())
    }

    fn handle_switch_statement(&mut self, body: &str) -> Result<(), String> {
        let switch_start = body.find("switch").ok_or("Invalid switch statement")?;
        let paren_start = body[switch_start..].find('(').ok_or("Invalid switch syntax")? + switch_start;
        let paren_end = self.find_matching_paren(body, paren_start).ok_or("Invalid switch syntax")?;
        
        let switch_expr = &body[paren_start + 1..paren_end];
        let switch_value = self.evaluate_numeric_expression(switch_expr)?;

        let body_start = body[paren_end..].find('{').ok_or("Invalid switch body")? + paren_end;
        let body_end = self.find_matching_brace(body, body_start).ok_or("Unmatched braces")?;
        let switch_body = &body[body_start + 1..body_end];

        // Find matching case
        let mut _found_case = false;
        let mut execute_remaining = false;
        
        let lines: Vec<&str> = switch_body.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.starts_with("case") {
                let case_value_str = line.strip_prefix("case").unwrap().trim().trim_end_matches(':');
                if let Ok(case_value) = case_value_str.parse::<i64>() {
                    if case_value == switch_value || execute_remaining {
                        _found_case = true;
                        execute_remaining = true;
                        i += 1;
                        continue;
                    }
                }
            } else if line.starts_with("default:") {
                execute_remaining = true;
                i += 1;
                continue;
            } else if line == "break;" && execute_remaining {
                break;
            } else if execute_remaining && !line.is_empty() {
                self.execute_statement(line)?;
            }
            
            i += 1;
        }

        Ok(())
    }

    fn evaluate_condition(&mut self, condition: &str) -> Result<bool, String> {
        let condition = condition.trim();

        // Handle logical operators
        if condition.contains("&&") {
            let parts: Vec<&str> = condition.splitn(2, "&&").collect();
            let left = self.evaluate_condition(parts[0].trim())?;
            let right = self.evaluate_condition(parts[1].trim())?;
            return Ok(left && right);
        }

        if condition.contains("||") {
            let parts: Vec<&str> = condition.splitn(2, "||").collect();
            let left = self.evaluate_condition(parts[0].trim())?;
            let right = self.evaluate_condition(parts[1].trim())?;
            return Ok(left || right);
        }

        if condition.starts_with('!') {
            let inner = self.evaluate_condition(&condition[1..].trim())?;
            return Ok(!inner);
        }

        // Handle comparison operators
        if condition.contains("<=") {
            let parts: Vec<&str> = condition.splitn(2, "<=").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left <= right);
        }

        if condition.contains(">=") {
            let parts: Vec<&str> = condition.splitn(2, ">=").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left >= right);
        }

        if condition.contains("==") {
            let parts: Vec<&str> = condition.splitn(2, "==").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left == right);
        }

        if condition.contains("!=") {
            let parts: Vec<&str> = condition.splitn(2, "!=").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left != right);
        }

        if condition.contains('<') && !condition.contains("<<") {
            let parts: Vec<&str> = condition.splitn(2, '<').collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left < right);
        }

        if condition.contains('>') && !condition.contains(">>") {
            let parts: Vec<&str> = condition.splitn(2, '>').collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left > right);
        }

        // Handle simple boolean values
        let val = self.evaluate_numeric_expression(condition)?;
        Ok(val != 0)
    }

    fn handle_increment_decrement(&mut self, statement: &str) -> Result<(), String> {
        if statement.contains("++") {
            let var_name = statement.replace("++", "").trim().to_string();
            if let Some(val) = self.variables.get(&var_name) {
                match val {
                    Value::Int(i) => {
                        self.variables.insert(var_name, Value::Int(i + 1));
                    },
                    Value::Float(f) => {
                        self.variables.insert(var_name, Value::Float(f + 1.0));
                    },
                    _ => {}
                }
            }
        } else if statement.contains("--") {
            let var_name = statement.replace("--", "").trim().to_string();
            if let Some(val) = self.variables.get(&var_name) {
                match val {
                    Value::Int(i) => {
                        self.variables.insert(var_name, Value::Int(i - 1));
                    },
                    Value::Float(f) => {
                        self.variables.insert(var_name, Value::Float(f - 1.0));
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_printf(&mut self, statement: &str) -> Result<(), String> {
        let start = statement.find('(')
            .ok_or_else(|| "Error: Invalid printf syntax".to_string())?;
        let end = statement.rfind(')')
            .ok_or_else(|| "Error: Invalid printf syntax".to_string())?;
        
        let args = &statement[start + 1..end];
        let parts = self.split_args(args);
        
        if parts.is_empty() {
            return Ok(());
        }

        let format_str = parts[0].trim_matches('"').trim_matches('\'');
        let mut result = format_str.to_string();
        
        // Handle escape sequences
        result = result.replace("\\n", "\n");
        result = result.replace("\\t", "\t");
        result = result.replace("\\\\", "\\");
        result = result.replace("\\r", "\r");
        result = result.replace("\\0", "\0");
        
        // Handle format specifiers
        let mut arg_index = 1;
        let format_specs = vec!["%p", "%d", "%i", "%f", "%lf", "%c", "%s", "%ld", "%u", "%x", "%o"];
        
        for spec in format_specs {
            while result.contains(spec) && arg_index < parts.len() {
                let value = self.evaluate_value_expression(&parts[arg_index])?;
                let replacement = match value {
                    Value::Int(i) => {
                        if spec == "%x" {
                            format!("{:x}", i)
                        } else if spec == "%o" {
                            format!("{:o}", i)
                        } else {
                            i.to_string()
                        }
                    },
                    Value::Float(f) => f.to_string(),
                    Value::Char(c) => c.to_string(),
                    Value::String(s) => s,
                    Value::Bool(b) => (b as i32).to_string(),
                    Value::Array(_) => "[array]".to_string(),
                    Value::Pointer(addr) => {
                        if spec == "%x" {
                            format!("{:x}", addr)
                        } else if spec == "%p" {
                            format!("0x{:x}", addr)
                        } else {
                            format!("0x{:x}", addr)
                        }
                    },
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

    fn handle_gets(&mut self, _statement: &str) -> Result<(), String> {
        // Simplified gets - just acknowledge it for now
        Ok(())
    }

    fn handle_strlen(&mut self, statement: &str) -> Result<(), String> {
        if statement.contains('=') {
            let parts: Vec<&str> = statement.split('=').collect();
            let var_part = parts[0].trim();
            let var_name = if var_part.contains(' ') {
                var_part.split_whitespace().last().unwrap()
            } else {
                var_part
            };
            
            let strlen_part = parts[1].trim();
            let start = strlen_part.find('(').ok_or("Invalid strlen syntax")?;
            let end = strlen_part.rfind(')').ok_or("Invalid strlen syntax")?;
            let arg = &strlen_part[start + 1..end].trim();
            
            let length = if arg.starts_with('"') {
                arg.trim_matches('"').len() as i64
            } else {
                // It's a variable
                if let Some(Value::String(s)) = self.variables.get(*arg) {
                    s.len() as i64
                } else {
                    0
                }
            };
            
            self.variables.insert(var_name.to_string(), Value::Int(length));
        }
        Ok(())
    }

    fn handle_strcpy(&mut self, statement: &str) -> Result<(), String> {
        let start = statement.find('(').ok_or("Invalid strcpy syntax")?;
        let end = statement.rfind(')').ok_or("Invalid strcpy syntax")?;
        let args_str = &statement[start + 1..end];
        let args = self.split_args(args_str);
        
        if args.len() >= 2 {
            let dest = args[0].trim();
            let src = args[1].trim();
            
            let src_value = if src.starts_with('"') {
                Value::String(src.trim_matches('"').to_string())
            } else {
                self.variables.get(src).cloned().unwrap_or(Value::String(String::new()))
            };
            
            self.variables.insert(dest.to_string(), src_value);
        }
        Ok(())
    }

    fn handle_strcmp(&mut self, _statement: &str) -> Result<(), String> {
        // This would typically return a value for comparison
        // For now, simplified implementation
        Ok(())
    }

    fn handle_strcat(&mut self, statement: &str) -> Result<(), String> {
        let start = statement.find('(').ok_or("Invalid strcat syntax")?;
        let end = statement.rfind(')').ok_or("Invalid strcat syntax")?;
        let args_str = &statement[start + 1..end];
        let args = self.split_args(args_str);
        
        if args.len() >= 2 {
            let dest = args[0].trim();
            let src = args[1].trim();
            
            if let Some(Value::String(dest_str)) = self.variables.get(dest).cloned() {
                let src_str = if src.starts_with('"') {
                    src.trim_matches('"').to_string()
                } else {
                    match self.variables.get(src) {
                        Some(Value::String(s)) => s.clone(),
                        _ => String::new(),
                    }
                };
                
                let result = format!("{}{}", dest_str, src_str);
                self.variables.insert(dest.to_string(), Value::String(result));
            }
        }
        Ok(())
    }

    fn handle_math_function(&mut self, statement: &str) -> Result<(), String> {
        if !statement.contains('=') {
            return Ok(());
        }

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
            let args = self.split_args(args_str);
            if args.len() == 2 {
                let base = self.evaluate_numeric_expression(args[0].trim())?;
                let exp = self.evaluate_numeric_expression(args[1].trim())?;
                let result = (base as f64).powf(exp as f64);
                self.variables.insert(var_name.to_string(), Value::Float(result));
            }
            return Ok(());
        }

        // Handle abs/fabs
        if expr.contains("abs") || expr.contains("fabs") {
            let start = expr.find('(').ok_or("Invalid abs syntax")?;
            let end = expr.rfind(')').ok_or("Invalid abs syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            if expr.contains("fabs") {
                self.variables.insert(var_name.to_string(), Value::Float((value as f64).abs()));
            } else {
                self.variables.insert(var_name.to_string(), Value::Int(value.abs()));
            }
            return Ok(());
        }

        // Handle ceil
        if expr.contains("ceil") {
            let start = expr.find('(').ok_or("Invalid ceil syntax")?;
            let end = expr.rfind(')').ok_or("Invalid ceil syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            let result = (value as f64).ceil();
            self.variables.insert(var_name.to_string(), Value::Float(result));
            return Ok(());
        }

        // Handle floor
        if expr.contains("floor") {
            let start = expr.find('(').ok_or("Invalid floor syntax")?;
            let end = expr.rfind(')').ok_or("Invalid floor syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            let result = (value as f64).floor();
            self.variables.insert(var_name.to_string(), Value::Float(result));
            return Ok(());
        }

        // Handle exp
        if expr.contains("exp") {
            let start = expr.find('(').ok_or("Invalid exp syntax")?;
            let end = expr.rfind(')').ok_or("Invalid exp syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            let result = (value as f64).exp();
            self.variables.insert(var_name.to_string(), Value::Float(result));
            return Ok(());
        }

        // Handle log
        if expr.contains("log") {
            let start = expr.find('(').ok_or("Invalid log syntax")?;
            let end = expr.rfind(')').ok_or("Invalid log syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            let result = (value as f64).ln();
            self.variables.insert(var_name.to_string(), Value::Float(result));
            return Ok(());
        }

        // Handle sin, cos, tan
        if expr.contains("sin") {
            let start = expr.find('(').ok_or("Invalid sin syntax")?;
            let end = expr.rfind(')').ok_or("Invalid sin syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            let result = (value as f64).sin();
            self.variables.insert(var_name.to_string(), Value::Float(result));
            return Ok(());
        }

        if expr.contains("cos") {
            let start = expr.find('(').ok_or("Invalid cos syntax")?;
            let end = expr.rfind(')').ok_or("Invalid cos syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            let result = (value as f64).cos();
            self.variables.insert(var_name.to_string(), Value::Float(result));
            return Ok(());
        }

        if expr.contains("tan") {
            let start = expr.find('(').ok_or("Invalid tan syntax")?;
            let end = expr.rfind(')').ok_or("Invalid tan syntax")?;
            let arg = &expr[start + 1..end];
            let value = self.evaluate_numeric_expression(arg)?;
            let result = (value as f64).tan();
            self.variables.insert(var_name.to_string(), Value::Float(result));
            return Ok(());
        }

        Ok(())
    }

    fn handle_rand(&mut self, statement: &str) -> Result<(), String> {
        if statement.contains('=') {
            let parts: Vec<&str> = statement.split('=').collect();
            let var_part = parts[0].trim();
            let var_name = if var_part.contains(' ') {
                var_part.split_whitespace().last().unwrap()
            } else {
                var_part
            };
            
            // Simple pseudo-random number generator
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            self.output.len().hash(&mut hasher);
            let random_val = (hasher.finish() % 32768) as i64;
            
            self.variables.insert(var_name.to_string(), Value::Int(random_val));
        }
        Ok(())
    }

    fn handle_srand(&mut self, _statement: &str) -> Result<(), String> {
        // Acknowledge srand but don't implement seeding for now
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
        } else if statement.starts_with("long ") {
            ("long", statement.strip_prefix("long ").unwrap())
        } else if statement.starts_with("short ") {
            ("short", statement.strip_prefix("short ").unwrap())
        } else {
            return Err("Unknown type".to_string());
        };

        // Handle pointer declarations (e.g., int *ptr or int* ptr)
        let rest = rest.trim();
        let is_pointer = rest.starts_with('*');
        let rest = if is_pointer {
            rest.trim_start_matches('*').trim()
        } else {
            rest
        };

        // Handle array declarations
        if rest.contains('[') && !is_pointer {
            let bracket_pos = rest.find('[').unwrap();
            let var_name = rest[..bracket_pos].trim().to_string();
            let bracket_end = rest.find(']').ok_or("Invalid array syntax")?;
            let size_str = &rest[bracket_pos + 1..bracket_end];
            let size = if size_str.is_empty() {
                0
            } else {
                self.evaluate_numeric_expression(size_str)? as usize
            };
            
            let default_value = match var_type {
                "float" | "double" => Value::Float(0.0),
                "char" => Value::Char('\0'),
                _ => Value::Int(0),
            };
            
            let array = vec![default_value.clone(); size];
            let array_value = Value::Array(array);
            
            // Store array in memory and create a "pointer" to it
            let addr = self.memory.allocate(array_value.clone());
            self.memory.address_map.insert(var_name.clone(), addr);
            self.variables.insert(var_name, array_value);
            return Ok(());
        }

        if let Some(eq_pos) = rest.find('=') {
            let var_name = rest[..eq_pos].trim().to_string();
            let expr = rest[eq_pos + 1..].trim();
            
            let value = if is_pointer {
                // Handle pointer initialization
                self.evaluate_pointer_expression(expr)?
            } else {
                match var_type {
                    "float" | "double" => {
                        let num = self.evaluate_numeric_expression(expr)? as f64;
                        Value::Float(num)
                    },
                    "char" => {
                        if expr.starts_with('\'') {
                            let ch = expr.trim_matches('\'').chars().next().unwrap_or('\0');
                            Value::Char(ch)
                        } else {
                            let num = self.evaluate_numeric_expression(expr)?;
                            Value::Char(num as u8 as char)
                        }
                    },
                    _ => {
                        let num = self.evaluate_numeric_expression(expr)?;
                        Value::Int(num)
                    }
                }
            };
            
            if !is_pointer {
                // For non-pointers, store them in memory so they can be referenced
                let addr = self.memory.get_address_of(&var_name, &value);
                self.memory.update_variable_address(&var_name, &value);
            }
            
            self.variables.insert(var_name, value);
        } else {
            // Just declaration without initialization
            let var_name = rest.trim().to_string();
            let value = if is_pointer {
                Value::Pointer(0) // NULL pointer
            } else {
                match var_type {
                    "float" | "double" => Value::Float(0.0),
                    "char" => Value::Char('\0'),
                    _ => Value::Int(0),
                }
            };
            
            if !is_pointer {
                let addr = self.memory.get_address_of(&var_name, &value);
                self.memory.update_variable_address(&var_name, &value);
            }
            
            self.variables.insert(var_name, value);
        }
        
        Ok(())
    }

    fn evaluate_pointer_expression(&mut self, expr: &str) -> Result<Value, String> {
        let expr = expr.trim();
        
        // Handle NULL or 0
        if expr == "NULL" || expr == "0" {
            return Ok(Value::Pointer(0));
        }
        
        // Handle address-of operator: &variable
        if expr.starts_with('&') {
            let var_name = expr[1..].trim();
            
            // Handle array element: &arr[index]
            if var_name.contains('[') {
                let bracket_pos = var_name.find('[').unwrap();
                let array_name = var_name[..bracket_pos].trim();
                let bracket_end = var_name.find(']').ok_or("Invalid array syntax")?;
                let index_expr = &var_name[bracket_pos + 1..bracket_end];
                let index = self.evaluate_numeric_expression(index_expr)? as usize;
                
                // Get base address of array and add offset
                if let Some(&base_addr) = self.memory.address_map.get(array_name) {
                    let element_addr = base_addr + (index as i64 * 8);
                    return Ok(Value::Pointer(element_addr));
                } else {
                    return Err(format!("Variable '{}' not found", array_name));
                }
            }
            
            // Regular variable address
            if let Some(value) = self.variables.get(var_name) {
                let addr = self.memory.get_address_of(var_name, value);
                return Ok(Value::Pointer(addr));
            } else {
                return Err(format!("Variable '{}' not found", var_name));
            }
        }
        
        // Handle dereference in expression (this is for pointer assignment)
        if expr.starts_with('*') {
            let ptr_expr = expr[1..].trim();
            if let Some(Value::Pointer(addr)) = self.variables.get(ptr_expr) {
                let value = self.memory.read(*addr)?;
                return Ok(Value::Pointer(*addr)); // Return the address for pointer-to-pointer
            }
        }
        
        // Handle direct pointer variable or expression
        if let Some(value) = self.variables.get(expr) {
            if let Value::Pointer(addr) = value {
                return Ok(Value::Pointer(*addr));
            }
        }
        
        // Try to evaluate as numeric expression (cast to pointer)
        let num = self.evaluate_numeric_expression(expr)?;
        Ok(Value::Pointer(num))
    }

    fn handle_compound_assignment(&mut self, statement: &str) -> Result<(), String> {
        let ops = vec!["+=", "-=", "*=", "/=", "%="];
        
        for op in ops {
            if statement.contains(op) {
                let parts: Vec<&str> = statement.splitn(2, op).collect();
                if parts.len() == 2 {
                    let var_name = parts[0].trim();
                    let expr = parts[1].trim();
                    
                    let current_val = self.evaluate_numeric_expression(var_name)?;
                    let expr_val = self.evaluate_numeric_expression(expr)?;
                    
                    let result = match op {
                        "+=" => current_val + expr_val,
                        "-=" => current_val - expr_val,
                        "*=" => current_val * expr_val,
                        "/=" => {
                            if expr_val == 0 {
                                return Err("Division by zero".to_string());
                            }
                            current_val / expr_val
                        },
                        "%=" => current_val % expr_val,
                        _ => current_val,
                    };
                    
                    self.variables.insert(var_name.to_string(), Value::Int(result));
                    return Ok(());
                }
            }
        }
        
        Ok(())
    }

    fn handle_assignment(&mut self, statement: &str) -> Result<(), String> {
        let parts: Vec<&str> = statement.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err("Error: Invalid assignment syntax".to_string());
        }
        
        let left = parts[0].trim();
        let expr = parts[1].trim();

        // Handle pointer dereference assignment: *ptr = value
        if left.starts_with('*') {
            let ptr_name = left[1..].trim();
            if let Some(Value::Pointer(addr)) = self.variables.get(ptr_name) {
                let addr = *addr;
                let value = if expr.starts_with('"') {
                    Value::String(expr.trim_matches('"').to_string())
                } else if expr.starts_with('\'') {
                    Value::Char(expr.trim_matches('\'').chars().next().unwrap_or('\0'))
                } else if expr.starts_with('&') {
                    // Assigning an address
                    self.evaluate_pointer_expression(expr)?
                } else {
                    // Try numeric or variable
                    if let Some(var_value) = self.variables.get(expr) {
                        var_value.clone()
                    } else {
                        let num = self.evaluate_numeric_expression(expr)?;
                        Value::Int(num)
                    }
                };
                
                self.memory.write(addr, value.clone())?;
                
                // Update the variable map if this address corresponds to a variable
                for (var_name, &var_addr) in &self.memory.address_map {
                    if var_addr == addr {
                        self.variables.insert(var_name.clone(), value.clone());
                        break;
                    }
                }
                
                return Ok(());
            } else {
                return Err(format!("'{}' is not a valid pointer", ptr_name));
            }
        }

        // Handle array element assignment
        if left.contains('[') {
            let bracket_pos = left.find('[').unwrap();
            let var_name = left[..bracket_pos].trim();
            let bracket_end = left.find(']').ok_or("Invalid array syntax")?;
            let index_expr = &left[bracket_pos + 1..bracket_end];
            let index = self.evaluate_numeric_expression(index_expr)? as usize;
            
            let value = if expr.starts_with('"') {
                Value::String(expr.trim_matches('"').to_string())
            } else if expr.starts_with('\'') {
                Value::Char(expr.trim_matches('\'').chars().next().unwrap_or('\0'))
            } else {
                let num = self.evaluate_numeric_expression(expr)?;
                Value::Int(num)
            };
            
            if let Some(Value::Array(arr)) = self.variables.get_mut(var_name) {
                if index < arr.len() {
                    arr[index] = value.clone();
                    
                    // Update memory if this array has an address
                    if let Some(&base_addr) = self.memory.address_map.get(var_name) {
                        let element_addr = base_addr + (index as i64 * 8);
                        if self.memory.heap.contains_key(&element_addr) {
                            self.memory.write(element_addr, value)?;
                        }
                    }
                }
            }
            return Ok(());
        }

        let var_name = left.to_string();
        
        // Handle pointer assignment
        if expr.starts_with('&') || (self.variables.get(&var_name).map(|v| matches!(v, Value::Pointer(_))).unwrap_or(false)) {
            let value = self.evaluate_pointer_expression(expr)?;
            self.variables.insert(var_name, value);
            return Ok(());
        }
        
        // Check if variable exists to determine type
        if let Some(existing_value) = self.variables.get(&var_name).cloned() {
            let value = match existing_value {
                Value::Float(_) => {
                    let num = self.evaluate_numeric_expression(expr)? as f64;
                    Value::Float(num)
                },
                Value::Char(_) => {
                    if expr.starts_with('\'') {
                        let ch = expr.trim_matches('\'').chars().next().unwrap_or('\0');
                        Value::Char(ch)
                    } else {
                        let num = self.evaluate_numeric_expression(expr)?;
                        Value::Char(num as u8 as char)
                    }
                },
                Value::String(_) => {
                    if expr.starts_with('"') {
                        Value::String(expr.trim_matches('"').to_string())
                    } else {
                        existing_value
                    }
                },
                Value::Pointer(_) => {
                    self.evaluate_pointer_expression(expr)?
                },
                _ => {
                    let num = self.evaluate_numeric_expression(expr)?;
                    Value::Int(num)
                }
            };
            
            self.variables.insert(var_name.clone(), value.clone());
            
            // Update memory
            if let Some(&addr) = self.memory.address_map.get(&var_name) {
                self.memory.write(addr, value)?;
            }
        } else {
            // New variable
            let value = if expr.starts_with('"') {
                Value::String(expr.trim_matches('"').to_string())
            } else if expr.starts_with('\'') {
                Value::Char(expr.trim_matches('\'').chars().next().unwrap_or('\0'))
            } else if expr.starts_with('&') {
                self.evaluate_pointer_expression(expr)?
            } else {
                let num = self.evaluate_numeric_expression(expr)?;
                Value::Int(num)
            };
            
            let addr = self.memory.get_address_of(&var_name, &value);
            self.memory.update_variable_address(&var_name, &value);
            self.variables.insert(var_name, value);
        }
        
        Ok(())
    }

    fn evaluate_value_expression(&mut self, expr: &str) -> Result<Value, String> {
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

    fn evaluate_numeric_expression(&mut self, expr: &str) -> Result<i64, String> {
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
                Value::Array(_) => Err("Cannot convert array to number".to_string()),
                Value::Pointer(addr) => Ok(*addr), // Pointer can be used as integer (address)
            };
        }

        // Handle array element access
        if expr.contains('[') {
            let bracket_pos = expr.find('[').unwrap();
            let var_name = expr[..bracket_pos].trim();
            let bracket_end = expr.find(']').ok_or("Invalid array syntax")?;
            let index_expr = &expr[bracket_pos + 1..bracket_end];
            let index = self.evaluate_numeric_expression(index_expr)? as usize;
            
            if let Some(Value::Array(arr)) = self.variables.get(var_name) {
                if index < arr.len() {
                    return match &arr[index] {
                        Value::Int(i) => Ok(*i),
                        Value::Float(f) => Ok(*f as i64),
                        Value::Char(c) => Ok(*c as i64),
                        Value::Bool(b) => Ok(*b as i64),
                        _ => Err("Invalid array element type".to_string()),
                    };
                }
            }
        }

        // Handle pointer dereference: *ptr
        if expr.starts_with('*') {
            let ptr_expr = expr[1..].trim();
            if let Some(Value::Pointer(addr)) = self.variables.get(ptr_expr) {
                let value = self.memory.read(*addr)?;
                return match value {
                    Value::Int(i) => Ok(i),
                    Value::Float(f) => Ok(f as i64),
                    Value::Char(c) => Ok(c as i64),
                    Value::Bool(b) => Ok(b as i64),
                    _ => Err("Cannot dereference to numeric value".to_string()),
                };
            } else {
                return Err(format!("'{}' is not a valid pointer", ptr_expr));
            }
        }

        // Handle address-of operator: &variable (returns address as number)
        if expr.starts_with('&') {
            let var_name = expr[1..].trim();
            if let Some(value) = self.variables.get(var_name) {
                let addr = self.memory.get_address_of(var_name, value);
                return Ok(addr);
            } else {
                return Err(format!("Variable '{}' not found", var_name));
            }
        }

        // Handle ternary operator
        if expr.contains('?') && expr.contains(':') {
            let q_pos = expr.find('?').unwrap();
            let c_pos = expr.rfind(':').unwrap();
            
            let condition = &expr[..q_pos].trim();
            let true_expr = &expr[q_pos + 1..c_pos].trim();
            let false_expr = &expr[c_pos + 1..].trim();
            
            if self.evaluate_condition(condition)? {
                return self.evaluate_numeric_expression(true_expr);
            } else {
                return self.evaluate_numeric_expression(false_expr);
            }
        }

        // Handle parentheses
        if expr.starts_with('(') && expr.ends_with(')') {
            return self.evaluate_numeric_expression(&expr[1..expr.len() - 1]);
        }
        
        // Handle bitwise operators
        let chars: Vec<char> = expr.chars().collect();
        let mut depth = 0;
        
        // Handle bitwise OR
        for i in (0..chars.len()).rev() {
            match chars[i] {
                ')' => depth += 1,
                '(' => depth -= 1,
                '|' if depth == 0 && (i == 0 || chars[i-1] != '|') && (i == chars.len()-1 || chars[i+1] != '|') => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left | right);
                }
                _ => {}
            }
        }

        // Handle bitwise AND
        depth = 0;
        for i in (0..chars.len()).rev() {
            match chars[i] {
                ')' => depth += 1,
                '(' => depth -= 1,
                '&' if depth == 0 && (i == 0 || chars[i-1] != '&') && (i == chars.len()-1 || chars[i+1] != '&') => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left & right);
                }
                _ => {}
            }
        }

        // Handle bitwise XOR
        depth = 0;
        for i in (0..chars.len()).rev() {
            match chars[i] {
                ')' => depth += 1,
                '(' => depth -= 1,
                '^' if depth == 0 => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left ^ right);
                }
                _ => {}
            }
        }

        // Handle bit shifts
        if expr.contains("<<") {
            let parts: Vec<&str> = expr.splitn(2, "<<").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left << right);
        }

        if expr.contains(">>") {
            let parts: Vec<&str> = expr.splitn(2, ">>").collect();
            let left = self.evaluate_numeric_expression(parts[0].trim())?;
            let right = self.evaluate_numeric_expression(parts[1].trim())?;
            return Ok(left >> right);
        }
        
        // Handle simple arithmetic expressions with operator precedence
        // First handle + and -
        depth = 0;
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
                '-' if depth == 0 && i > 0 && chars[i-1] != 'e' && chars[i-1] != 'E' => {
                    let left = self.evaluate_numeric_expression(&expr[..i])?;
                    let right = self.evaluate_numeric_expression(&expr[i + 1..])?;
                    return Ok(left - right);
                }
                _ => {}
            }
        }

        // Then handle * and / and %
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

        // Handle unary minus
        if expr.starts_with('-') {
            let val = self.evaluate_numeric_expression(&expr[1..])?;
            return Ok(-val);
        }

        // Handle bitwise NOT
        if expr.starts_with('~') {
            let val = self.evaluate_numeric_expression(&expr[1..])?;
            return Ok(!val);
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
    fn test_for_loop_with_break() {
        let code = r#"
            int main() {
                for(int i = 0; i < 10; i++) {
                    if(i == 5) {
                        break;
                    }
                    printf("%d ", i);
                }
                return 0;
            }
        "#;
        
        let result = compile_and_run_c(code);
        assert!(result.contains("0 1 2 3 4"));
    }

    #[test]
    fn test_else_statement() {
        let code = r#"
            int main() {
                int x = 5;
                if(x > 10) {
                    printf("Greater\n");
                } else {
                    printf("Smaller\n");
                }
                return 0;
            }
        "#;
        
        let result = compile_and_run_c(code);
        assert!(result.contains("Smaller"));
    }
}

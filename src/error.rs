use std::fmt;
use colored::*;

/// Main error type for the Recolon language
/// Provides structured error handling with context and suggestions
#[derive(Debug, Clone)]
pub enum RecolonError {
    SyntaxError(SyntaxError),
    RuntimeError(RuntimeError),
    TypeError(TypeError),
    MathError(MathError),
    IOError(IOError),
    ImportError(ImportError),
}

/// Syntax errors that occur during parsing
#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub token: Option<String>,
    pub suggestion: Option<String>,
}

/// Runtime errors that occur during execution
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
    pub function: Option<String>,
    pub stack_trace: Vec<String>,
    pub suggestion: Option<String>,
}

/// Type-related errors
#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub line: usize,
    pub expected_type: String,
    pub actual_type: String,
    pub suggestion: Option<String>,
}

/// Mathematical operation errors
#[derive(Debug, Clone)]
pub struct MathError {
    pub message: String,
    pub line: usize,
    pub operation: String,
    pub operands: Vec<String>,
    pub suggestion: Option<String>,
}

/// I/O related errors
#[derive(Debug, Clone)]
pub struct IOError {
    pub message: String,
    pub line: usize,
    pub file_path: Option<String>,
    pub suggestion: Option<String>,
}

/// Import/module related errors
#[derive(Debug, Clone)]
pub struct ImportError {
    pub message: String,
    pub line: usize,
    pub module_name: String,
    pub suggestion: Option<String>,
}

impl RecolonError {
    /// Create a new syntax error
    pub fn syntax(message: String, line: usize, column: usize) -> Self {
        RecolonError::SyntaxError(SyntaxError {
            message,
            line,
            column,
            token: None,
            suggestion: None,
        })
    }

    /// Create a new syntax error with token information
    pub fn syntax_with_token(message: String, line: usize, column: usize, token: String) -> Self {
        RecolonError::SyntaxError(SyntaxError {
            message,
            line,
            column,
            token: Some(token),
            suggestion: None,
        })
    }

    /// Create a new runtime error
    pub fn runtime(message: String, line: usize) -> Self {
        RecolonError::RuntimeError(RuntimeError {
            message,
            line,
            function: None,
            stack_trace: Vec::new(),
            suggestion: None,
        })
    }

    /// Create a new type error
    pub fn type_error(message: String, line: usize, expected: String, actual: String) -> Self {
        RecolonError::TypeError(TypeError {
            message,
            line,
            expected_type: expected,
            actual_type: actual,
            suggestion: None,
        })
    }

    /// Create a new math error
    pub fn math(message: String, line: usize, operation: String) -> Self {
        RecolonError::MathError(MathError {
            message,
            line,
            operation,
            operands: Vec::new(),
            suggestion: None,
        })
    }

    /// Create a new I/O error
    pub fn io(message: String, line: usize) -> Self {
        RecolonError::IOError(IOError {
            message,
            line,
            file_path: None,
            suggestion: None,
        })
    }

    /// Create a new import error
    pub fn import(message: String, line: usize, module_name: String) -> Self {
        RecolonError::ImportError(ImportError {
            message,
            line,
            module_name,
            suggestion: None,
        })
    }

    /// Add a suggestion to the error
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        match &mut self {
            RecolonError::SyntaxError(err) => err.suggestion = Some(suggestion),
            RecolonError::RuntimeError(err) => err.suggestion = Some(suggestion),
            RecolonError::TypeError(err) => err.suggestion = Some(suggestion),
            RecolonError::MathError(err) => err.suggestion = Some(suggestion),
            RecolonError::IOError(err) => err.suggestion = Some(suggestion),
            RecolonError::ImportError(err) => err.suggestion = Some(suggestion),
        }
        self
    }

    /// Get the line number for this error
    pub fn line(&self) -> usize {
        match self {
            RecolonError::SyntaxError(err) => err.line,
            RecolonError::RuntimeError(err) => err.line,
            RecolonError::TypeError(err) => err.line,
            RecolonError::MathError(err) => err.line,
            RecolonError::IOError(err) => err.line,
            RecolonError::ImportError(err) => err.line,
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        match self {
            RecolonError::SyntaxError(err) => &err.message,
            RecolonError::RuntimeError(err) => &err.message,
            RecolonError::TypeError(err) => &err.message,
            RecolonError::MathError(err) => &err.message,
            RecolonError::IOError(err) => &err.message,
            RecolonError::ImportError(err) => &err.message,
        }
    }

    /// Get the error type name
    pub fn error_type(&self) -> &str {
        match self {
            RecolonError::SyntaxError(_) => "SyntaxError",
            RecolonError::RuntimeError(_) => "RuntimeError",
            RecolonError::TypeError(_) => "TypeError",
            RecolonError::MathError(_) => "MathError",
            RecolonError::IOError(_) => "IOError",
            RecolonError::ImportError(_) => "ImportError",
        }
    }

    /// Format the error with colors and context
    pub fn format_error(&self, source_code: Option<&str>) -> String {
        let mut output = String::new();
        
        // Error header
        output.push_str(&format!("{}: {}\n", 
            self.error_type().red().bold(),
            self.message()
        ));

        // Line information
        output.push_str(&format!(" {} line {}\n", 
            "-->".blue().bold(),
            self.line().to_string().yellow()
        ));

        // Show source code context if available
        if let Some(source) = source_code {
            let lines: Vec<&str> = source.lines().collect();
            let error_line = self.line();
            
            if error_line > 0 && error_line <= lines.len() {
                let line_num = error_line;
                let line_content = lines[error_line - 1];
                
                // Show line number and content
                output.push_str(&format!(" {} | {}\n", 
                    line_num.to_string().blue().bold(),
                    line_content
                ));
                
                // Show error indicator
                if let RecolonError::SyntaxError(syntax_err) = self {
                    let spaces = " ".repeat(syntax_err.column);
                    output.push_str(&format!(" {} | {}{}\n", 
                        " ".repeat(line_num.to_string().len()),
                        spaces,
                        "^".red().bold()
                    ));
                }
            }
        }

        // Show additional context based on error type
        match self {
            RecolonError::TypeError(type_err) => {
                output.push_str(&format!("\n {} Expected: {}, Found: {}\n",
                    "note:".blue().bold(),
                    type_err.expected_type.green(),
                    type_err.actual_type.red()
                ));
            },
            RecolonError::MathError(math_err) => {
                output.push_str(&format!("\n {} Operation: {}\n",
                    "note:".blue().bold(),
                    math_err.operation.cyan()
                ));
            },
            RecolonError::ImportError(import_err) => {
                output.push_str(&format!("\n {} Module: {}\n",
                    "note:".blue().bold(),
                    import_err.module_name.cyan()
                ));
            },
            _ => {}
        }

        // Show suggestion if available
        if let Some(suggestion) = self.get_suggestion() {
            output.push_str(&format!("\n {} {}\n",
                "help:".green().bold(),
                suggestion
            ));
        }

        output
    }

    /// Get the suggestion for this error
    fn get_suggestion(&self) -> Option<&str> {
        match self {
            RecolonError::SyntaxError(err) => err.suggestion.as_deref(),
            RecolonError::RuntimeError(err) => err.suggestion.as_deref(),
            RecolonError::TypeError(err) => err.suggestion.as_deref(),
            RecolonError::MathError(err) => err.suggestion.as_deref(),
            RecolonError::IOError(err) => err.suggestion.as_deref(),
            RecolonError::ImportError(err) => err.suggestion.as_deref(),
        }
    }
}

impl fmt::Display for RecolonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_error(None))
    }
}

impl std::error::Error for RecolonError {}

/// Convert from String to RecolonError for backward compatibility
impl From<String> for RecolonError {
    fn from(message: String) -> Self {
        RecolonError::runtime(message, 0)
    }
}

/// Convert from &str to RecolonError for backward compatibility
impl From<&str> for RecolonError {
    fn from(message: &str) -> Self {
        RecolonError::runtime(message.to_string(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_error_creation() {
        let error = RecolonError::syntax("Unexpected token".to_string(), 10, 5);
        assert_eq!(error.line(), 10);
        assert_eq!(error.message(), "Unexpected token");
        assert_eq!(error.error_type(), "SyntaxError");
    }

    #[test]
    fn test_error_with_suggestion() {
        let error = RecolonError::math("Division by zero".to_string(), 5, "division".to_string())
            .with_suggestion("Check if divisor is zero before division".to_string());
        
        assert_eq!(error.line(), 5);
        assert!(error.get_suggestion().is_some());
    }

    #[test]
    fn test_type_error() {
        let error = RecolonError::type_error(
            "Type mismatch".to_string(),
            15,
            "Number".to_string(),
            "String".to_string()
        );
        
        assert_eq!(error.line(), 15);
        if let RecolonError::TypeError(type_err) = error {
            assert_eq!(type_err.expected_type, "Number");
            assert_eq!(type_err.actual_type, "String");
        }
    }
}

use lex::Token;

trait Error {
    fn error(&self, message: &str);
}

impl Error for Token {
    fn error(&self, message: &str) {
        println!("Error: {} at line {}", message, self.line);
    }
}
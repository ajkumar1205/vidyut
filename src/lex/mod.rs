use colored::Colorize;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            line: 1,
        }
    }

    /// Starts parsing the content of the file
    pub fn parse(&mut self) {
        let num = HashSet::from(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        let alpha = HashSet::from([
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H",
            "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
            "Z",
        ]);
        while self.current < self.source.len() {
            let s = &self.source[self.current..self.current + 1];
            match s {
                "." => {
                    self.tokens.push(Token::new(s, TokenType::Dot, self.line));
                    self.advance();
                }
                "," => {
                    self.tokens.push(Token::new(s, TokenType::Comma, self.line));
                    self.advance();
                }
                ";" => {
                    self.tokens
                        .push(Token::new(s, TokenType::Semicolon, self.line));
                    self.advance();
                }
                ":" => {
                    self.tokens.push(Token::new(s, TokenType::Colon, self.line));
                    self.advance();
                }
                "(" => {
                    self.tokens
                        .push(Token::new(s, TokenType::LeftParen, self.line));
                    self.advance();
                }
                ")" => {
                    self.tokens
                        .push(Token::new(s, TokenType::RightParen, self.line));
                    self.advance();
                }
                "{" => {
                    self.tokens
                        .push(Token::new(s, TokenType::LeftBrace, self.line));
                    self.advance();
                }
                "}" => {
                    self.tokens
                        .push(Token::new(s, TokenType::RightBrace, self.line));
                    self.advance();
                }
                "[" => {
                    self.tokens
                        .push(Token::new(s, TokenType::LeftBracket, self.line));
                    self.advance();
                }
                "]" => {
                    self.tokens
                        .push(Token::new(s, TokenType::RightBracket, self.line));
                    self.advance();
                }
                "+" => {
                    self.tokens.push(Token::new(s, TokenType::Plus, self.line));
                    self.advance();
                }
                "/" => {
                    // Comment Handling is yet to be implemented
                    // Check for the next character and if it is / then handle comment.
                    self.tokens.push(Token::new(s, TokenType::Slash, self.line));
                    self.advance();
                }
                "*" => {
                    if self.check("*") {
                        self.tokens
                            .push(Token::new("**", TokenType::Power, self.line));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Star, self.line));
                    }
                    self.advance();
                }
                "%" => {
                    self.tokens
                        .push(Token::new(s, TokenType::Percent, self.line));
                    self.advance();
                }
                "|" => {
                    self.tokens.push(Token::new(s, TokenType::Pipe, self.line));
                    self.advance();
                }
                "^" => {
                    self.tokens.push(Token::new(s, TokenType::Caret, self.line));
                    self.advance();
                }
                "&" => {
                    self.tokens
                        .push(Token::new(s, TokenType::Ampersand, self.line));
                    self.advance();
                }
                "!" => {
                    self.tokens.push(Token::new(s, TokenType::Bang, self.line));
                    self.advance();
                }
                "?" => {
                    self.tokens
                        .push(Token::new(s, TokenType::Question, self.line));
                    self.advance();
                }
                "<" => {
                    if self.check("=") {
                        self.tokens
                            .push(Token::new("<=", TokenType::LessEqual, self.line));
                        self.advance();
                    } else if self.check("-") {
                        self.tokens
                            .push(Token::new("<-", TokenType::LeftAssign, self.line));
                        self.advance();
                    } else if self.check("<") {
                        self.tokens
                            .push(Token::new("<<", TokenType::LeftShift, self.line));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Less, self.line));
                    }
                    self.advance();
                }
                ">" => {
                    if self.check("=") {
                        self.tokens
                            .push(Token::new(">=", TokenType::GreaterEqual, self.line));
                        self.advance();
                    } else if self.check(">") {
                        self.tokens
                            .push(Token::new(">>", TokenType::RightShift, self.line));
                        self.advance();
                    } else {
                        self.tokens
                            .push(Token::new(s, TokenType::Greater, self.line));
                    }
                    self.advance();
                }
                "=" => {
                    if self.check("=") {
                        self.tokens
                            .push(Token::new("==", TokenType::EqualEqual, self.line));
                        self.advance();
                    } else if self.check(">") {
                        self.tokens
                            .push(Token::new("=>", TokenType::Arrow, self.line));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Equal, self.line));
                    }
                    self.advance();
                }
                "-" => {
                    if self.check(">") {
                        self.tokens
                            .push(Token::new("->", TokenType::RightAssign, self.line));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Minus, self.line));
                    }
                    self.advance();
                }
                s if alpha.contains(s) || s == "_" => {
                    self.identifier();
                }
                s if num.contains(s) => {
                    self.number();
                }
                "\"" => {
                    self.string();
                }
                "\'" => {
                    self.char();
                }
                " " | "\t" | "\r" => {
                    self.advance();
                }
                "\n" => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    panic!("Unexpected character: {}", s);
                }
            }
            // Uncomment for token printing
            // print!("{}[2J", 27 as char);
            // self.printTokens();
        }
        self.tokens.push(Token::new("", TokenType::Eof, self.line));
    }

    /// `advance` `fn` is used to move forward by 1.
    ///
    /// It will be used everywhere, whenever need to move forward.
    fn advance(&mut self) {
        self.current += 1;
    }

    /// Checks the next character of the file, after `self.current`.
    ///
    /// Used for the double charactered operators and so.
    fn check(&self, s: &str) -> bool {
        if self.ended() {
            return false;
        }
        let i = self.current + 1;
        let c = &self.source[i..i + 1];
        c == s
    }

    /// Checks for the end of the file.
    fn ended(&self) -> bool {
        self.current + 1 >= self.source.len()
    }

    /// `char` `fn` deals with the `char type` in the language.
    ///
    /// Simply panics if no character is specified at between `''` or uncomplete `char`.
    fn char(&mut self) {
        if self.ended() {
            panic!(
                "Unterminated character definition at {}, in line {}",
                self.current, self.line
            );
        }

        self.advance(); // Move past the initial '

        let mut ch = String::new();
        let current_char = self
            .source
            .chars()
            .nth(self.current)
            .expect("Unexpected program exit");

        if current_char == '\\' {
            self.advance();
            let escape_char = self
                .source
                .chars()
                .nth(self.current)
                .expect("Unexpected program exit");

            ch.push(match escape_char {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                _ => panic!(
                    "Unknown escape character at {}, in line {}",
                    self.current, self.line
                ),
            });
        } else {
            ch.push(current_char);
        }

        self.advance(); // Move past the character or escape sequence

        if self
            .source
            .chars()
            .nth(self.current)
            .expect("Unexpected program exit")
            != '\''
        {
            panic!(
                "Unterminated character definition at {}, in line {}",
                self.current, self.line
            );
        }

        self.advance(); // Move past the closing '

        self.tokens
            .push(Token::new(&ch, TokenType::CharLiteral, self.line));
    }

    /// `string` `fn` deals with the string literals in the language.
    ///
    /// If encounter `"` `string` is called.
    fn string(&mut self) {
        let mut s = String::new();
        self.advance(); // Move past the initial "

        while !self.ended() {
            let current_char = self
                .source
                .chars()
                .nth(self.current)
                .expect("Unexpected program exit");

            if current_char == '"' {
                break;
            }

            if current_char == '\\' {
                self.advance();
                let escape_char = self
                    .source
                    .chars()
                    .nth(self.current)
                    .expect("Unexpected program exit");

                s.push(match escape_char {
                    't' => '\t',
                    'n' => '\n',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    _ => panic!(
                        "Unknown escape sequence at {}, in line {}",
                        self.current, self.line
                    ),
                });
            } else {
                s.push(current_char);
            }

            self.advance();
        }

        if self.ended() {
            panic!(
                "Unterminated string definition at {}, in line {}",
                self.current, self.line
            );
        }

        self.advance(); // Move past the closing "

        self.tokens
            .push(Token::new(&s, TokenType::StringLiteral, self.line));
    }

    /// `number` `fn` deals with the number literals in the language.
    ///
    /// If encounter any number `number` is called.
    fn number(&mut self) {
        let mut n = String::new();
        while !self.ended() {
            let c = self
                .source
                .chars()
                .nth(self.current)
                .expect("Unexpected Program Exit");
            if c.is_numeric() || c == '.' {
                n.push(c);
            } else {
                break;
            }
            self.advance();
        }
        self.tokens
            .push(Token::new(&n, TokenType::NumberLiteral, self.line));
    }

    /// `indentifier` `fn` deals with the variables in the language.
    ///
    /// If encounter any alphabet or `_` `indentifier` is called.
    fn identifier(&mut self) {
        let num = HashSet::from(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        let alpha = HashSet::from([
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H",
            "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
            "Z",
        ]);
        let keywords = HashMap::from([
            ("void", TokenType::Void),
            ("main", TokenType::Main),
            ("let", TokenType::Let),
            ("const", TokenType::Const),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("while", TokenType::While),
            ("return", TokenType::Return),
            ("break", TokenType::Break),
            ("continue", TokenType::Continue),
            ("in", TokenType::In),
            ("match", TokenType::Match),
            ("struct", TokenType::Struct),
            ("enum", TokenType::Enum),
            ("impl", TokenType::Impl),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("goto", TokenType::Goto),
            ("label", TokenType::Label),
            ("mut", TokenType::Mut),
            ("pub", TokenType::Pub),
            ("and", TokenType::And),
            ("or", TokenType::Or),
            ("not", TokenType::Not),
            ("i8", TokenType::I8),
            ("i16", TokenType::I16),
            ("i32", TokenType::I32),
            ("i64", TokenType::I64),
            ("f32", TokenType::F32),
            ("f64", TokenType::F64),
            ("u8", TokenType::U8),
            ("u16", TokenType::U16),
            ("u32", TokenType::U32),
            ("u64", TokenType::U64),
            ("bool", TokenType::Bool),
            ("byte", TokenType::Byte),
            ("char", TokenType::Char),
            ("String", TokenType::String),
        ]);
        let mut id = String::new();
        while !self.ended() {
            let c = self
                .source
                .chars()
                .nth(self.current)
                .expect("Unexpected Program Exit");
            let ch = String::from(c);
            if num.contains(ch.as_str()) || alpha.contains(ch.as_str()) {
                id.push(c);
            } else {
                break;
            }
            self.advance();
        }

        if keywords.contains_key(id.as_str()) {
            let ttype = keywords[id.as_str()];
            self.tokens.push(Token::new(&id, ttype, self.line));
            return;
        } else {
            self.tokens
                .push(Token::new(&id, TokenType::Identifier, self.line));
        }
    }

    fn printTokens(&self) {
        for token in self.tokens.iter() {
            match token.token_type {
                TokenType::Identifier => {
                    println!(
                        "{}: {}",
                        token.token_type.to_string().green(),
                        token.lexeme.green().bold()
                    );
                }
                TokenType::NumberLiteral => {
                    println!(
                        "{}: {}",
                        token.token_type.to_string().yellow(),
                        token.lexeme.yellow().bold()
                    );
                }
                TokenType::StringLiteral | TokenType::CharLiteral => {
                    println!(
                        "{}: {}",
                        token.token_type.to_string().red(),
                        token.lexeme.red().bold()
                    );
                }
                TokenType::LeftBrace | TokenType::RightBrace => {
                    println!(
                        "{}: {}",
                        token.token_type.to_string().yellow(),
                        token.lexeme.yellow().bold()
                    );
                }
                TokenType::LeftBracket | TokenType::RightBracket => {
                    println!(
                        "{}: {}",
                        token.token_type.to_string().red(),
                        token.lexeme.red().bold()
                    );
                }
                TokenType::LeftParen | TokenType::RightParen => {
                    println!(
                        "{}: {}",
                        token.token_type.to_string().green(),
                        token.lexeme.green().bold()
                    );
                }
                _ => {
                    println!(
                        "{}: {}",
                        token.token_type.to_string().purple(),
                        token.lexeme.purple().bold()
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(lexeme: &str, token_type: TokenType, line: usize) -> Self {
        let lexeme = lexeme.to_string();

        Self {
            lexeme,
            token_type,
            line,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// `variableName`
    Identifier,

    /// Operators
    /// `=`
    Equal,
    /// `==`
    EqualEqual,
    /// `<`
    Less,
    /// `<=`
    LessEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,
    /// `=>`
    Arrow,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `!`
    Bang,
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `:`
    Colon,
    /// `?`
    Question,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `<-`
    LeftAssign,
    /// `->`
    RightAssign,
    /// `|`
    Pipe,
    /// **
    Power,
    /// `^`
    Caret,
    /// `&`
    Ampersand,
    /// `<<`
    LeftShift,
    /// `>>`
    RightShift,

    /// Literals
    /// `"string"`
    StringLiteral,
    /// `'c'`
    CharLiteral,
    /// `12345`
    NumberLiteral,

    /// Keywords
    /// `void`
    Void,
    /// `main`
    Main,
    /// `let`
    Let,
    /// `const`
    Const,
    /// `if`
    If,
    /// `else`
    Else,
    /// `while`
    While,
    /// `return`
    Return,
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `in`
    In,
    /// `match`
    Match,
    /// `struct`
    Struct,
    /// `enum`
    Enum,
    /// `impl`
    Impl,
    /// `true`
    True,
    /// `false`
    False,
    /// `goto`
    Goto,
    /// `label`
    Label,
    /// `mut`
    Mut,
    ///  `pub`
    Pub,
    /// `and`
    And,
    /// `or`
    Or,
    /// 'not'
    Not,

    /// Types
    /// `i8`
    I8,
    /// `i16`
    I16,
    /// `i32`
    I32,
    /// `i64`
    I64,
    /// `f32`
    F32,
    /// `f64`
    F64,
    /// `u8`
    U8,
    /// `u16`
    U16,
    /// `u32`
    U32,
    /// `u64`
    U64,
    /// `bool`
    Bool,
    /// `byte`
    Byte,
    /// `char`
    Char,
    /// String
    String,
    /// End of File
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Arrow => write!(f, "Arrow"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Power => write!(f, "Power"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Percent => write!(f, "Percent"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Question => write!(f, "Question"),
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::LeftBracket => write!(f, "LeftBracket"),
            TokenType::RightBracket => write!(f, "RightBracket"),
            TokenType::LeftAssign => write!(f, "LeftAssign"),
            TokenType::RightAssign => write!(f, "RightAssign"),
            TokenType::Pipe => write!(f, "Pipe"),
            TokenType::Caret => write!(f, "Caret"),
            TokenType::Ampersand => write!(f, "Ampersand"),
            TokenType::LeftShift => write!(f, "LeftShift"),
            TokenType::RightShift => write!(f, "RightShift"),
            TokenType::StringLiteral => write!(f, "StringLiteral"),
            TokenType::CharLiteral => write!(f, "CharLiteral"),
            TokenType::NumberLiteral => write!(f, "NumberLiteral"),
            TokenType::Void => write!(f, "Void"),
            TokenType::Main => write!(f, "Main"),
            TokenType::Let => write!(f, "Let"),
            TokenType::Const => write!(f, "Const"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::While => write!(f, "While"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Break => write!(f, "Break"),
            TokenType::Continue => write!(f, "Continue"),
            TokenType::In => write!(f, "In"),
            TokenType::Match => write!(f, "Match"),
            TokenType::Struct => write!(f, "Struct"),
            TokenType::Enum => write!(f, "Enum"),
            TokenType::Impl => write!(f, "Impl"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::Goto => write!(f, "Goto"),
            TokenType::Label => write!(f, "Label"),
            TokenType::Mut => write!(f, "Mut"),
            TokenType::Pub => write!(f, "Pub"),
            TokenType::And => write!(f, "And"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Not => write!(f, "Not"),
            TokenType::I8 => write!(f, "I8"),
            TokenType::I16 => write!(f, "I16"),
            TokenType::I32 => write!(f, "I32"),
            TokenType::I64 => write!(f, "I64"),
            TokenType::F32 => write!(f, "F32"),
            TokenType::F64 => write!(f, "F64"),
            TokenType::U8 => write!(f, "U8"),
            TokenType::U16 => write!(f, "U16"),
            TokenType::U32 => write!(f, "U32"),
            TokenType::U64 => write!(f, "U64"),
            TokenType::Bool => write!(f, "Bool"),
            TokenType::Byte => write!(f, "Byte"),
            TokenType::Char => write!(f, "Char"),
            TokenType::String => write!(f, "String"),
            TokenType::Eof => write!(f, "Eof"),
        }
    }
}

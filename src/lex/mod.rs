pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn parse(&mut self) {
        while self.current < self.source.len() {
            let s = &self.source[self.current..self.current + 1];
            match s {
                "." => {
                    self.tokens.push(Token::new(s, TokenType::Dot));
                    self.advance();
                }
                "," => {
                    self.tokens.push(Token::new(s, TokenType::Comma));
                    self.advance();
                }
                ";" => {
                    self.tokens.push(Token::new(s, TokenType::Semicolon));
                    self.advance();
                }
                ":" => {
                    self.tokens.push(Token::new(s, TokenType::Colon));
                    self.advance();
                }
                "(" => {
                    self.tokens.push(Token::new(s, TokenType::LeftParen));
                    self.advance();
                }
                ")" => {
                    self.tokens.push(Token::new(s, TokenType::RightParen));
                    self.advance();
                }
                "{" => {
                    self.tokens.push(Token::new(s, TokenType::LeftBrace));
                    self.advance();
                }
                "}" => {
                    self.tokens.push(Token::new(s, TokenType::RightBrace));
                    self.advance();
                }
                "[" => {
                    self.tokens.push(Token::new(s, TokenType::LeftBracket));
                    self.advance();
                }
                "]" => {
                    self.tokens.push(Token::new(s, TokenType::RightBracket));
                    self.advance();
                }
                "+" => {
                    self.tokens.push(Token::new(s, TokenType::Plus));
                    self.advance();
                }
                "/" => {
                    self.tokens.push(Token::new(s, TokenType::Slash));
                    self.advance();
                }
                "*" => {
                    self.tokens.push(Token::new(s, TokenType::Star));
                    self.advance();
                }
                "%" => {
                    self.tokens.push(Token::new(s, TokenType::Percent));
                    self.advance();
                }
                "|" => {
                    self.tokens.push(Token::new(s, TokenType::Pipe));
                    self.advance();
                }
                "^" => {
                    self.tokens.push(Token::new(s, TokenType::Caret));
                    self.advance();
                }
                "&" => {
                    self.tokens.push(Token::new(s, TokenType::Ampersand));
                    self.advance();
                }
                "!" => {
                    self.tokens.push(Token::new(s, TokenType::Bang));
                    self.advance();
                }
                "?" => {
                    self.tokens.push(Token::new(s, TokenType::Question));
                    self.advance();
                }
                "<" => {
                    if self.check("=") {
                        self.tokens.push(Token::new("<=", TokenType::LessEqual));
                        self.advance();
                    } else if self.check("-") {
                        self.tokens.push(Token::new("<-", TokenType::LeftAssign));
                        self.advance();
                    } else if self.check("<") {
                        self.tokens.push(Token::new("<<", TokenType::LeftShift));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Less));
                    }
                    self.advance();
                }
                ">" => {
                    if self.check("=") {
                        self.tokens.push(Token::new(">=", TokenType::GreaterEqual));
                        self.advance();
                    } else if self.check(">") {
                        self.tokens.push(Token::new(">>", TokenType::RightShift));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Greater));
                    }
                    self.advance();
                }
                "=" => {
                    if self.check("=") {
                        self.tokens.push(Token::new("==", TokenType::EqualEqual));
                        self.advance();
                    } else if self.check(">") {
                        self.tokens.push(Token::new("=>", TokenType::Arrow));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Equal));
                    }
                    self.advance();
                }
                "-" => {
                    if self.check(">") {
                        self.tokens.push(Token::new("->", TokenType::RightAssign));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(s, TokenType::Minus));
                    }
                    self.advance();
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
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn check(&self, s: &str) -> bool {
        if self.current + 1 >= self.source.len() {
            return false;
        }
        let i = self.current + 1;
        let c = &self.source[i..i + 1];
        c == s
    }

    fn char(&mut self) {
        if self.current + 1 >= self.source.len() {
            panic!("Unterminated character Definition at {}, in line {}", self.current, self.line);
        }
        let mut ch = String::new();
        if self.check("\\") {
            self.advance();
            ch.push('\\');
        } else {
            ch.push(
                self.source.chars()
                .nth(self.current)
                .expect("Unexpected end of the file")
            );
        }
        self.advance();
        if self.check("'") {
            self.advance();
            self.tokens.push(Token::new(&ch, TokenType::CharLiteral));
        } else {
            panic!("Unterminated character Definition at {}, in line {}", self.current, self.line);
        }
    }

}

pub struct Token {
    lexeme: String,
    token_type: TokenType,
}

impl Token {
    pub fn new(lexeme: &str, token_type: TokenType) -> Self {
        let lexeme = lexeme.to_string();
        Self { lexeme, token_type }
    }
}

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
}

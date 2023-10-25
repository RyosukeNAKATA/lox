pub enum TokenType {
    // single token
    LEFT_PAREN,
    RIGTH_PATEN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    FOT,
    MINUS,
    PLUS,
    SMICOLON,
    SLASH,
    STAR,
    // pairs token
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_THAN,
    LESS,
    LESS_EQUAL,
    // literal
    identifier,
    string,
    number,
    // keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

pub struct Token {
    pub ty: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
    pub col: i64,
}
struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    err: Option<Error>,
    start: usize,
    current: usize,
    line: usize,
    col: i64,
    keywords: HashMap<String, TokenType>,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            err: None,
            start: 0,
            current: 0,
            line: 1,
            col: -1,
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
                ("lambda", TokenType::Lambda),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
        }
    }
}

impl Scanner {
    fn scan_tokens(&mut self, input: String) {
        self.source = input.into_bytes();

        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }

        match self.err {
            Some(_) => {}
            None => self.tokens.push(Token {
                ty: TokenType::Eof,
                lexeme: Vec::new(),
                literal: None,
                line: self.line,
                col: self.col,
            }),
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.col = 0
            }
            '"' => self.string(),
            _ => {
                if Scanner::is_decimal_digit(c) {
                    self.number()
                } else if Scanner::is_alpha(c) {
                    self.identifier()
                } else {
                    self.err = Some(Error {
                        what: format!("scanner can't handle {}", c),
                        line: self.line,
                        col: self.col,
                    })
                }
            }
        }
    }
}

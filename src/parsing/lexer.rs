use super::preprocessor::preprocess;
use super::token::Token;

const digits: &str = "0123456789";
const alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
const alphabet_c: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Lexer {
    code: String,
    pos: usize,
    current: Option<char>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code: code.clone(),
            pos: 0,
            current: code.chars().nth(0),
        }
    }

    pub fn next(&mut self) {
        self.pos += 1;
        self.current = if self.pos < self.code.len() {
            self.code.chars().nth(self.pos)
        } else {
            None
        };
    }

    pub fn tokenize(&mut self) {
        let mut tokens: Vec<Token> = vec![];
        self.code = preprocess(self.code.clone());
        
        loop {
            let token: Option<Token> = match self.current {
                // SKIP OR BREAK
                Some(' ') => None,
                Some('\n') => None,
                None => break,

                // OPERATORS
                Some('+') => Some(Token::ADD),
                Some('-') => Some(Token::SUBTRACT),
                Some('*') => Some(Token::MULTIPLY),
                Some('/') => Some(Token::DIVIDE),
                Some('(') => Some(Token::LBRACK),
                Some(')') => Some(Token::RBRACK),
                
                // REST
                Some(c) => {
                    if digits.contains(c) {
                        Some(self.make_nr_token())
                    } else {
                        None
                    }
                },
            };

            match token {
                Some(token) => {
                    tokens.push(token);
                    self.next();
                },
                None => self.next(),
            }
        }

        println!("{:?}", tokens);
    }

    fn make_nr_token(&mut self) -> Token {
        let mut nr: String = String::new();
        let mut dot_amount: u8 = 0;

        while (self.current != None) && (digits.contains(self.current.unwrap()) || self.current.unwrap() == '.') {
            if self.current.unwrap() == '.' {
                if dot_amount == 1 {
                    panic!("Unexpected additional '.' in Number.");
                }
                dot_amount += 1;
            }
            nr.push(self.current.unwrap());
            self.next();
        }

        if dot_amount == 1 {
            return Token::FLOAT(nr.parse::<f32>().unwrap());
        } else {
            return Token::INT(nr.parse::<i32>().unwrap());
        }
    }
}
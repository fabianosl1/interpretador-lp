use core::panic;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Variable(String),
    Not,
    And,
    Or,
    Implies,
    Iff,
    LParen,
    RParen,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        if self.position >= self.input.len() {
            return Ok(Token::EOF);
        }

        self.skip_whitespace();

        let ch = self.input[self.position];
        let token: Token;

        match ch {
            '~' => token = Token::Not,
            '&' => token = Token::And,
            '|' => token = Token::Or,
            '(' => token = Token::LParen,
            ')' => token = Token::RParen,
            '<' => token = self.iff(),
            '-' => token = self.implies(),
            'p' => token = self.variable(),
            _ => {
                let message = format!("Caractere inesperado '{}' na posição {}", ch, self.position);
                return Err(message);
            }
        }

        self.position += 1;
        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn iff(&mut self) -> Token {
        if self.position + 2 >= self.input.len() {
            panic!("Bi-implicação mal formada na posição {}", self.position);
        }

        if self.input[self.position + 1] != '-' || self.input[self.position + 2] != '>' {
            panic!("Bi-implicação mal formada na posição {}", self.position);
        }

        self.position += 2;
        Token::Iff
    }

    fn implies(&mut self) -> Token {
        self.position += 1;

        if self.position >= self.input.len() || self.input[self.position] != '>' {
            panic!("Implicação mal formada na posição {}", self.position);
        }

        Token::Implies
    }

    fn variable(&mut self) -> Token {
        let start = self.position;
        self.position += 1;

        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            self.position += 1;
        }

        let variable: String = self.input[start..self.position].iter().collect();

        Token::Variable(variable)
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn implies() {
        let expect = vec![
            Token::Variable("p1".to_string()),
            Token::Implies,
            Token::Variable("p3".to_string()),
            Token::EOF,
        ];

        let mut lexer = Lexer::new("p1p -> p3");

        let result = fetch_all_tokens(&mut lexer);

        assert_eq!(result, expect);
    }

    #[test]
    fn legal_token() {}
    fn fetch_all_tokens(lexer: &mut Lexer) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Ok(token) = lexer.get_next_token() {
            tokens.push(token.clone());
            
            if token == Token::EOF {
                break;
            }
        }

        tokens
    }
}

use regex::Regex;

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
            '~' => {
                self.position += 1;
                token = Token::Not;
            }
            '&' => {
                self.position += 1;
                token = Token::And;
            }
            '|' => {
                self.position += 1;
                token = Token::Or;
            }
            '(' => {
                self.position += 1;
                token = Token::LParen;
            }
            ')' => {
                self.position += 1;
                token = Token::RParen;
            }
            '<' => token = self.iff()?,
            '-' => token = self.implies()?,
            _ => {
                if ch.is_ascii_alphanumeric() {
                    token = self.variable()?
                } else {
                    let message =
                        format!("Caractere inesperado '{}' na posição {}", ch, self.position);
                    return Err(message);
                }
            }
        }

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn iff(&mut self) -> Result<Token, String> {
        let symbol = "<->";
        self.check_symbol(symbol)?;

        self.position += symbol.len();
        Ok(Token::Iff)
    }

    fn implies(&mut self) -> Result<Token, String> {
        let symbol = "->";
        self.check_symbol(symbol)?;

        self.position += symbol.len();
        Ok(Token::Implies)
    }

    fn check_symbol(&mut self, symbol: &str) -> Result<(), String> {
        let start = self.position;
        let mut end = start + symbol.len();

        if end >= self.input.len() {
            end = self.input.len();
        }

        let word: String = self.input[start..end].iter().collect();

        if word != symbol {
            return Err(format!("'{}' não faz parte da linguagem", word));
        }

        Ok(())
    }

    fn variable(&mut self) -> Result<Token, String> {
        let start = self.position;

        while self.position < self.input.len()
            && (self.input[self.position].is_numeric() || self.input[self.position].is_alphabetic())
        {
            self.position += 1;
        }

        let variable: String = self.input[start..self.position].iter().collect();

        let re = Regex::new(r"^p\d+$").unwrap();

        if re.is_match(&variable) {
            return Ok(Token::Variable(variable));
        }

        Err(format!("'{}' não é uma letra proposicional", variable))
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn when_valid_innput_then_ordened_tokens() {
        let tokens = vec![
            Token::Variable("p1".to_string()),
            Token::Implies,
            Token::Variable("p3".to_string()),
            Token::EOF,
        ];

        let mut lexer = Lexer::new("p1 -> p3");

        tokens
            .iter()
            .for_each(|expected| match lexer.get_next_token() {
                Ok(result) => assert_eq!(*expected, result),
                Err(message) => panic!("{}", message),
            });
    }

    #[test]
    fn when_ilegal_variable_then_except() {
        let mut lexer = Lexer::new("p1p p P");

        assert!(matches!(lexer.get_next_token(), Err(_)));
        assert!(matches!(lexer.get_next_token(), Err(_)));
        assert!(matches!(lexer.get_next_token(), Err(_)));
    }

    #[test]
    fn when_invalid_token_then_except() {
        let mut lexer = Lexer::new("p1 * p2");

        assert!(matches!(lexer.get_next_token(), Ok(_)));
        assert!(matches!(lexer.get_next_token(), Err(_)));
    }
}

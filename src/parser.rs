use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Variable(String),
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Implies(Box<Expression>, Box<Expression>),
    Iff(Box<Expression>, Box<Expression>),
    Grouped(Box<Expression>),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        match lexer.get_next_token() {
            Ok(current_token) => Parser {
                lexer,
                current_token,
            },
            Err(message) => panic!("{}", message),
        }
    }

    fn next_token(&mut self) {
        match self.lexer.get_next_token() {
            Ok(token) => self.current_token = token,
            Err(message) => panic!("{}", message),
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        let result = self.parse_expression()?;

        if self.current_token != Token::EOF {
            return Err(format!("não consumiu todos os tokens"));
        }

        Ok(result)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        Ok(self.parse_iff()?)
    }

    fn parse_iff(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_implies()?;

        while self.current_token == Token::Iff {
            self.next_token();
            let right = self.parse_implies()?;
            left = Expression::Iff(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_implies(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_or()?;

        while self.current_token == Token::Implies {
            self.next_token();
            let right = self.parse_or()?;
            left = Expression::Implies(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_and()?;

        while self.current_token == Token::Or {
            self.next_token();
            let right = self.parse_and()?;
            left = Expression::Or(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_not()?;

        while self.current_token == Token::And {
            self.next_token();
            let right = self.parse_not()?;
            left = Expression::And(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expression, String> {
        if self.current_token == Token::Not {
            self.next_token();
            let expression = self.parse_not()?;
            return Ok(Expression::Not(Box::new(expression)));
        }

        Ok(self.parse_primary()?)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match &self.current_token {
            Token::LParen => {
                self.next_token();
                let expression = self.parse_expression()?;

                if self.current_token != Token::RParen {
                    return Err(format!(
                        "Esperado ')', mas encontrou {:?}",
                        self.current_token
                    ));
                }

                self.next_token();
                Ok(Expression::Grouped(Box::new(expression)))
            }
            Token::Variable(variable) => {
                let expression = Expression::Variable(variable.clone());
                self.next_token();
                Ok(expression)
            }
            _ => Err(format!("token inesperado {:?}", self.current_token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::{Expression, Parser};

    #[test]
    fn invalid_expression() {
        let result = arrange("p1 -> ()");
        assert!(matches!(result, Err(_)))
    }

    #[test]
    fn valid_expression() {
        let result = arrange("p1 -> p2");
        assert!(matches!(result, Ok(_)))
    }

    #[test]
    fn correct_ast() {
        let result = arrange("(p1 | p2) -> p3").unwrap();

        let expected = Expression::Implies(
            Box::new(Expression::Grouped(Box::new(Expression::Or(
                Box::new(Expression::Variable("p1".to_string())),
                Box::new(Expression::Variable("p2".to_string())),
            )))),
            Box::new(Expression::Variable("p3".to_string())),
        );

        assert_eq!(result, expected)
    }

    #[test]
    fn ast_grouped() {
        let result = arrange("p1 | (p2 -> p3)").unwrap();

        let expected = Expression::Or(
            Box::new(Expression::Variable("p1".to_string())),
            Box::new(Expression::Grouped(Box::new(Expression::Implies(
                Box::new(Expression::Variable("p2".to_string())),
                Box::new(Expression::Variable("p3".to_string())),
            )))),
        );

        assert_eq!(result, expected)
    }

    fn arrange(input: &str) -> Result<Expression, String> {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        parser.parse()
    }
}

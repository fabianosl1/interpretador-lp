use std::collections::HashSet;

use crate::{lexer::{Lexer, Token}, Expression};


pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    variables: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Result<Self, String> {
        let current_token = lexer.get_next_token()?;

        Ok(Parser {
            lexer,
            current_token,
            variables: HashSet::new(),
        })
    }

    fn next_token(&mut self) -> Result<(), String> {
        self.current_token = self.lexer.get_next_token()?;
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        let result = self.parse_expression()?;

        if self.current_token != Token::EOF {
            return Err(format!("não consumiu todos os tokens"));
        }

        Ok(result)
    }

    pub fn get_variables(&mut self) -> Vec<String> {
        self.variables.iter().cloned().collect()
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        Ok(self.parse_iff()?)
    }

    fn parse_iff(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_implies()?;

        while self.current_token == Token::Iff {
            self.next_token()?;
            let right = self.parse_implies()?;
            left = Expression::Iff(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_implies(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_or()?;

        while self.current_token == Token::Implies {
            self.next_token()?;
            let right = self.parse_or()?;
            left = Expression::Implies(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_and()?;

        while self.current_token == Token::Or {
            self.next_token()?;
            let right = self.parse_and()?;
            left = Expression::Or(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_not()?;

        while self.current_token == Token::And {
            self.next_token()?;
            let right = self.parse_not()?;
            left = Expression::And(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expression, String> {
        if self.current_token == Token::Not {
            self.next_token()?;
            let expression = self.parse_not()?;
            return Ok(Expression::Not(Box::new(expression)));
        }

        Ok(self.parse_primary()?)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match &self.current_token {
            Token::LParen => {
                self.next_token()?;
                let expression = self.parse_expression()?;

                if self.current_token != Token::RParen {
                    return Err(format!(
                        "Esperado ')', mas encontrou {:?}",
                        self.current_token
                    ));
                }

                self.next_token()?;
                Ok(Expression::Grouped(Box::new(expression)))
            }
            Token::Variable(variable) => {
                let expression = Expression::Variable(variable.clone());

                self.variables.insert(variable.clone());
                
                self.next_token()?;

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
    fn when_input_valid_then_ok() {
        let result = arrange("p1 -> p2");
        assert!(matches!(result, Ok(_)))
    }

    #[test]
    fn when_valid_token_then_ok() {
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
    fn when_has_grouped_then_ast_with_grouped() {
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

    #[test]
    fn when_get_variables_then_ok() {
        let mut lexer = Lexer::new("p1 | (p2 -> p3)");
        let mut parser = Parser::new(&mut lexer).unwrap();
        let _ = parser.parse();

        let expected = vec!["p1".to_string(), "p2".to_string(), "p3".to_string()];

        let mut result = parser.get_variables();

        result.sort();

        assert_eq!(result, expected)
    }

    fn arrange(input: &str) -> Result<Expression, String> {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer).unwrap();

        parser.parse()
    }
}

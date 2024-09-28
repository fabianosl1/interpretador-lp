use std::collections::HashMap;

use evaluation::eval;
use lexer::Lexer;
use parser::Parser;
use serde::Serialize;
use table::generate_table;

mod evaluation;
mod lexer;
mod parser;
mod table;

#[derive(Serialize)]
pub enum Type {
    Tautology,
    Contradiction,
    Contigent,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub enum Expression {
    Variable(String),
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Implies(Box<Expression>, Box<Expression>),
    Iff(Box<Expression>, Box<Expression>),
    Grouped(Box<Expression>),
}

pub fn get_table(variables: &Vec<String>) -> Vec<HashMap<String, bool>> {
    generate_table(&variables)
}

pub fn get_type(
    expression: &Expression,
    table: &Vec<HashMap<String, bool>>,
) -> Result<Type, String> {
    let mut count_true = 0;

    for row in table {
        let result = eval(expression, &row)?;

        if result == true {
            count_true += 1
        }
    }

    match count_true {
        0 => Ok(Type::Contradiction),
        _ => {
            if count_true == table.len() {
                return Ok(Type::Tautology);
            }

            Ok(Type::Contigent)
        }
    }
}

pub fn parser(input: &str) -> Result<(Expression, Vec<String>), String> {
    let mut lexer = Lexer::new(&input);
    let mut parser = Parser::new(&mut lexer)?;

    let expression = parser.parse()?;
    let variables = parser.get_variables();

    Ok((expression, variables))
}

pub fn evaluation_expression(
    expression: &Expression,
    variables: &HashMap<String, bool>,
) -> Result<bool, String> {
    eval(expression, variables)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn when_contradition_input_then_ok() {
        let (expression, variables) = parser("p1 & ~p1").unwrap();
        let table = get_table(&variables);

        let result = get_type(&expression, &table).unwrap();

        assert!(matches!(result, Type::Contradiction))
    }
    
    #[test]
    fn when_tautology_input_then_ok() {
        let (expression, variables) = parser("(p1 & p2) -> (p1 | p2)").unwrap();
        let table = get_table(&variables);
        
        let result = get_type(&expression, &table).unwrap();

        assert!(matches!(result, Type::Tautology))
    }

    #[test]
    fn when_contigent_input_then_ok() {
        let (expression, variables) = parser("(p1 | p2) & p3").unwrap();
        let table = get_table(&variables);

        let result = get_type(&expression, &table).unwrap();

        assert!(matches!(result, Type::Contigent))
    }
}

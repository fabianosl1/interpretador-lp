use std::collections::HashMap;

use crate::Expression;


pub fn eval(expression: &Expression, variables: &HashMap<String, bool>) -> Result<bool, String> {
    match expression {
        Expression::Variable(name) => eval_variable(name.clone(), variables),
        Expression::Not(inner) => eval_not(inner, variables),
        Expression::And(left, right) => eval_and(left, right, variables),
        Expression::Or(left, right) => eval_or(left, &right, variables),
        Expression::Implies(left, right) => eval_implies(left, right, variables),
        Expression::Iff(left, right) => eval_iff(left, right, variables),
        Expression::Grouped(inner) => eval_grouped(inner, variables),
    }
}

pub fn eval_variable(name: String, variables: &HashMap<String, bool>) -> Result<bool, String> {
    variables
        .get(&name)
        .copied()
        .ok_or(format!("valor para '{}' não definido", name))
}

pub fn eval_not(inner: &Expression, variables: &HashMap<String, bool>) -> Result<bool, String> {
    Ok(!eval(inner, variables)?)
}

pub fn eval_and(
    left: &Expression,
    right: &Expression,
    variables: &HashMap<String, bool>,
) -> Result<bool, String> {
    Ok(eval(left, variables)? && eval(right, variables)?)
}

pub fn eval_or(
    left: &Expression,
    right: &Expression,
    variables: &HashMap<String, bool>,
) -> Result<bool, String> {
    Ok(eval(left, variables)? || eval(right, variables)?)
}

pub fn eval_implies(
    left: &Expression,
    right: &Expression,
    variables: &HashMap<String, bool>,
) -> Result<bool, String> {
    let value_left = !eval(left, variables)?;
    Ok(value_left || (value_left && eval(right, variables)?))
}

pub fn eval_iff(
    left: &Expression,
    right: &Expression,
    variables: &HashMap<String, bool>,
) -> Result<bool, String> {
    Ok(eval(left, variables)? == eval(right, variables)?)
}

pub fn eval_grouped(inner: &Expression, variables: &HashMap<String, bool>) -> Result<bool, String> {
    Ok(eval(inner, variables)?)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use crate::{evaluation::eval, lexer::Lexer, parser::Parser, Expression};


    #[test]
    fn when_valid_inputs_then_ok() {
        let inputs = vec!["p1 & p2", "p1 | p2", "~(p1 & p2)"];

        let variables = arrange_variables();

        inputs.iter().for_each(|input| {
            let expression = arrange_expression(&input);
            let result = eval(&expression, &variables);

            assert!(matches!(result, Ok(_)));
        })
    }

    #[test]
    fn when_should_be_true_then_true() {
        let expression = arrange_expression("~(p1 & p2)");
        let mut variables = arrange_variables();

        let mut result = eval(&expression, &variables).unwrap();
        assert_eq!(result, false);

        variables.insert("p2".to_string(), false);

        result = eval(&expression, &variables).unwrap();
        assert_eq!(result, true);
    }

    fn arrange_expression(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        parser.parse().unwrap()
    }

    fn arrange_variables() -> HashMap<String, bool> {
        let mut variables: HashMap<String, bool> = HashMap::new();

        variables.insert("p1".to_string(), true);
        variables.insert("p2".to_string(), true);

        variables
    }
}

use std::collections::HashMap;

pub fn generate_table(variables: &Vec<String>) -> Vec<HashMap<String, bool>> {
    let size = 2_i32.pow(variables.len() as u32);
    let mut table = Vec::new();

    for i in 0..size {
        let mut row = HashMap::new();

        for (position, name) in variables.iter().enumerate() {
            let value = (i >> position) & 1 == 1;
            row.insert(name.clone(), value);
        }

        table.push(row);
    }

    table
}

#[cfg(test)]
mod tests {
    use super::generate_table;

    #[test]
    fn when_size_3_then_3() {
        let input = vec!["p1".to_string(), "p2".to_string(), "p3".to_string()];
        let expect = 2_i32.pow(input.len() as u32);

        let result = generate_table(&input);

        assert_eq!(result.len() as i32, expect);
    }
}

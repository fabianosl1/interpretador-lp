// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use ilp;

#[tauri::command]
fn get_type(input: &str, table: Vec<HashMap<String, bool>>) -> Result<ilp::Type, String> {
    let (expression, _) = ilp::parser(input)?;
    Ok(ilp::get_type(&expression, &table)?)
}

#[tauri::command]
fn get_table(input: &str) -> Result<Vec<HashMap<String, bool>>, String> {
    let (expression, variables_names) = ilp::parser(input)?;
    let mut table = ilp::get_table(&variables_names);

    for variables in table.iter_mut() {
        let result = ilp::evaluation_expression(&expression, &variables)?;
        variables.insert(String::from("result"), result);
    }

    Ok(table)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_type, get_table])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

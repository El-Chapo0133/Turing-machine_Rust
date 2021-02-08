
use serde_json::{Result};
use serde::{Deserialize, Serialize};
use std::fs;


#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Output {
    pub executable: Vec<ExecutableStep>,
    pub base_tape: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutableStep {
    pub name: String,
    pub todo: Vec<Step>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    pub read: String,
    pub write: String,
    pub r#move: u8, // need the "r#", but will be identified as "move"
    pub goto: String
}

impl Output {
    pub fn new() -> Output {
        Output {
            executable: Vec::new(),
            base_tape: String::from(""),
        }
    }
}
// impl Step {
//     pub fn clone_step(input: &Step) -> Step {
//         Step {
//             read: input.read.clone(),
//             write: input.write.clone(),
//             r#move: input.r#move.clone(),
//             goto: input.goto.clone(),
//         }
//     }
// }

pub fn get_executable() -> Result<Output> {
    let executable_file = read_executable().expect("Error reading the executable file");

    serde_json::from_str(&executable_file)
}


fn read_executable() -> std::io::Result<String> {
    let filename = String::from("./_resources/executable.json");
    fs::read_to_string(filename)
}